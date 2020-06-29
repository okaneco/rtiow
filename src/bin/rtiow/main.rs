#![warn(rust_2018_idioms, unsafe_code)]

use rtiow::scene::second::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize values for the image output
    let aspect_ratio: f64 = 16.0 / 9.0;
    let img_w: u32 = 384;

    let samples: u32 = 100;
    let max_depth = 50;

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
    let mut rng = rand::thread_rng();

    // Create world and camera
    let (cam, world) = perlin_spheres(&mut rng, img_w, img_h);

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
