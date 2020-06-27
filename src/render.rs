//! Rendering functions for ray tracing. Files are written out in PPM format.

use std::io::Write;

use rand::Rng;
#[cfg(feature = "threads")]
use rayon::prelude::*;

use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::ray::ray_color;
use crate::vec3::Color;

/// Run ray tracing in a single thread.
pub fn run_single_ppm<R: Rng, W: Write>(
    mut w: &mut W,
    img_w: u32,
    img_h: u32,
    samples: u32,
    max_depth: u32,
    mut rng: &mut R,
    world: &dyn Hittable,
    cam: &Camera,
) -> Result<(), std::io::Error> {
    writeln!(&mut w, "P3\n{} {}\n255", img_w, img_h)?;

    for j in (0..img_h).rev() {
        eprint!("\rScanlines remaining: {}   ", j);
        std::io::stderr().flush()?;
        for i in 0..img_w {
            let pixel_color: Color = (0..samples).fold(Color::new_with(0.0), |pix, _| {
                let u = (f64::from(i) + rng.gen::<f64>()) * f64::from(img_w - 1).recip();
                let v = (f64::from(j) + rng.gen::<f64>()) * f64::from(img_h - 1).recip();
                let r = cam.get_ray(&mut rng, u, v);
                pix + ray_color(&mut rng, &r, world, max_depth)
            });
            let color = pixel_color.into_u8_color(f64::from(samples));
            writeln!(&mut w, "{} {} {}", color.0, color.1, color.2)?;
        }
    }

    Ok(())
}

/// Run multi-threaded ray tracing.
#[cfg(feature = "threads")]
pub fn run_threaded_ppm<W, H>(
    mut w: &mut W,
    img_w: u32,
    img_h: u32,
    samples: u32,
    max_depth: u32,
    world: &H,
    cam: &Camera,
) -> Result<(), std::io::Error>
where
    W: Write,
    H: Hittable + Sync,
{
    writeln!(&mut w, "P3\n{} {}\n255", img_w, img_h)?;

    let colors = (0..img_h * img_w)
        .into_par_iter()
        .map(|x| {
            let pixel_color = (0..samples).fold(Color::new_with(0.0), |pix, _| {
                let mut rng = rand::thread_rng();
                let u = (f64::from(x % img_w) + rng.gen::<f64>()) * f64::from(img_w - 1).recip();
                let v = (f64::from(img_h - 1 - x / img_w) + rng.gen::<f64>())
                    * f64::from(img_h - 1).recip();
                let r = cam.get_ray(&mut rng, u, v);
                pix + ray_color(&mut rng, &r, world, max_depth)
            });
            pixel_color.into_u8_color(f64::from(samples))
        })
        .collect::<Vec<crate::vec3::ColorU8>>();

    for color in colors {
        writeln!(&mut w, "{} {} {}", color.0, color.1, color.2)?;
    }

    Ok(())
}
