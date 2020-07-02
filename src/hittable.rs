//! Hittable class for keeping track of items that have been struck by a `Ray`.
//! This includes the construction of primitives like spheres and other objects
//! to place in a scene.

use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

/// A record of an object's status for being hit by a `Ray`.
#[derive(Clone, Default)]
pub struct HitRecord {
    /// Point where the `Ray` struck.
    pub p: Point3,
    /// Normal direction of the hit.
    pub normal: Vec3,
    /// Material of the object that was struck.
    pub material: std::sync::Arc<Material>,
    /// Where along the `Ray` the object was struck.
    pub t: f64,
    /// U surface coordinate of the object.
    pub u: f64,
    /// V surface coordinate of the object.
    pub v: f64,
    /// Whether the object struck was the front face.
    pub front_face: bool,
}

impl HitRecord {
    /// Determine whether the `Ray` struck from inside or outside of an object.
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
    /// Compute the bounding box of an object.
    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut crate::aabb::Aabb) -> bool;
}

mod box_prim;
mod constant_medium;
mod flip_face;
mod list;
mod sphere;
mod translate;

pub use box_prim::BoxPrim;
pub use constant_medium::ConstantMedium;
pub use flip_face::FlipFace;
pub use list::HittableList;
pub use sphere::{get_sphere_uv, MovingSphere, Sphere};
pub use translate::{RotateY, Translate};
