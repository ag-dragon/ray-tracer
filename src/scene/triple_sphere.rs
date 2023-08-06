use crate::scene::Scene;
use crate::vectors::{Vec3, Color};
use crate::material::{Lambertian};
use crate::texture::{Checker, UVTexture, ImageTexture};
use crate::shape::{Hittable, Sphere};
use crate::Camera;

pub fn gen_scene() -> Scene {
    let lookfrom = Vec3::new(0.0, 1.0, -9.0);
    let lookat = Vec3::<f64>::zero();
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let aspect_ratio = 3.0 / 2.0;
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        40.0,
        aspect_ratio,
        0.0,
        5.0
    );

    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();
    objects.push(Box::new(Sphere::new(
                Vec3::new(0.0, 0.0, 0.0),
                0.5,
                Lambertian { albedo: UVTexture {} }
    )));
    objects.push(Box::new(Sphere::new(
                Vec3::new(1.0, 0.0, 0.5),
                0.5,
                Lambertian { albedo: Checker {
                    odd_color: Color::new(0.1, 0.1, 0.1),
                    even_color: Color::new(0.9, 0.9, 0.9),
                    scale: 32.0,
                }}
    )));
    objects.push(Box::new(Sphere::new(
                Vec3::new(-1.0, 0.0, 0.5),
                0.5,
                Lambertian { albedo: ImageTexture::load(String::from("./examples/textures/earth.jpg")) }
    )));
    let skybox = ImageTexture::load(String::from("./examples/textures/envmap.jpg"));
    Scene::new(cam, objects, Some(skybox))
}
