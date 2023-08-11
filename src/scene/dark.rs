use crate::scene::Scene;
use crate::vector::{Vec3, Color};
use crate::material::{Lambertian, DiffuseLight, Metal, Dielectric};
use crate::texture::{Checker, ImageTexture, SolidColor};
use crate::shape::{Hittable, Sphere, Plane};
use crate::Camera;

pub fn gen_scene() -> Scene {
    let lookfrom = Vec3::new(0.0, 0.0, 3.0);
    let lookat = Vec3::<f64>::zero();
    let vup = Vec3::new(0.0, 1.0, 1.0);
    let aspect_ratio = 3.0 / 2.0;
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        70.0,
        aspect_ratio,
        0.0,
        5.0
    );

    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();
    objects.push(Box::new(Sphere::new(
                Vec3::new(0.0, 1.0, 0.0),
                0.5,
                DiffuseLight { emit: SolidColor { color: Color::new(4.0, 4.0, 4.0) } }
    )));
    objects.push(Box::new(Plane::new(
                Vec3::new(0.0, -1.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                Lambertian {
                    albedo: Checker {
                        odd_color: Color::new(0.2, 0.2, 0.2),
                        even_color: Color::new(0.8, 0.8, 0.8),
                        scale: 0.5,
                    }
                }
    )));
    Scene::new(cam, objects, None, Color::zero())
}
