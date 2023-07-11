mod vectors;
mod ray;
mod shape;

use vectors::Vec3;
use ray::Ray;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = ((image_width as f64) / aspect_ratio) as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin: Vec3<f64> = Vec3::zero();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - (horizontal/2.0)
        - (vertical/2.0) - Vec3::new(0.0, 0.0, focal_length);

    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let u = (i as f64) / ((image_width as f64) - 1.0);
            let v = (j as f64) / ((image_height as f64) - 1.0);
            let r = Ray::new(
                origin,
                lower_left_corner + horizontal*u + vertical*v - origin
                );

            let pixel_color = r.color();
            println!(
                "{} {} {}",
                pixel_color.x,
                pixel_color.y,
                pixel_color.z
            );
        }
    }
}
