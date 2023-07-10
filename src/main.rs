mod vec3;
use vec3::Vec3;

fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let r = (i as f64) / ((image_width as f64) - 1.0);
            let g = (j as f64) / ((image_height as f64) - 1.0);
            let b = 0.25;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            println!("{ir} {ig} {ib}");
        }
    }

    let v = Vec3::new(1.0, 1.0, 1.0);
    
    eprintln!("{:?}", v);
}
