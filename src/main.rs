mod vec3;
use vec3::Vec3;

type Color = Vec3<i32>;

fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let pixel_color = Color::new(
                (((i as f64) / ((image_width as f64) - 1.0)) * 255.999) as i32,
                (((j as f64) / ((image_height as f64) - 1.0)) * 255.999) as i32,
                (0.25 * 255.999) as i32
            );

            println!(
                "{} {} {}",
                pixel_color.x,
                pixel_color.y,
                pixel_color.z
            );
        }
    }
}
