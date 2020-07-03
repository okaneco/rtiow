//! Scenes from the third book, *Raytracing the Rest of Your Life*.

use std::sync::Arc;

use crate::aarect::{AaRect, Plane};
use crate::camera::Camera;
use crate::hittable::{BoxPrim, FlipFace, HittableList, RotateY, Translate};
use crate::material::Material::{DiffLight, Lambertian, Metallic};
use crate::material::{DiffuseLight, Lambert, Metal};
use crate::texture::SolidColor;
use crate::vec3::{Color, Point3, Vec3};

/// Section 6.1: Refactored Cornell box.
pub fn cornell_box<R: rand::Rng>(
    _rng: &mut R,
    img_w: u32,
    img_h: u32,
) -> Result<
    (
        Camera,
        HittableList,
        Arc<dyn crate::hittable::Hittable + Send + Sync>,
    ),
    Box<dyn std::error::Error>,
> {
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
    world.add(Arc::new(FlipFace::new(Arc::new(AaRect::new(
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        Arc::new(DiffLight(difflight)),
        Plane::Xz,
    )))));
    let lights = std::sync::Arc::new(crate::aarect::AaRect::new(
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        std::sync::Arc::new(crate::material::Material::default()),
        crate::aarect::Plane::Xz,
    ));

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

    Ok((cam, world, lights))
}

/// Section 12.2: Cornell box with metallic block.
pub fn cornell_box_metal<R: rand::Rng>(
    _rng: &mut R,
    img_w: u32,
    img_h: u32,
) -> Result<
    (
        Camera,
        HittableList,
        Arc<dyn crate::hittable::Hittable + Send + Sync>,
    ),
    Box<dyn std::error::Error>,
> {
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
    world.add(Arc::new(FlipFace::new(Arc::new(AaRect::new(
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        Arc::new(DiffLight(difflight)),
        Plane::Xz,
    )))));
    let lights = std::sync::Arc::new(crate::aarect::AaRect::new(
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        std::sync::Arc::new(crate::material::Material::default()),
        crate::aarect::Plane::Xz,
    ));

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
    let aluminum = Arc::new(Metallic(Metal::new(Color::new(0.8, 0.85, 0.88), 0.0)));
    let box1 = Arc::new(BoxPrim::new(
        &Point3::new_with(0.0),
        &Point3::new(165.0, 330.0, 165.0),
        aluminum,
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

    Ok((cam, world, lights))
}

/// Section 12.4: Cornell box with glass sphere.
pub fn cornell_box_sphere<R: rand::Rng>(
    _rng: &mut R,
    img_w: u32,
    img_h: u32,
) -> Result<
    (
        Camera,
        HittableList,
        Arc<dyn crate::hittable::Hittable + Send + Sync>,
    ),
    Box<dyn std::error::Error>,
> {
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
    world.add(Arc::new(FlipFace::new(Arc::new(AaRect::new(
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        Arc::new(DiffLight(difflight)),
        Plane::Xz,
    )))));
    let lights = std::sync::Arc::new(crate::aarect::AaRect::new(
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        std::sync::Arc::new(crate::material::Material::default()),
        crate::aarect::Plane::Xz,
    ));

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
        white,
    ));
    let box1 = Translate::new(
        Arc::new(RotateY::new(box1, 15.0, 0.0, 1.0)),
        Vec3::new(265.0, 0.0, 295.0),
    );
    world.add(Arc::new(box1));

    let glass_sphere = crate::hittable::Sphere::new(
        Point3::new(190.0, 90.0, 190.0),
        90.0,
        crate::material::Material::Dielectric(crate::material::Diel::new(1.5)),
    );
    world.add(Arc::new(glass_sphere));

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

    Ok((cam, world, lights))
}
