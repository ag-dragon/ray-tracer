use clap::Parser;
use std::time::Instant;
use rtir::scene;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("images/image.png"))]
    file_path: String,

    #[arg(short, long, default_value_t = 800)]
    image_width: i32,

    #[arg(short, long, default_value_t = 100)]
    samples_per_pixel: i32,

    #[arg(short, long, default_value_t = 20)]
    max_depth: i32,
}

fn main() {
    let args = Args::parse();
    let file_path = args.file_path;

    let start = Instant::now();

    // Scene
    let scene = scene::dark::gen_scene();

    let image_buffer = rtir::render(
        &scene,
        args.image_width,
        args.samples_per_pixel,
        args.max_depth
    );

    println!("Time elapsed: {}ms", start.elapsed().as_millis());

    let image_height = ((args.image_width as f64) / scene.camera.aspect_ratio) as i32;
    let result = image::save_buffer(
        file_path,
        &image_buffer[..],
        args.image_width as u32,
        image_height as u32,
        image::ColorType::Rgb8
    );

    match result {
        Err(e) => eprintln!("Error saving image: {e:?}"),
        _ => (),
    }
    println!("Done!");
}
