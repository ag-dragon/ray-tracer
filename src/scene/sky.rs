use crate::scene::Scene;
use crate::vector::{Vec3, Color};
use crate::material::Metal;
use crate::texture::ImageTexture;
use crate::shape::{Hittable, Sphere};
use crate::Camera;

pub fn gen_scene() -> Scene {
    let lookfrom = Vec3::new(1.0, 0.2, 3.0);
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
                Vec3::new(0.0, -1000.0, 0.0),
                1000.0,
                //Lambertian { albedo: SolidColor { color: Color::new(0.8, 0.8, 0.8) } }
                Metal {
                    albedo: Color::new(1.0, 1.0, 1.0),
                    fuzz: 0.0,
                }
    )));
    objects.push(Box::new(Sphere::new(
                Vec3::new(0.0, 0.5, 0.0),
                0.5,
                Metal {
                    albedo: Color::new(1.0, 1.0, 1.0),
                    fuzz: 0.0,
                }
    )));
    let skybox = ImageTexture::load(String::from("./assets/textures/sky.png"));
    Scene::new(cam, objects, Some(skybox))
}
