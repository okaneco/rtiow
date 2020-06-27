#![warn(rust_2018_idioms, unsafe_code)]

use rand::SeedableRng;

use rtiow::scene::first::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize values for the image output
    let aspect_ratio: f64 = 16.0 / 9.0;
    let img_w: u32 = 384;

    let samples: u32 = 100;
    let max_depth = 50;
    let seed: u64 = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();

    // Cli arg parsing. `-- image0.ppm samples width height`.
    let mut args = std::env::args().skip(1);
    let filename = &args.next().unwrap_or_else(|| "image0.ppm".to_owned());
    let samples = args
        .next()
        .map_or_else(|| samples, |v| v.parse().unwrap_or_else(|_| samples));
    let img_w = args
        .next()
        .map_or_else(|| img_w, |v| v.parse().unwrap_or_else(|_| img_w));
    let img_h = args.next().map_or_else(
        || (f64::from(img_w) * aspect_ratio.recip()) as u32,
        |v| {
            v.parse()
                .unwrap_or_else(|_| (f64::from(img_w) * aspect_ratio.recip()) as u32)
        },
    );
    let mut w = std::io::BufWriter::new(std::fs::File::create(&filename)?);
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);

    // Create world and camera

    let world = final_scene(&mut rng);

    let lookfrom = rtiow::vec3::Point3::new(13.0, 2.0, 3.0);
    let lookat = rtiow::vec3::Point3::new(0.0, 0.0, 0.0);
    let vup = rtiow::vec3::Vec3::new(0.0, 1.0, 0.0);
    let vfov = 20.0;
    let aspect_ratio = f64::from(img_w) * f64::from(img_h).recip();
    let focus_dist = 10.0;
    let aperture = 0.0;

    let cam = rtiow::camera::Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        focus_dist,
    );

    // let world = sometimes_refract();
    // let cam = rtiow::camera::Camera::default();

    // Raytrace!
    /* Single thread */
    // let now = std::time::Instant::now();
    // rtiow::render::run_single_ppm(
    //     &mut w, img_w, img_h, samples, max_depth, &mut rng, &world, &cam,
    // )?;
    // eprintln!("\nDone in {:.2?}.", std::time::Instant::now() - now);

    /* rayon PPM output */
    let now = std::time::Instant::now();
    rtiow::render::run_threaded_ppm(&mut w, img_w, img_h, samples, max_depth, &world, &cam)?;
    eprintln!("\nDone in {:.2?}.", std::time::Instant::now() - now);

    Ok(())
}
