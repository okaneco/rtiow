//! Hittable class for keeping track of items that have been struck by a `Ray`.
//! This includes the construction of primitives like spheres and other objects
//! to place in a scene.

use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

/// A record of an object's status for being hit by a `Ray`.
#[derive(Copy, Clone, Debug, Default)]
pub struct HitRecord {
    /// Point where the `Ray` struck.
    pub p: Point3,
    /// Normal direction of the hit.
    pub normal: Vec3,
    /// Material of the object that was struck.
    pub material: Material,
    /// Where along the `Ray` the object was struck.
    pub t: f64,
    /// Whether the object struck was the front face.
    pub front_face: bool,
}

impl HitRecord {
    /// Determine whether the `Ray` struck the outward face of an object.
    #[inline]
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.direction().dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal.unit_vector()
        } else {
            -outward_normal.unit_vector()
        };
    }
}

/// Trait for making objects able to be hit by a `Ray`.
pub trait Hittable {
    /// Determine whether a ray hits an object.
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

mod list;
mod sphere;

pub use list::HittableList;
pub use sphere::{MovingSphere, Sphere};
