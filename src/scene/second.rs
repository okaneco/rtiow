//! Scenes from the second book, *Raytracing the Next Week*.

use std::sync::Arc;

use crate::aarect::{AaRect, Plane};
use crate::bvh::BvhNode;
use crate::camera::Camera;
use crate::hittable::{
    BoxPrim, ConstantMedium, FlipFace, HittableList, MovingSphere, RotateY, Sphere, Translate,
};
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
) -> Result<(Camera, HittableList), Box<dyn std::error::Error>> {
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
                    Arc::new(Lambertian(Lambert::new(Arc::new(SolidColor::from_color(
                        Color::random(rng) * Color::random(rng),
                    ))))),
                )));
            } else if choose_mat < 0.95 {
                // metal
                world.add(Arc::new(Sphere::new(
                    center,
                    radius,
                    Metallic(Metal::new(Color::random_range(rng, 0.3, 1.0), 0.0)),
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
        Metallic(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)),
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
    Ok((cam, world))
}

/// Section 4.3: Checkerboard world with BVH.
pub fn checker_world<R: rand::Rng>(
    rng: &mut R,
    img_w: u32,
    img_h: u32,
) -> Result<(Camera, HittableList), Box<dyn std::error::Error>> {
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
                    Arc::new(Lambertian(Lambert::new(Arc::new(SolidColor::from_color(
                        Color::random(rng) * Color::random(rng),
                    ))))),
                )));
            } else if choose_mat < 0.95 {
                // metal
                world.add(Arc::new(Sphere::new(
                    center,
                    radius,
                    Metallic(Metal::new(Color::random_range(rng, 0.3, 1.0), 0.0)),
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
        Metallic(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)),
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
    Ok((cam, world))
}

/// Section 4.4: Rendering a scene with two checker spheres.
pub fn two_spheres<R: rand::Rng>(
    _rng: &mut R,
    img_w: u32,
    img_h: u32,
) -> Result<(Camera, HittableList), Box<dyn std::error::Error>> {
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
    Ok((cam, world))
}

/// Section 5.1: Scene with two Perlin spheres.
pub fn perlin_spheres<R: rand::Rng>(
    _rng: &mut R,
    img_w: u32,
    img_h: u32,
) -> Result<(Camera, HittableList), Box<dyn std::error::Error>> {
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
        Lambertian(Lambert::new(perlin_tex)),
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
    Ok((cam, world))
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
pub fn simple_light<R: rand::Rng>(
    _rng: &mut R,
    img_w: u32,
    img_h: u32,
) -> Result<(Camera, HittableList), Box<dyn std::error::Error>> {
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
        Lambertian(Lambert::new(perlin_tex)),
    )));

    let difflight = DiffuseLight::new(Arc::new(SolidColor::new_with(4.0)));
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
        Arc::new(DiffLight(difflight)),
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

    Ok((cam, world))
}

/// Section 7.6: Empty Cornell Box scene.
pub fn naive_cornell_box<R: rand::Rng>(
    _rng: &mut R,
    img_w: u32,
    img_h: u32,
) -> Result<(Camera, HittableList), Box<dyn std::error::Error>> {
    let mut world = HittableList::new();

    let red = Arc::new(Lambertian(Lambert::new(Arc::new(SolidColor::new(
        0.65, 0.05, 0.05,
    )))));
    let white = Arc::new(Lambertian(Lambert::new(Arc::new(SolidColor::new(
        0.73, 0.73, 0.73,
    )))));
    let green = Arc::new(Lambertian(Lambert::new(Arc::new(SolidColor::new(
        0.12, 0.45, 0.15,
    )))));
    let difflight = DiffuseLight::new(Arc::new(SolidColor::new_with(15.0)));

    // Light
    world.add(Arc::new(AaRect::new(
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        Arc::new(DiffLight(difflight)),
        Plane::Xz,
    )));

    // Planes
    world.add(Arc::new(AaRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        green,
        Plane::Yz,
    )));
    world.add(Arc::new(AaRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        red,
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
        white,
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

    Ok((cam, world))
}

/// Section 7.7: Empty Cornell Box scene with adjusted normals.
pub fn cornell_box<R: rand::Rng>(
    _rng: &mut R,
    img_w: u32,
    img_h: u32,
) -> Result<(Camera, HittableList), Box<dyn std::error::Error>> {
    let mut world = HittableList::new();

    let red = Arc::new(Lambertian(Lambert::new(Arc::new(SolidColor::new(
        0.65, 0.05, 0.05,
    )))));
    let white = Arc::new(Lambertian(Lambert::new(Arc::new(SolidColor::new(
        0.73, 0.73, 0.73,
    )))));
    let green = Arc::new(Lambertian(Lambert::new(Arc::new(SolidColor::new(
        0.12, 0.45, 0.15,
    )))));
    let difflight = DiffuseLight::new(Arc::new(SolidColor::new_with(15.0)));

    // Light
    world.add(Arc::new(AaRect::new(
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        Arc::new(DiffLight(difflight)),
        Plane::Xz,
    )));

    // Planes
    world.add(Arc::new(FlipFace::new(Arc::new(AaRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        green,
        Plane::Yz,
    )))));
    world.add(Arc::new(AaRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        red,
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

    // Boxes
    let box1 = Arc::new(BoxPrim::new(
        &Point3::new_with(0.0),
        &Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    let box1 = Translate::new(
        Arc::new(RotateY::new(box1, 15.0, 0.0, 1.0)),
        Vec3::new(265.0, 0.0, 295.0),
    );
    world.add(Arc::new(box1));

    let box2 = Arc::new(BoxPrim::new(
        &Point3::new_with(0.0),
        &Point3::new_with(165.0),
        white,
    ));
    let box2 = Translate::new(
        Arc::new(RotateY::new(box2, -18.0, 0.0, 1.0)),
        Vec3::new(130.0, 0.0, 65.0),
    );
    world.add(Arc::new(box2));

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

    Ok((cam, world))
}

/// Section 9.2: Cornell box scene with smoke and fog volumes.
pub fn cornell_smoke<R: rand::Rng>(
    _rng: &mut R,
    img_w: u32,
    img_h: u32,
) -> Result<(Camera, HittableList), Box<dyn std::error::Error>> {
    let mut world = HittableList::new();

    let red = Arc::new(Lambertian(Lambert::new(Arc::new(SolidColor::new(
        0.65, 0.05, 0.05,
    )))));
    let white = Arc::new(Lambertian(Lambert::new(Arc::new(SolidColor::new(
        0.73, 0.73, 0.73,
    )))));
    let green = Arc::new(Lambertian(Lambert::new(Arc::new(SolidColor::new(
        0.12, 0.45, 0.15,
    )))));
    let difflight = DiffuseLight::new(Arc::new(SolidColor::new_with(7.0)));

    // Light
    world.add(Arc::new(AaRect::new(
        113.0,
        443.0,
        127.0,
        432.0,
        554.0,
        Arc::new(DiffLight(difflight)),
        Plane::Xz,
    )));

    // Planes
    world.add(Arc::new(FlipFace::new(Arc::new(AaRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        green,
        Plane::Yz,
    )))));
    world.add(Arc::new(AaRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        red,
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

    // Boxes
    let box1 = Arc::new(BoxPrim::new(
        &Point3::new_with(0.0),
        &Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    let box1 = Translate::new(
        Arc::new(RotateY::new(box1, 15.0, 0.0, 1.0)),
        Vec3::new(265.0, 0.0, 295.0),
    );

    let box2 = Arc::new(BoxPrim::new(
        &Point3::new_with(0.0),
        &Point3::new_with(165.0),
        white,
    ));
    let box2 = Translate::new(
        Arc::new(RotateY::new(box2, -18.0, 0.0, 1.0)),
        Vec3::new(130.0, 0.0, 65.0),
    );

    world.add(Arc::new(ConstantMedium::new(
        Arc::new(box1),
        Arc::new(SolidColor::new_with(0.0)),
        0.01,
    )));
    world.add(Arc::new(ConstantMedium::new(
        Arc::new(box2),
        Arc::new(SolidColor::new_with(1.0)),
        0.01,
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

    Ok((cam, world))
}

/// Chapter 10: A scene testing all features.
#[cfg(feature = "images")]
pub fn final_scene<R: rand::Rng>(
    rng: &mut R,
    img_w: u32,
    img_h: u32,
) -> Result<(Camera, HittableList), Box<dyn std::error::Error>> {
    let mut boxes1 = HittableList::new();

    let ground = Arc::new(Lambertian(Lambert::new(Arc::new(SolidColor::new(
        0.48, 0.83, 0.53,
    )))));

    let boxes_per_side = 20;

    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + f64::from(i) * w;
            let z0 = -1000.0 + f64::from(j) * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = rng.gen_range(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add(Arc::new(BoxPrim::new(
                &Point3::new(x0, y0, z0),
                &Point3::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }

    let mut objects = HittableList::new();

    objects.add(Arc::new(BvhNode::bvh_node(rng, &mut boxes1, 0.0, 1.0)));

    // Light
    let light = DiffuseLight::new(Arc::new(SolidColor::new_with(7.0)));
    objects.add(Arc::new(AaRect::new(
        123.0,
        423.0,
        147.0,
        412.0,
        554.0,
        Arc::new(DiffLight(light)),
        Plane::Xz,
    )));

    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let moving_sphere_material = Arc::new(Lambertian(Lambert::new(Arc::new(SolidColor::new(
        0.7, 0.3, 0.1,
    )))));
    objects.add(Arc::new(MovingSphere::new(
        center1,
        center2,
        0.0,
        1.0,
        50.0,
        moving_sphere_material,
    )));

    objects.add(Arc::new(Sphere::new(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Dielectric(Diel::new(1.5)),
    )));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Metallic(Metal::new(Color::new(0.8, 0.8, 0.9), 10.0)),
    )));

    let boundary = Arc::new(Sphere::new(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Dielectric(Diel::new(1.5)),
    ));
    objects.add(boundary.clone());
    objects.add(Arc::new(ConstantMedium::new(
        boundary,
        Arc::new(SolidColor::new(0.2, 0.4, 0.9)),
        0.2,
    )));
    let boundary = Arc::new(Sphere::new(
        Point3::new_with(0.0),
        5000.0,
        Dielectric(Diel::new(1.5)),
    ));
    objects.add(Arc::new(ConstantMedium::new(
        boundary,
        Arc::new(SolidColor::new(1.0, 1.0, 1.0)),
        0.0001,
    )));

    // Earth sphere
    // let earth_texture = ImageTexture::new("earthmap.jpg")?;
    // objects.add(Arc::new(Sphere::new(
    //     Point3::new(400.0, 200.0, 400.0),
    //     100.0,
    //     Lambertian(Lambert::new(Arc::new(earth_texture))),
    // )));

    // Perlin Ball
    let perlin_tex = Arc::new(Noise::new_with(1.0, NoiseType::Marble, 0.1, 7, 10.0));
    objects.add(Arc::new(Sphere::new(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        Lambertian(Lambert::new(perlin_tex)),
    )));

    // Box of spheres
    let mut boxes2 = HittableList::new();
    let white = Lambertian(Lambert::new(Arc::new(SolidColor::new(0.73, 0.73, 0.73))));
    let ns = 1000;
    for _ in 0..ns {
        boxes2.add(Arc::new(Sphere::new(
            Point3::random_range(rng, 0.0, 165.0),
            10.0,
            white.clone(),
        )));
    }

    objects.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(BvhNode::bvh_node(rng, &mut boxes2, 0.0, 1.0)),
            15.0,
            0.0,
            1.0,
        )),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    let lookfrom = Point3::new(478.0, 278.0, -600.0);
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

    Ok((cam, objects))
}
