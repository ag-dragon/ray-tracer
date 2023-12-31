use crate::scene::Scene;
use crate::vector::{Vec3, Color};
use crate::material::{Lambertian, DiffuseLight};
use crate::texture::{Checker, UVTexture, ImageTexture};
use crate::shape::{Hittable, Sphere};
use crate::Camera;

pub fn gen_scene() -> Scene {
    let lookfrom = Vec3::new(0.0, 1.0, -5.0);
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
                DiffuseLight { emit: UVTexture {} }
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
                Lambertian { albedo: ImageTexture::load(String::from("./assets/textures/earth.jpg")) }
    )));
    let skybox = ImageTexture::load(String::from("./assets/textures/stars.jpg"));
    Scene::new(cam, objects, Some(skybox), Color::zero())
}
