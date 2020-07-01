use std::sync::Arc;

use rand::Rng;

use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;

/// Fog volume.
#[derive(Clone)]
pub struct ConstantMedium {
    /// Boundary of volume.
    pub boundary: Arc<dyn Hittable + Send + Sync>,
    /// Material of volume.
    pub phase_function: Arc<Material>,
    /// Negative inverse density of volume.
    pub neg_inv_density: f64,
}

impl ConstantMedium {
    /// Create a new constant medium.
    pub fn new(
        boundary: Arc<dyn Hittable + Send + Sync>,
        phase_function: Arc<dyn crate::texture::Texture + Send + Sync>,
        neg_inv_density: f64,
    ) -> Self {
        Self {
            boundary,
            phase_function: Arc::new(Material::Iso(crate::material::Isotropic::new(
                phase_function,
            ))),
            neg_inv_density: -1.0 / neg_inv_density,
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64, rec: &mut super::HitRecord) -> bool {
        let mut rec1 = HitRecord::default();
        let mut rec2 = HitRecord::default();

        if !self
            .boundary
            .hit(r, f64::NEG_INFINITY, f64::INFINITY, &mut rec1)
        {
            return false;
        }

        if !self
            .boundary
            .hit(r, rec1.t + 0.0001, f64::INFINITY, &mut rec2)
        {
            return false;
        }

        if rec1.t < t_min {
            rec1.t = t_min;
        }

        if rec2.t > t_max {
            rec2.t = t_max;
        }

        if rec1.t >= rec2.t {
            return false;
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = r.direction().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * rand::thread_rng().gen::<f64>().ln();

        if hit_distance > distance_inside_boundary {
            return false;
        }

        rec.t = rec1.t + hit_distance * ray_length.recip();
        rec.p = r.at(rec.t);

        rec.normal = crate::vec3::Vec3::new(1.0, 0.0, 0.0);
        rec.front_face = true;
        rec.material = self.phase_function.clone();

        true
    }

    fn bounding_box(
        &self,
        t0: std::primitive::f64,
        t1: std::primitive::f64,
        output_box: &mut crate::aabb::Aabb,
    ) -> std::primitive::bool {
        self.boundary.bounding_box(t0, t1, output_box)
    }
}
