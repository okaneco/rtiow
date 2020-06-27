//! Vector that simulates the path of light in a scene.
use rand::Rng;

use crate::hittable::{HitRecord, Hittable};
use crate::vec3::{Color, Point3, Vec3};

/// Ray struct used for calculating where light hits in the scene.
#[derive(Copy, Clone, Debug, Default)]
pub struct Ray {
    /// Origin point of the `Ray`.
    pub orig: Point3,
    /// Direction of the `Ray`.
    pub dir: Vec3,
}

impl Ray {
    /// Create a new `Ray`.
    pub fn new(orig: Point3, dir: Vec3) -> Self {
        Ray { orig, dir }
    }

    /// Return the origin of the `Ray`.
    pub fn origin(&self) -> Point3 {
        self.orig
    }

    /// Return the direction of the `Ray`.
    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    /// Return a point along the `Ray`.
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
}

/// Color produced by a ray bounce.
pub fn ray_color<R: Rng>(rng: &mut R, r: &Ray, world: &dyn Hittable, max_depth: u32) -> Color {
    let mut rec = HitRecord::default();
    if max_depth == 0 {
        return Color::new_with(0.0);
    }

    if world.hit(r, 0.001, f64::INFINITY, &mut rec) {
        let mut scattered = Ray::default();
        let mut attenuation = Color::default();
        if rec
            .material
            .scatter(rng, r, &rec, &mut attenuation, &mut scattered)
        {
            return attenuation * ray_color(rng, &scattered, world, max_depth - 1);
        }
    }

    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new_with(1.0) + t * Color::new(0.5, 0.7, 1.0)
}
