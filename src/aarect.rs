//! Axis-aligned rectangles for building objects.

use crate::hittable::Hittable;
use crate::material::Material;
use crate::vec3::{Point3, Vec3};

const PADDING: f64 = 0.0001;

/// Axis-aligned rectangle in the XY plane.
#[derive(Clone)]
pub struct AaRect {
    /// Minimum first axis value.
    pub a0: f64,
    /// Maximum first axis value.
    pub a1: f64,
    /// Minimum second axis value.
    pub b0: f64,
    /// Maximum second axis value.
    pub b1: f64,
    /// Orthogonal plane distance component.
    pub k: f64,
    /// Material of the plane.
    pub mp: std::sync::Arc<Material>,
    /// Plane type of axis-aligned rectangle.
    pub plane: Plane,
}

/// Plane types for axis-aligned rectangles.
#[derive(Clone, Copy, Debug)]
pub enum Plane {
    /// XY-plane rectangle.
    Xy,
    /// XZ-plane rectangle.
    Xz,
    /// YZ-plane rectangle.
    Yz,
}

impl AaRect {
    /// Create an axis-aligned rectangle.
    pub fn new(a0: f64, a1: f64, b0: f64, b1: f64, k: f64, mp: Material, plane: Plane) -> Self {
        Self {
            a0,
            a1,
            b0,
            b1,
            k,
            mp: std::sync::Arc::new(mp),
            plane,
        }
    }
}

impl Hittable for AaRect {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut crate::hittable::HitRecord,
    ) -> bool {
        match self.plane {
            Plane::Xy => {
                let t = (self.k - r.origin().z()) * r.direction().z().recip();
                if t < t_min || t > t_max {
                    return false;
                }

                let x = r.origin().x() + t * r.direction().x();
                let y = r.origin().y() + t * r.direction().y();
                if x < self.a0 || x > self.a1 || y < self.b0 || y > self.b1 {
                    return false;
                }

                rec.u = (x - self.a0) * (self.a1 - self.a0).recip();
                rec.v = (y - self.b0) * (self.b1 - self.b0).recip();
                rec.t = t;
                let outward_normal = Vec3::new(0.0, 0.0, 1.0);
                rec.set_face_normal(r, &outward_normal);
                rec.material = self.mp.clone();
                rec.p = r.at(t);

                true
            }
            Plane::Xz => {
                let t = (self.k - r.origin().y()) / r.direction().y();
                if t < t_min || t > t_max {
                    return false;
                }

                let x = r.origin().x() + t * r.direction().x();
                let z = r.origin().z() + t * r.direction().z();
                if x < self.a0 || x > self.a1 || z < self.b0 || z > self.b1 {
                    return false;
                }

                rec.u = (x - self.a0) * (self.a1 - self.a0).recip();
                rec.v = (z - self.b0) * (self.b1 - self.b0).recip();
                rec.t = t;
                let outward_normal = Vec3::new(0.0, 1.0, 0.0);
                rec.set_face_normal(r, &outward_normal);
                rec.material = self.mp.clone();
                rec.p = r.at(t);

                true
            }
            Plane::Yz => {
                let t = (self.k - r.origin().x()) / r.direction().x();
                if t < t_min || t > t_max {
                    return false;
                }

                let y = r.origin().y() + t * r.direction().y();
                let z = r.origin().z() + t * r.direction().z();
                if y < self.a0 || y > self.a1 || z < self.b0 || z > self.b1 {
                    return false;
                }

                rec.u = (y - self.a0) * (self.a1 - self.a0);
                rec.v = (z - self.b0) * (self.b1 - self.b0);
                rec.t = t;
                let outward_normal = Vec3::new(1.0, 0.0, 0.0);
                rec.set_face_normal(r, &outward_normal);
                rec.material = self.mp.clone();
                rec.p = r.at(t);

                true
            }
        }
    }

    fn bounding_box(&self, _t0: f64, _t1: f64, output_box: &mut crate::aabb::Aabb) -> bool {
        match self.plane {
            Plane::Xy => {
                // Pad the bounding box so it doesn't have a non-zero width in any dimension
                *output_box = crate::aabb::Aabb::new(
                    &Point3::new(self.a0, self.b0, self.k - PADDING),
                    &Point3::new(self.a1, self.b1, self.k + PADDING),
                );
                true
            }
            Plane::Xz => {
                *output_box = crate::aabb::Aabb::new(
                    &Point3::new(self.a0, self.k - PADDING, self.b0),
                    &Point3::new(self.a1, self.k + PADDING, self.b1),
                );
                true
            }
            Plane::Yz => {
                *output_box = crate::aabb::Aabb::new(
                    &Point3::new(self.k - PADDING, self.a0, self.b0),
                    &Point3::new(self.k + PADDING, self.a1, self.b1),
                );
                true
            }
        }
    }
}
