mod vectors;
mod ray;
mod shape;
mod camera;
mod scene;
mod material;

use vectors::{Vec3, Color};
use camera::Camera;

use rand::{thread_rng, Rng};
use num::clamp;
use rayon::prelude::*;

use std::time::Instant;

fn main() {
    // Image
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 400;
    let image_height = ((image_width as f64) / aspect_ratio) as i32;
    let samples_per_pixel = 50;
    let max_depth = 5;

    // Scene
    let scene = scene::weekend::gen_scene();

    // Camera
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        0.1,
        10.0
    );

    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    let mut image_buffer = vec![Color::zero(); (image_width * image_height) as usize];
    let rows: Vec<(usize, &mut [Color])> = image_buffer.chunks_mut(image_width as usize).enumerate().collect();

    let start = Instant::now();
    rows.into_par_iter().for_each(|(j, row)| {
        let mut rng = thread_rng();
        eprintln!("Starting row {}", j);
        for i in 0..image_width {
            let mut pixel_color_sum = Vec3::zero();
            for _s in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen_range(0.0..1.0)) / ((image_width as f64) - 1.0);
                let v = (j as f64 + rng.gen_range(0.0..1.0)) / ((image_height as f64) - 1.0);
                let r = cam.get_ray(u, v);
                pixel_color_sum += r.color(&scene, max_depth);
            }

            let scale = 1.0 / samples_per_pixel as f64;
            pixel_color_sum.x *= scale;
            pixel_color_sum.y *= scale;
            pixel_color_sum.z *= scale;


            row[i as usize].x = pixel_color_sum.x.sqrt();
            row[i as usize].y = pixel_color_sum.y.sqrt();
            row[i as usize].z = pixel_color_sum.z.sqrt();
        }
        eprintln!("Finished row {}", j);
    });
    eprintln!("Time elapsed: {}ms", start.elapsed().as_millis());

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            println!(
                "{} {} {}",
                (256.0 * clamp(image_buffer[((j*image_width)+i) as usize].x, 0.0, 0.999)) as u8,
                (256.0 * clamp(image_buffer[((j*image_width)+i) as usize].y, 0.0, 0.999)) as u8,
                (256.0 * clamp(image_buffer[((j*image_width)+i) as usize].z, 0.0, 0.999)) as u8
            );
        }
    }

    eprintln!("Done!");
}
