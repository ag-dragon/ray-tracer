pub mod vectors;
pub mod ray;
pub mod shape;
pub mod camera;
pub mod scene;
pub mod material;
pub mod texture;

use vectors::Vec3;
use camera::Camera;
use crate::scene::Scene;

use rand::{thread_rng, Rng};
use num::clamp;
use rayon::prelude::*;

pub fn render(scene: &Scene, image_width: i32, samples_per_pixel: i32, max_depth: i32) -> Vec<u8> {
    let cam = &scene.camera;
    let image_height = ((image_width as f64) / cam.aspect_ratio) as i32;

    let mut image_buffer: Vec<u8> = vec![0; (image_width * image_height * 3) as usize];
    let rows: Vec<(usize, &mut [u8])> = image_buffer.chunks_mut((image_width * 3) as usize).rev().enumerate().collect();

    rows.into_par_iter().for_each(|(j, row)| {
        let mut rng = thread_rng();
        println!("Started row {}", j);
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


            row[((i*3)+0) as usize] = (256.0 * clamp(pixel_color_sum.x.sqrt(), 0.0, 0.999)) as u8;
            row[((i*3)+1) as usize] = (256.0 * clamp(pixel_color_sum.y.sqrt(), 0.0, 0.999)) as u8;
            row[((i*3)+2) as usize] = (256.0 * clamp(pixel_color_sum.z.sqrt(), 0.0, 0.999)) as u8;
        }
        println!("Finished row {}", j);
    });

    image_buffer
}
