//! Vector that simulates the path of light in a scene.

use crate::hittable::{HitRecord, Hittable};
use crate::material::ScatterRecord;
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
    lights: std::sync::Arc<dyn Hittable + Send + Sync>,
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

    let mut srec = ScatterRecord::default();
    let emitted = rec.material.emitted(r, &rec);
    if !rec.material.scatter(rng, &r, &rec, &mut srec) {
        return emitted;
    }
    if let Some(_) = srec.specular_ray {
        return srec.attenuation
            * ray_color(
                rng,
                &srec.specular_ray.unwrap(),
                background,
                world,
                lights,
                max_depth - 1,
            );
    }

    let light_ptr = std::sync::Arc::new(crate::pdf::HittablePdf::new(&rec.p, lights.clone()));
    let p = crate::pdf::MixturePdf {
        p0: light_ptr,
        p1: srec.pdf_ptr.unwrap(),
    };

    let scattered = Ray::new(rec.p, p.generate(rng), r.time());
    let pdf_val = p.value(&scattered.direction());

    emitted
        + srec.attenuation
            * rec.material.scattering_pdf(rng, r, &rec, &scattered)
            * ray_color(rng, &scattered, background, world, lights, max_depth - 1)
            * pdf_val.recip()
}
