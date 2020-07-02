//! Box primitive object for raytracing.

use std::sync::Arc;

use crate::aarect::{AaRect, Plane};
use crate::hittable::{flip_face::FlipFace, HitRecord, Hittable, HittableList};
use crate::material::Material;
use crate::vec3::Point3;

/// Box object.
#[derive(Clone)]
pub struct BoxPrim {
    /// Minimum point of box.
    pub box_min: Point3,
    /// Maximum point of box.
    pub box_max: Point3,
    /// Sides of the box.
    pub sides: HittableList,
}

impl BoxPrim {
    /// Creat a new box primitive.
    pub fn new(p0: &Point3, p1: &Point3, ptr: Arc<Material>) -> Self {
        let box_min = *p0;
        let box_max = *p1;

        let mut sides = HittableList::with_capacity(6);

        sides.add(Arc::new(AaRect::new(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p1.z(),
            ptr.clone(),
            Plane::Xy,
        )));
        sides.add(Arc::new(FlipFace::new(Arc::new(AaRect::new(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p0.z(),
            ptr.clone(),
            Plane::Xy,
        )))));

        sides.add(Arc::new(AaRect::new(
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p1.y(),
            ptr.clone(),
            Plane::Xz,
        )));
        sides.add(Arc::new(FlipFace::new(Arc::new(AaRect::new(
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p0.y(),
            ptr.clone(),
            Plane::Xz,
        )))));

        sides.add(Arc::new(AaRect::new(
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p1.x(),
            ptr.clone(),
            Plane::Yz,
        )));
        sides.add(Arc::new(FlipFace::new(Arc::new(AaRect::new(
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p0.x(),
            ptr,
            Plane::Yz,
        )))));

        Self {
            box_min,
            box_max,
            sides,
        }
    }
}

impl Hittable for BoxPrim {
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        self.sides.hit(r, t_min, t_max, rec)
    }

    fn bounding_box(&self, _t0: f64, _t1: f64, output_box: &mut crate::aabb::Aabb) -> bool {
        *output_box = crate::aabb::Aabb::new(&self.box_min, &self.box_max);
        true
    }
}
