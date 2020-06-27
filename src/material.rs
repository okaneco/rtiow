//! Materials that can be struck by a ray and how they affect light.

use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};

/// Type of material.
#[derive(Copy, Clone, Debug)]
pub enum Material {
    /// Diffuse material.
    Lambertian(Lambert),
    /// Metallic material.
    Metallic(Metal),
    /// Dielectric material.
    Dielectric(Diel),
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
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        match self {
            Material::Lambertian(mat) => {
                let scatter_dir = rec.normal + Vec3::random_unit_vector(rng);
                *scattered = Ray::new(rec.p, scatter_dir);
                *attenuation = mat.albedo;
                true
            }
            Material::Metallic(mat) => {
                let reflected = Vec3::reflect(&r_in.direction().unit_vector(), &rec.normal);
                *scattered = Ray::new(rec.p, reflected);
                *attenuation = mat.albedo;
                scattered.direction().dot(&rec.normal) > 0.0
            }
            Material::Dielectric(ri) => {
                *attenuation = Color::new_with(1.0);
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
                    *scattered = Ray::new(rec.p, reflected);
                } else {
                    let refracted = Vec3::refract(&unit_dir, &rec.normal, etai_over_etat);
                    *scattered = Ray::new(rec.p, refracted);
                }
                true
            }
        }
    }
}

/// Diffuse material.
#[derive(Copy, Clone, Debug, Default)]
pub struct Lambert {
    /// Base color of the material.
    pub albedo: Color,
}

impl Lambert {
    /// Create a new `Lambert` material.
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

/// Metallic material.
#[derive(Copy, Clone, Debug, Default)]
pub struct Metal {
    /// Base color of the material.
    pub albedo: Color,
}

impl Metal {
    /// Create a new `Metal` material.
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

/// Dielectric material for simulating clear objects like water and glass.
#[derive(Copy, Clone, Debug, Default)]
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

/// Schlick approximation for reflectivity.
#[inline]
pub fn schlick(cos: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) * (1.0 + ref_idx).recip();
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cos).powi(5)
}
