//! Vector that simulates the path of light in a scene.

use crate::hittable::{HitRecord, Hittable};
use crate::pdf::Pdf;
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
pub fn ray_color(
    rng: &mut rand::rngs::ThreadRng,
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
    let emitted = rec.material.emitted(r, &rec);
    let mut pdf_val = 0.0;
    let mut albedo = Color::default();

    if !rec
        .material
        .scatter(rng, r, &rec, &mut albedo, &mut scattered, &mut pdf_val)
    {
        return emitted;
    }

    let light_shape = std::sync::Arc::new(crate::aarect::AaRect::new(
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        std::sync::Arc::new(crate::material::Material::default()),
        crate::aarect::Plane::Xz,
    ));
    let p0 = std::sync::Arc::new(crate::pdf::HittablePdf::new(&rec.p, light_shape));
    let p1 = std::sync::Arc::new(crate::pdf::CosPdf::new(&rec.normal));
    let p = crate::pdf::MixturePdf::new(vec![p0, p1]);

    scattered = Ray::new(rec.p, p.generate(rng), r.time());
    pdf_val = p.value(&scattered.direction());

    emitted
        + albedo
            * rec.material.scattering_pdf(rng, r, &rec, &scattered)
            * ray_color(rng, &scattered, background, world, max_depth - 1)
            * pdf_val.recip()
}
