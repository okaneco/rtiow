//! Materials that can be struck by a ray and how they affect light.

use std::sync::Arc;

use crate::hittable::HitRecord;
use crate::onb::Onb;
use crate::ray::Ray;
use crate::texture::{SolidColor, Texture};
use crate::vec3::{Color, Vec3};

/// Type of material.
#[derive(Clone)]
pub enum Material {
    /// Diffuse material.
    Lambertian(Lambert),
    /// Metallic material.
    Metallic(Metal),
    /// Dielectric material.
    Dielectric(Diel),
    /// Diffuse light material.
    DiffLight(DiffuseLight),
    /// Isotropic material.
    Iso(Isotropic),
}

impl core::default::Default for Material {
    fn default() -> Self {
        Material::Lambertian(Lambert::default())
    }
}

impl Material {
    /// Scattering function for how the material affects light.
    pub fn scatter<R: rand::Rng>(
        &self,
        rng: &mut R,
        r_in: &Ray,
        rec: &HitRecord,
        albedo: &mut Color,
        scattered: &mut Ray,
        pdf: &mut f64,
    ) -> bool {
        match self {
            Material::Lambertian(mat) => {
                let uvw = Onb::build_from_w(&rec.normal);
                let direction = uvw.local(&Vec3::random_cosine_direction(rng));
                *scattered = Ray::new(rec.p, direction.unit_vector(), r_in.time());
                *albedo = mat.albedo.value(rec.u, rec.v, &rec.p);
                *pdf = uvw.w().dot(&scattered.direction()) * core::f64::consts::FRAC_1_PI;
                true
            }
            Material::Metallic(mat) => {
                let reflected = Vec3::reflect(&r_in.direction().unit_vector(), &rec.normal);
                *scattered = Ray::new(
                    rec.p,
                    reflected + mat.fuzz * Vec3::random_in_unit_sphere(rng),
                    r_in.time(),
                );
                *albedo = mat.albedo;
                scattered.direction().dot(&rec.normal) > 0.0
            }
            Material::Dielectric(ri) => {
                *albedo = Color::new_with(1.0);
                let etai_over_etat = if rec.front_face {
                    1.0 * ri.refraction_index.recip()
                } else {
                    ri.refraction_index
                };

                let unit_dir = r_in.direction().unit_vector();
                let cos_theta = (-unit_dir).dot(&rec.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                if etai_over_etat * sin_theta > 1.0
                    || rng.gen::<f64>() < schlick(cos_theta, etai_over_etat)
                {
                    let reflected = Vec3::reflect(&unit_dir, &rec.normal);
                    *scattered = Ray::new(rec.p, reflected, r_in.time());
                } else {
                    let refracted = Vec3::refract(&unit_dir, &rec.normal, etai_over_etat);
                    *scattered = Ray::new(rec.p, refracted, r_in.time());
                }
                true
            }
            Material::Iso(mat) => {
                *scattered = Ray::new(rec.p, Vec3::random_in_unit_sphere(rng), r_in.time());
                *albedo = mat.albedo.value(rec.u, rec.v, &rec.p);
                true
            }
            Material::DiffLight(_) => false,
        }
    }

    /// Scattering probability distribution function for importance sampling.
    pub fn scattering_pdf<R: rand::Rng>(
        &self,
        _rng: &mut R,
        _r_in: &Ray,
        rec: &HitRecord,
        scattered: &Ray,
    ) -> f64 {
        match self {
            Material::Lambertian(_mat) => {
                let cosine = rec.normal.dot(&scattered.direction().unit_vector());
                if cosine < 0.0 {
                    0.0
                } else {
                    cosine * core::f64::consts::FRAC_1_PI
                }
            }
            Material::Metallic(_mat) => todo!(),
            Material::Dielectric(_ri) => todo!(),
            Material::Iso(_mat) => todo!(),
            Material::DiffLight(_) => todo!(),
        }
    }

    /// Color emitted by the material.
    pub fn emitted(&self, _r_in: &Ray, rec: &HitRecord) -> Color {
        match self {
            Material::DiffLight(diff) => {
                if rec.front_face {
                    diff.emit.value(rec.u, rec.v, &rec.p)
                } else {
                    Color::new_with(0.0)
                }
            }
            _ => Color::new_with(0.0),
        }
    }
}

/// Diffuse material.
#[derive(Clone)]
pub struct Lambert {
    /// Base color of the material.
    pub albedo: Arc<dyn Texture + Send + Sync>,
}

impl Lambert {
    /// Create a new `Lambert` material.
    pub fn new(color: Arc<dyn Texture + Send + Sync>) -> Self {
        Self { albedo: color }
    }
}

impl core::default::Default for Lambert {
    fn default() -> Self {
        Self {
            albedo: Arc::new(SolidColor::new(0.2, 0.6, 0.8)),
        }
    }
}

/// Metallic material.
#[derive(Clone, Copy, Debug, Default)]
pub struct Metal {
    /// Base color of the material.
    pub albedo: Color,
    /// Fuzz factor of the reflection.
    pub fuzz: f64,
}

impl Metal {
    /// Create a new `Metal` material.
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

/// Dielectric material for simulating clear objects like water and glass.
#[derive(Clone, Copy, Debug, Default)]
pub struct Diel {
    /// Refraction index of the dielectric.
    ///
    /// Air is typically 1.0, glass 1.3-1.7, and diamond is 2.4.
    pub refraction_index: f64,
}

impl Diel {
    /// Create a new `Diel` material.
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

#[derive(Clone)]
/// Diffuse emitting light.
pub struct DiffuseLight {
    /// Diffuse emitting texture.
    pub emit: Arc<dyn Texture + Send + Sync>,
}

impl DiffuseLight {
    /// Create a new diffuse light.
    pub fn new(emit: Arc<dyn Texture + Send + Sync>) -> Self {
        Self { emit }
    }
}

/// Isotropic scattering material.
#[derive(Clone)]
pub struct Isotropic {
    /// Based texture of the material.
    pub albedo: Arc<dyn Texture + Send + Sync>,
}

impl Isotropic {
    /// Create new isotropic material.
    pub fn new(albedo: Arc<dyn Texture + Send + Sync>) -> Self {
        Self { albedo }
    }
}
/// Schlick approximation for reflectivity.
#[inline]
pub fn schlick(cos: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) * (1.0 + ref_idx).recip();
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cos).powi(5)
}
