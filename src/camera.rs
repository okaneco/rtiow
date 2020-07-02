//! Module for managing the virtual camera in a scene.

use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

/// Adjustable scene camera.
#[derive(Clone, Debug)]
pub struct Camera {
    orig: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
    time0: f64,
    time1: f64,
}

impl Camera {
    /// Create new instance of adjustable camera.
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        time0: f64,
        time1: f64,
    ) -> Self {
        let viewport_height = 2.0 * (vfov.to_radians() * 0.5).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u);

        let orig = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        Self {
            orig,
            lower_left_corner: orig - horizontal * 0.5 - vertical * 0.5 - focus_dist * w,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius: aperture * 0.5,
            time0,
            time1,
        }
    }

    /// Create a new default camera from an aspect ratio.
    pub fn new_with(img_w: u32, img_h: u32) -> Self {
        Camera::new(
            Point3::new_with(0.0),
            Point3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 1.0, 0.0),
            90.0,
            f64::from(img_w) * f64::from(img_h).recip(),
            0.0,
            1.0,
            0.0,
            1.0,
        )
    }

    /// Create a ray from the camera.
    pub fn get_ray<R: rand::Rng>(&self, rng: &mut R, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_circle(rng);
        let offset = self.u * rd.x() + self.v * rd.y();

        Ray::new(
            self.orig + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.orig - offset,
            rng.gen_range(self.time0, self.time1),
        )
    }
}

impl core::default::Default for Camera {
    fn default() -> Self {
        Camera::new(
            Point3::new_with(0.0),
            Point3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 1.0, 0.0),
            90.0,
            16.0 / 9.0,
            0.0,
            1.0,
            0.0,
            1.0,
        )
    }
}
