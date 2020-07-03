//! Handle translations of hittable objects.

use std::sync::Arc;

use crate::hittable::Hittable;

/// Allow `Hittable` objects to be translated.
#[derive(Clone)]
pub struct Translate {
    /// Pointer to the underlying object.
    pub pointer: Arc<dyn Hittable + Send + Sync>,
    /// Translation of object.
    pub offset: crate::vec3::Vec3,
}

impl Translate {
    /// Create a new translatable instance.
    pub fn new(pointer: Arc<dyn Hittable + Send + Sync>, offset: crate::vec3::Vec3) -> Self {
        Self { pointer, offset }
    }
}

impl Hittable for Translate {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        t_min: std::primitive::f64,
        t_max: std::primitive::f64,
        rec: &mut crate::hittable::HitRecord,
    ) -> std::primitive::bool {
        let moved_r = crate::ray::Ray::new(r.origin() - self.offset, r.direction(), r.time());
        if !self.pointer.hit(&moved_r, t_min, t_max, rec) {
            return false;
        }

        rec.p += self.offset;
        rec.set_face_normal(&moved_r, &rec.normal.clone());

        true
    }

    fn bounding_box(
        &self,
        t0: std::primitive::f64,
        t1: std::primitive::f64,
        output_box: &mut crate::aabb::Aabb,
    ) -> std::primitive::bool {
        if !self.pointer.bounding_box(t0, t1, output_box) {
            return false;
        }

        *output_box = crate::aabb::Aabb::new(
            &(output_box.min() + self.offset),
            &(output_box.max() + self.offset),
        );
        true
    }
}

/// Object for allowing rotation of `Hittable`s.
pub struct RotateY {
    /// Pointer to `Hittable` object.
    pub pointer: Arc<dyn Hittable + Send + Sync>,
    /// Stored sine calculation.
    pub sin_theta: f64,
    /// Stored cosine calculation.
    pub cos_theta: f64,
    /// Whether the object has a box.
    pub has_box: bool,
    /// Bounding box for the object.
    pub bbox: crate::aabb::Aabb,
}

impl RotateY {
    /// Create a new translated object over the Y axis.
    pub fn new(
        pointer: Arc<dyn Hittable + Send + Sync>,
        angle: f64,
        t_min: f64,
        t_max: f64,
    ) -> Self {
        let mut bbox = crate::aabb::Aabb::default();
        let (sin, cos) = angle.to_radians().sin_cos();
        let sin_theta = sin;
        let cos_theta = cos;
        let has_box = pointer.bounding_box(t_min, t_max, &mut bbox);

        let mut min = crate::vec3::Point3::new_with(f64::INFINITY);
        let mut max = crate::vec3::Point3::new_with(f64::NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = f64::from(i) * bbox.max().x() + f64::from(1 - i) * bbox.min().x();
                    let y = f64::from(j) * bbox.max().y() + f64::from(1 - i) * bbox.min().y();
                    let z = f64::from(k) * bbox.max().z() + f64::from(1 - i) * bbox.min().z();

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = crate::vec3::Vec3(newx, y, newz);

                    min.0 = min.0.min(tester.0);
                    max.0 = max.0.max(tester.0);
                    min.1 = min.1.min(tester.1);
                    max.1 = max.1.max(tester.1);
                    min.2 = min.2.min(tester.2);
                    max.2 = max.2.max(tester.2);
                }
            }
        }

        Self {
            pointer,
            sin_theta,
            cos_theta,
            has_box,
            bbox: crate::aabb::Aabb::new(&min, &max),
        }
    }
}

impl Hittable for RotateY {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        t_min: std::primitive::f64,
        t_max: std::primitive::f64,
        rec: &mut super::HitRecord,
    ) -> std::primitive::bool {
        let mut origin = r.origin();
        let mut direction = r.direction();

        origin.0 = self.cos_theta * r.origin().x() - self.sin_theta * r.origin().z();
        origin.2 = self.sin_theta * r.origin().x() + self.cos_theta * r.origin().z();

        direction.0 = self.cos_theta * r.direction().x() - self.sin_theta * r.direction().z();
        direction.2 = self.sin_theta * r.direction().x() + self.cos_theta * r.direction().z();

        let rotated_r = crate::ray::Ray::new(origin, direction, r.time());

        if !self.pointer.hit(&rotated_r, t_min, t_max, rec) {
            return false;
        }

        let mut p = rec.p;
        let mut normal = rec.normal;

        p.0 = self.cos_theta * rec.p.x() + self.sin_theta * rec.p.z();
        p.2 = -self.sin_theta * rec.p.x() + self.cos_theta * rec.p.z();

        normal.0 = self.cos_theta * rec.normal.x() + self.sin_theta * rec.normal.z();
        normal.2 = -self.sin_theta * rec.normal.x() + self.cos_theta * rec.normal.z();

        rec.p = p;
        rec.set_face_normal(&rotated_r, &normal);

        true
    }
    fn bounding_box(
        &self,
        _t0: std::primitive::f64,
        _t1: std::primitive::f64,
        output_box: &mut crate::aabb::Aabb,
    ) -> std::primitive::bool {
        *output_box = self.bbox;
        self.has_box
    }
}
