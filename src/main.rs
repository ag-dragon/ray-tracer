mod vectors;
mod ray;
mod shape;
mod camera;
mod scene;
mod material;

use vectors::{Vec3, Color};
use shape::{Hittable, Sphere};
use camera::Camera;
use scene::Scene;
use material::{Lambertian, Metal, Dielectric};

use rand::{thread_rng, Rng};
use num::clamp;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 800;
    let image_height = ((image_width as f64) / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // Scene
    let mat_ground = Lambertian { albedo: Color::new(0.8, 0.8, 0.0) };
    let mat_center = Lambertian { albedo: Color::new(0.1, 0.2, 0.5) };
    //let mat_left = Metal { albedo: Color::new(0.8, 0.8, 0.8), fuzz: 0.3 };
    let mat_left = Dielectric { ir: 1.5 };
    let mat_right = Metal { albedo: Color::new(0.8, 0.6, 0.2), fuzz: 0.0 };

    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();
    objects.push(Box::new(Sphere::new(
                Vec3::new(0.0, -100.5, -1.0),
                100.0,
                mat_ground
    )));
    objects.push(Box::new(Sphere::new(
                Vec3::new(0.0, 0.0, -1.0),
                0.5,
                mat_center
    )));
    objects.push(Box::new(Sphere::new(
                Vec3::new(-1.0, 0.0, -1.0),
                0.5,
                mat_left
    )));
    objects.push(Box::new(Sphere::new(
                Vec3::new(1.0, 0.0, -1.0),
                0.5,
                mat_right
    )));
    let scene = Scene::new(objects);

    // Camera
    let cam = Camera::new();

    // rng
    let mut rng = thread_rng();

    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    for j in (0..image_height).rev() {
        eprintln!("Scanelines remaining: {j}");
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

            println!(
                "{} {} {}",
                (256.0 * clamp(pixel_color_sum.x.sqrt(), 0.0, 0.999)) as u8,
                (256.0 * clamp(pixel_color_sum.y.sqrt(), 0.0, 0.999)) as u8,
                (256.0 * clamp(pixel_color_sum.z.sqrt(), 0.0, 0.999)) as u8
            );
        }
    }

    eprintln!("Done!");
}
