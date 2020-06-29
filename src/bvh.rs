//! Bounding volume hierarchy module.

use std::sync::Arc;

use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable, HittableList};

/// Bounding volume hierarchy node.
#[derive(Clone)]
pub struct BvhNode {
    /// Child to the left.
    pub left: Option<Arc<dyn Hittable + Send + Sync>>,
    /// Child to the right.
    pub right: Option<Arc<dyn Hittable + Send + Sync>>,
    /// Bounding box of node.
    pub bbox: Aabb,
}

impl BvhNode {
    /// Create a new `BvhNode`.
    pub fn new(
        left: Option<Arc<dyn Hittable + Send + Sync>>,
        right: Option<Arc<dyn Hittable + Send + Sync>>,
        bbox: Aabb,
    ) -> Self {
        Self { left, right, bbox }
    }

    /// Split the nodes into hierarchies.
    pub fn bvh_node<R: rand::Rng>(
        rng: &mut R,
        list: &mut HittableList,
        time0: f64,
        time1: f64,
    ) -> Self {
        let left;
        let right;
        let axis = match rng.gen_range(0u8, 3) {
            0 => Axis::X,
            1 => Axis::Y,
            _ => Axis::Z,
        };

        let object_span = list.objects.len();

        match object_span {
            0 => panic!("Cannot make a BVH from 0 objects!"),
            1 => {
                left = Some(list.objects.first().unwrap().clone());
                right = Some(list.objects.first().unwrap().clone());
            }
            2 => {
                if Self::box_compare(
                    list.objects.get(0).unwrap(),
                    list.objects.get(1).unwrap(),
                    axis,
                ) {
                    left = Some(list.objects.get(0).unwrap().clone());
                    right = Some(list.objects.get(1).unwrap().clone());
                } else {
                    left = Some(list.objects.get(1).unwrap().clone());
                    right = Some(list.objects.get(0).unwrap().clone());
                }
            }
            _ => {
                list.objects.sort_unstable_by(|a, b| {
                    if Self::box_compare(a, b, axis) {
                        core::cmp::Ordering::Greater
                    } else {
                        core::cmp::Ordering::Less
                    }
                });
                let mid = object_span / 2;
                left = Some(Arc::new(Self::bvh_node(
                    rng,
                    &mut HittableList {
                        objects: list.objects.drain(mid..).collect(),
                    },
                    time0,
                    time1,
                )));
                right = Some(Arc::new(Self::bvh_node(rng, list, time0, time1)));
            }
        }

        let mut box_left = Aabb::default();
        let mut box_right = Aabb::default();

        let left_node = match left.clone() {
            Some(node) => node.bounding_box(time0, time1, &mut box_left),
            None => false,
        };
        let right_node = match right.clone() {
            Some(node) => node.bounding_box(time0, time1, &mut box_right),
            None => false,
        };

        if !left_node || !right_node {
            eprintln!("No bounding box in bvh_node constructor.\n");
            return Self {
                left: None,
                right: None,
                bbox: Aabb::default(),
            };
        }

        let bbox = Aabb::surrounding_box(&box_left, &box_right);
        Self { left, right, bbox }
    }

    /// Comparator for node bounding boxes.
    pub fn box_compare(
        a: &Arc<dyn Hittable + Send + Sync>,
        b: &Arc<dyn Hittable + Send + Sync>,
        axis: Axis,
    ) -> bool {
        let mut box_a = Aabb::default();
        let mut box_b = Aabb::default();

        if !(a.bounding_box(0.0, 0.0, &mut box_a)) || !(b.bounding_box(0.0, 0.0, &mut box_b)) {
            eprintln!("No bounding box in bvh_node constructor.\n");
            return false;
        }

        match axis {
            Axis::X => box_a.min().x() < box_b.min().x(),
            Axis::Y => box_a.min().y() < box_b.min().y(),
            Axis::Z => box_a.min().z() < box_b.min().z(),
        }
    }
}

/// Cartesian axes.
#[derive(Clone, Copy, Debug)]
pub enum Axis {
    /// X-axis.
    X,
    /// Y-axis.
    Y,
    /// Z-axis.
    Z,
}

impl Hittable for BvhNode {
    fn bounding_box(&self, _t0: f64, _t1: f64, output_box: &mut crate::aabb::Aabb) -> bool {
        *output_box = self.bbox;
        true
    }

    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(r, t_min, t_max) {
            return false;
        }

        let hit_left = match &self.left {
            Some(node) => node.hit(r, t_min, t_max, rec),
            None => false,
        };
        let hit_right = match &self.right {
            Some(node) => node.hit(r, t_min, t_max, rec),
            None => false,
        };

        hit_left || hit_right
    }
}

impl core::default::Default for BvhNode {
    fn default() -> Self {
        Self {
            left: None,
            right: None,
            bbox: Aabb::default(),
        }
    }
}
