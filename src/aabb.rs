//! Axis-alligned bounding box used for Bounding Volume Hierarchy calculation.

use crate::ray::Ray;
use crate::vec3::Point3;

/// Axis-aligned bounding box.
#[derive(Clone, Copy, Default, Debug)]
pub struct Aabb {
    /// The louwer boundary of the box.
    pub min: Point3,
    /// The upper boundary of the box.
    pub max: Point3,
}

impl Aabb {
    /// Create a new axis-aligned bounding box.
    pub fn new(a: &Point3, b: &Point3) -> Self {
        Self { min: *a, max: *b }
    }

    /// Return the min.
    pub fn min(&self) -> Point3 {
        self.min
    }

    /// Return the max.
    pub fn max(&self) -> Point3 {
        self.max
    }

    /// Check whether a ray strikes the bounding box.
    pub fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> bool {
        let inv_d = r.direction().x().recip();
        let mut t0 = (self.min().x() - r.origin().x()) * inv_d;
        let mut t1 = (self.max().x() - r.origin().x()) * inv_d;
        if inv_d < 0.0 {
            core::mem::swap(&mut t0, &mut t1);
        }
        let tmin = if t0 > tmin { t0 } else { tmin };
        let tmax = if t1 < tmax { t1 } else { tmax };
        if tmax <= tmin {
            return false;
        }
        let inv_d = r.direction().y().recip();
        let mut t0 = (self.min().y() - r.origin().y()) * inv_d;
        let mut t1 = (self.max().y() - r.origin().y()) * inv_d;
        if inv_d < 0.0 {
            core::mem::swap(&mut t0, &mut t1);
        }
        let tmin = if t0 > tmin { t0 } else { tmin };
        let tmax = if t1 < tmax { t1 } else { tmax };
        if tmax <= tmin {
            return false;
        }
        let inv_d = r.direction().z().recip();
        let mut t0 = (self.min().z() - r.origin().z()) * inv_d;
        let mut t1 = (self.max().z() - r.origin().z()) * inv_d;
        if inv_d < 0.0 {
            core::mem::swap(&mut t0, &mut t1);
        }
        let tmin = if t0 > tmin { t0 } else { tmin };
        let tmax = if t1 < tmax { t1 } else { tmax };
        if tmax <= tmin {
            return false;
        }

        true
    }

    /// Compute the bounding box of two boxes.
    pub fn surrounding_box(box0: &Self, box1: &Self) -> Self {
        let a = Point3::new(
            box0.min().x().min(box1.min().x()),
            box0.min().y().min(box1.min().y()),
            box0.min().z().min(box1.min().z()),
        );
        let b = Point3::new(
            box0.max().x().max(box1.max().x()),
            box0.max().y().max(box1.max().y()),
            box0.max().z().max(box1.max().z()),
        );

        Aabb::new(&a, &b)
    }
}
