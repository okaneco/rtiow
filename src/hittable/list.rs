use crate::hittable::{HitRecord, Hittable};

/// Trait for attaching to objects that can be detected by rays.
#[derive(Clone, Debug, Default)]
pub struct HittableList<H: Hittable> {
    /// List of `Hittable` objects.
    pub objects: Vec<H>,
}

impl<H: Hittable> HittableList<H> {
    /// Create a new `HittableList`.
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    /// Create a new `HittableList` constructed from an initial object.
    pub fn new_from(object: H) -> Self {
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
    pub fn add(&mut self, object: H) {
        self.objects.push(object);
    }

    /// Clear the `HittableList`.
    pub fn clear(&mut self) {
        self.objects.clear()
    }
}

impl<H: Hittable> Hittable for HittableList<H> {
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for o in self.objects.iter() {
            if o.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }

        hit_anything
    }
}
