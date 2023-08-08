use crate::scene::Scene;
use crate::vectors::{Vec3, Color};
use crate::material::Lambertian;
use crate::texture::SolidColor;
use crate::shape::{Hittable, Triangle};
use crate::Camera;

pub fn gen_scene() -> Scene {
    let lookfrom = Vec3::new(0.0, 0.0, 3.0);
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
    objects.push(Box::new(Triangle::new(
                [
                    Vec3::new(0.0, 1.0, 0.0),
                    Vec3::new(1.0, -1.0, 0.0),
                    Vec3::new(-1.0, -1.0, 0.0)
                ],
                Lambertian {
                    albedo: SolidColor { color: Color::new(0.1, 0.8, 0.1) }
                }
    )));
    Scene::new(cam, objects, None)
}
