//! Vector that simulates the path of light in a scene.
use rand::Rng;

use crate::hittable::{HitRecord, Hittable};
use crate::vec3::{Color, Point3, Vec3};

/// Ray struct used for calculating where light hits in the scene.
#[derive(Clone, Copy, Debug, Default)]
pub struct Ray {
    /// Origin point of the `Ray`.
    pub orig: Point3,
    /// Direction of the `Ray`.
    pub dir: Vec3,
    /// The time at which a `Ray` existed.
    pub time: f64,
}

impl Ray {
    /// Create a new `Ray`.
    pub fn new(orig: Point3, dir: Vec3, time: f64) -> Self {
        Self { orig, dir, time }
    }

    /// Return the origin of the `Ray`.
    pub fn origin(&self) -> Point3 {
        self.orig
    }

    /// Return the direction of the `Ray`.
    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    /// Return the time information of the `Ray`.
    pub fn time(&self) -> f64 {
        self.time
    }

    /// Return a point along the `Ray`.
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
}

/// Color produced by a ray bounce.
pub fn ray_color<R: Rng>(
    rng: &mut R,
    r: &Ray,
    background: &Color,
    world: &dyn Hittable,
    max_depth: u32,
) -> Color {
    let mut rec = HitRecord::default();

    // Stop gathering light when bounce limit reached
    if max_depth == 0 {
        return Color::new_with(0.0);
    }

    // If the ray misses everything, return the background color
    if !world.hit(r, 0.001, f64::INFINITY, &mut rec) {
        return *background;
    }

    let mut scattered = Ray::default();
    let mut attenuation = Color::default();
    let emitted = rec.material.emitted(rec.u, rec.v, &rec.p);

    if !rec
        .material
        .scatter(rng, r, &rec, &mut attenuation, &mut scattered)
    {
        return emitted;
    }

    emitted + attenuation * ray_color(rng, &scattered, background, world, max_depth - 1)
}
