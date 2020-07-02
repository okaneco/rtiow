use std::sync::Arc;

use crate::hittable::{HitRecord, Hittable};

/// Trait for attaching to objects that can be detected by rays.
#[derive(Clone, Default)]
pub struct HittableList {
    /// List of `Hittable` objects.
    pub objects: Vec<Arc<dyn Hittable + Send + Sync>>,
}

impl HittableList {
    /// Create a new `HittableList`.
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    /// Create a new `HittableList` constructed from an initial object.
    pub fn new_from(object: Arc<dyn Hittable + Send + Sync>) -> Self {
        Self {
            objects: core::iter::once(object).collect(),
        }
    }

    /// Create a new `HittableList` with capacity `n`.
    pub fn with_capacity(n: usize) -> Self {
        Self {
            objects: Vec::with_capacity(n),
        }
    }

    /// Add an object to the `HittableList`.
    pub fn add(&mut self, object: Arc<dyn Hittable + Send + Sync>) {
        self.objects.push(object);
    }

    /// Clear the `HittableList`.
    pub fn clear(&mut self) {
        self.objects.clear()
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for o in self.objects.iter() {
            if o.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }

    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut crate::aabb::Aabb) -> bool {
        if self.objects.is_empty() {
            return false;
        }

        let mut temp_box = crate::aabb::Aabb::default();
        let mut first_box = true;

        for object in self.objects.iter() {
            if !(object.bounding_box(t0, t1, &mut temp_box)) {
                return false;
            }
            *output_box = if first_box {
                temp_box
            } else {
                crate::aabb::Aabb::surrounding_box(output_box, &temp_box)
            };
            first_box = false;
        }

        true
    }
}
