//! Scenes from the second book, *Raytracing the Next Week*.

use std::sync::Arc;

use crate::aarect::{AaRect, Plane};
use crate::bvh::BvhNode;
use crate::camera::Camera;
use crate::hittable::{FlipFace, HittableList, MovingSphere, Sphere};
use crate::material::Material::{Dielectric, DiffLight, Lambertian, Metallic};
use crate::material::{Diel, DiffuseLight, Lambert, Metal};
use crate::perlin::NoiseType;
use crate::texture::{Checker, ImageTexture, Noise, SolidColor};
use crate::vec3::{Color, Point3, Vec3};

/// Section 2.5: Book cover scene but with motion blur.
pub fn bouncing_spheres<R: rand::Rng>(
    rng: &mut R,
    img_w: u32,
    img_h: u32,
) -> (Camera, HittableList) {
    let mut world = HittableList::new();

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
                    Lambertian(Lambert::new(Arc::new(SolidColor::from_color(
                        Color::random(rng) * Color::random(rng),
                    )))),
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

    let mut world = HittableList::new_from(Arc::new(BvhNode::bvh_node(rng, &mut world, 0.0, 1.0)));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.5, -1000.0, 0.0),
        1000.0,
        Lambertian(Lambert::new(Arc::new(SolidColor::new_with(0.5)))),
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Dielectric(Diel::new(1.5)),
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Lambertian(Lambert::new(Arc::new(SolidColor::new(0.4, 0.2, 0.1)))),
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

/// Section 4.3: Checkerboard world with BVH.
pub fn checker_world<R: rand::Rng>(rng: &mut R, img_w: u32, img_h: u32) -> (Camera, HittableList) {
    let mut world = HittableList::new();

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
                    Lambertian(Lambert::new(Arc::new(SolidColor::from_color(
                        Color::random(rng) * Color::random(rng),
                    )))),
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

    let mut world = HittableList::new_from(Arc::new(BvhNode::bvh_node(rng, &mut world, 0.0, 1.0)));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.5, -1000.0, 0.0),
        1000.0,
        Lambertian(Lambert::new(Arc::new(Checker::new(
            Arc::new(SolidColor::new(0.2, 0.1, 0.7)),
            Arc::new(SolidColor::new(0.9, 0.3, 0.2)),
        )))),
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Dielectric(Diel::new(1.5)),
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Lambertian(Lambert::new(Arc::new(SolidColor::new(0.4, 0.2, 0.1)))),
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

/// Section 4.4: Rendering a scene with two checker spheres.
pub fn two_spheres<R: rand::Rng>(_rng: &mut R, img_w: u32, img_h: u32) -> (Camera, HittableList) {
    let mut world = HittableList::new();

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Lambertian(Lambert::new(Arc::new(Checker::new(
            Arc::new(SolidColor::new(0.2, 0.1, 0.7)),
            Arc::new(SolidColor::new(0.9, 0.3, 0.2)),
        )))),
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Lambertian(Lambert::new(Arc::new(Checker::new(
            Arc::new(SolidColor::new(0.2, 0.1, 0.7)),
            Arc::new(SolidColor::new(0.9, 0.3, 0.2)),
        )))),
    )));

    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let vfov = 20.0;
    let aspect_ratio = f64::from(img_w) * f64::from(img_h).recip();
    let focus_dist = 10.0;
    let aperture = 0.0;
    let time0 = 0.0;
    let time1 = 1.0;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        focus_dist,
        time0,
        time1,
    );
    (cam, world)
}

/// Section 5.1: Scene with two Perlin spheres.
pub fn perlin_spheres<R: rand::Rng>(
    _rng: &mut R,
    img_w: u32,
    img_h: u32,
) -> (Camera, HittableList) {
    let perlin_tex = Arc::new(Noise::new_with(1.0, NoiseType::Square, 1.0, 7, 10.0));

    let mut world = HittableList::with_capacity(2);
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian(Lambert::new(perlin_tex.clone())),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Lambertian(Lambert::new(perlin_tex.clone())),
    )));

    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let vfov = 40.0;
    let aspect_ratio = f64::from(img_w) * f64::from(img_h).recip();
    let focus_dist = 10.0;
    let aperture = 0.0;
    let time0 = 0.0;
    let time1 = 1.0;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        focus_dist,
        time0,
        time1,
    );
    (cam, world)
}

/// Section 6.2: Load an image texture. In `ray_color`, only return attenuation.
#[cfg(feature = "images")]
pub fn earth<R: rand::Rng>(
    _rng: &mut R,
    img_w: u32,
    img_h: u32,
) -> Result<(Camera, HittableList), Box<dyn std::error::Error>> {
    let earth_texture = ImageTexture::new("earthmap.jpg")?;
    let globe = Arc::new(Sphere::new(
        Point3::default(),
        2.0,
        Lambertian(Lambert::new(Arc::new(earth_texture))),
    ));

    let world = HittableList::new_from(globe);

    let lookfrom = Point3::new(0.0, 0.0, 12.0);
    let lookat = Point3::new_with(0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let vfov = 20.0;
    let aspect_ratio = f64::from(img_w) * f64::from(img_h).recip();
    let focus_dist = 10.0;
    let aperture = 0.0;
    let time0 = 0.0;
    let time1 = 1.0;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        focus_dist,
        time0,
        time1,
    );

    Ok((cam, world))
}

/// Section 7.4: Turning objects into lights. Scene with a sphere and rectangle
/// light.
pub fn simple_light<R: rand::Rng>(_rng: &mut R, img_w: u32, img_h: u32) -> (Camera, HittableList) {
    let mut world = HittableList::new();
    let radius = 2.0;

    let perlin_tex = Arc::new(Noise::new_with(1.0, NoiseType::Square, 1.0, 7, 10.0));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian(Lambert::new(perlin_tex.clone())),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, radius, 0.0),
        radius,
        Lambertian(Lambert::new(perlin_tex.clone())),
    )));

    let difflight = Arc::new(DiffuseLight::new(Arc::new(SolidColor::new_with(4.0))));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 7.0, 0.0),
        radius,
        DiffLight(difflight.clone()),
    )));
    world.add(Arc::new(AaRect::new(
        3.0,
        5.0,
        1.0,
        3.0,
        -2.0,
        DiffLight(difflight.clone()),
        Plane::Xy,
    )));

    let lookfrom = Point3::new(26.0, 3.0, 6.0);
    let lookat = Point3::new(0.0, 2.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let vfov = 20.0;
    let aspect_ratio = f64::from(img_w) * f64::from(img_h).recip();
    let focus_dist = 10.0;
    let aperture = 0.0;
    let time0 = 0.0;
    let time1 = 1.0;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        focus_dist,
        time0,
        time1,
    );

    (cam, world)
}

/// Section 7.6: Empty Cornell Box scene.
pub fn naive_cornell_box<R: rand::Rng>(
    _rng: &mut R,
    img_w: u32,
    img_h: u32,
) -> (Camera, HittableList) {
    let mut world = HittableList::new();

    let red = Lambertian(Lambert::new(Arc::new(SolidColor::new(0.65, 0.05, 0.05))));
    let white = Lambertian(Lambert::new(Arc::new(SolidColor::new(0.73, 0.73, 0.73))));
    let green = Lambertian(Lambert::new(Arc::new(SolidColor::new(0.12, 0.45, 0.15))));
    let difflight = Arc::new(DiffuseLight::new(Arc::new(SolidColor::new_with(15.0))));

    // Light
    world.add(Arc::new(AaRect::new(
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        DiffLight(difflight.clone()),
        Plane::Xz,
    )));

    // Planes
    world.add(Arc::new(AaRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        green.clone(),
        Plane::Yz,
    )));
    world.add(Arc::new(AaRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        red.clone(),
        Plane::Yz,
    )));
    world.add(Arc::new(AaRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
        Plane::Xz,
    )));
    world.add(Arc::new(AaRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
        Plane::Xz,
    )));
    world.add(Arc::new(AaRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
        Plane::Xy,
    )));

    let lookfrom = Point3::new(278.0, 278.0, -800.0);
    let lookat = Point3::new(278.0, 278.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let vfov = 40.0;
    let aspect_ratio = f64::from(img_w) * f64::from(img_h).recip();
    let focus_dist = 10.0;
    let aperture = 0.0;
    let time0 = 0.0;
    let time1 = 1.0;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        focus_dist,
        time0,
        time1,
    );

    (cam, world)
}

/// Section 7.7: Empty Cornell Box scene with adjusted normals.
pub fn cornell_box<R: rand::Rng>(_rng: &mut R, img_w: u32, img_h: u32) -> (Camera, HittableList) {
    let mut world = HittableList::new();

    let red = Lambertian(Lambert::new(Arc::new(SolidColor::new(0.65, 0.05, 0.05))));
    let white = Lambertian(Lambert::new(Arc::new(SolidColor::new(0.73, 0.73, 0.73))));
    let green = Lambertian(Lambert::new(Arc::new(SolidColor::new(0.12, 0.45, 0.15))));
    let difflight = Arc::new(DiffuseLight::new(Arc::new(SolidColor::new_with(15.0))));

    // Light
    world.add(Arc::new(AaRect::new(
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        DiffLight(difflight.clone()),
        Plane::Xz,
    )));

    // Planes
    world.add(Arc::new(FlipFace::new(Arc::new(AaRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        green.clone(),
        Plane::Yz,
    )))));
    world.add(Arc::new(AaRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        red.clone(),
        Plane::Yz,
    )));
    world.add(Arc::new(FlipFace::new(Arc::new(AaRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
        Plane::Xz,
    )))));
    world.add(Arc::new(AaRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
        Plane::Xz,
    )));
    world.add(Arc::new(FlipFace::new(Arc::new(AaRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
        Plane::Xy,
    )))));

    let lookfrom = Point3::new(278.0, 278.0, -800.0);
    let lookat = Point3::new(278.0, 278.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let vfov = 40.0;
    let aspect_ratio = f64::from(img_w) * f64::from(img_h).recip();
    let focus_dist = 10.0;
    let aperture = 0.0;
    let time0 = 0.0;
    let time1 = 1.0;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        focus_dist,
        time0,
        time1,
    );

    (cam, world)
}
