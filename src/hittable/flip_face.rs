use crate::hittable::Hittable;

/// Helper material to specify how materials should behave whether rays are
/// coming from inside or outside the object. This material allows us to swap
/// the normal of the material's face.
pub struct FlipFace {
    /// Pointer to the material.
    pub pointer: std::sync::Arc<dyn Hittable + Send + Sync>,
}

impl FlipFace {
    /// Create a new material which allows for flipping of faces.
    pub fn new(pointer: std::sync::Arc<dyn Hittable + Send + Sync>) -> Self {
        Self { pointer }
    }
}

impl Hittable for FlipFace {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        t_min: std::primitive::f64,
        t_max: std::primitive::f64,
        rec: &mut super::HitRecord,
    ) -> std::primitive::bool {
        if !self.pointer.hit(r, t_min, t_max, rec) {
            return false;
        }

        rec.front_face = !rec.front_face;
        true
    }

    fn bounding_box(
        &self,
        t0: std::primitive::f64,
        t1: std::primitive::f64,
        output_box: &mut crate::aabb::Aabb,
    ) -> std::primitive::bool {
        self.pointer.bounding_box(t0, t1, output_box)
    }
}
