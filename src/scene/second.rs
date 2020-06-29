//! Scenes from the second book, *Raytracing the Next Week*.

use std::sync::Arc;

use crate::camera::Camera;
use crate::hittable::{HittableList, MovingSphere, Sphere};
use crate::material::Material::{Dielectric, Lambertian, Metallic};
use crate::material::{Diel, Lambert, Metal};
use crate::vec3::{Color, Point3, Vec3};

/// Listing 2.5: Book cover scene but with motion blur.
pub fn bouncing_spheres<R: rand::Rng>(
    rng: &mut R,
    img_w: u32,
    img_h: u32,
) -> (Camera, HittableList) {
    let mut world = HittableList::new();

    world.add(Arc::new(Sphere::new(
        Point3::new(0.5, -1000.0, 0.0),
        1000.0,
        Lambertian(Lambert::new(Color::new_with(0.5))),
    )));

    // Add more balls to the scene and randomize the radius of the smaller ones
    let bound = 15;
    for (a, b) in (-bound..bound).flat_map(|x| core::iter::repeat(x).zip(-bound..bound)) {
        let radius = rng.gen_range(0.1, 0.3);

        let choose_mat = rng.gen::<f64>();
        let center = Point3::new(
            f64::from(a) + 0.9 * rng.gen::<f64>(),
            radius,
            f64::from(b) + 0.9 * rng.gen::<f64>(),
        );

        if (center - Point3::new(4.0, radius, 0.0)).length() > 0.9 {
            if choose_mat < 0.8 {
                // diffuse
                world.add(Arc::new(MovingSphere::new(
                    center,
                    center + Vec3::new(0.0, rng.gen_range(0.0, 0.5), 0.0),
                    0.0,
                    1.0,
                    radius,
                    Lambertian(Lambert::new(Color::random(rng) * Color::random(rng))),
                )));
            } else if choose_mat < 0.95 {
                // metal
                world.add(Arc::new(Sphere::new(
                    center,
                    radius,
                    Metallic(Metal::new(Color::random_range(rng, 0.3, 1.0))),
                )));
            } else {
                // glass
                world.add(Arc::new(Sphere::new(
                    center,
                    radius,
                    Dielectric(Diel::new(1.5)),
                )));
            }
        }
    }

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Dielectric(Diel::new(1.5)),
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Lambertian(Lambert::new(Color::new(0.4, 0.2, 0.1))),
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Metallic(Metal::new(Color::new(0.7, 0.6, 0.5))),
    )));

    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let vfov = 20.0;
    let aspect_ratio = f64::from(img_w) * f64::from(img_h).recip();
    let focus_dist = 10.0;
    let aperture = 0.0;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        focus_dist,
        0.0,
        1.0,
    );
    (cam, world)
}
