use crate::scene::Scene;
use crate::vector::{Vec3, Color};
use crate::material::{Lambertian, Metal};
use crate::texture::{ImageTexture, Checker};
use crate::shape::{Hittable, Mesh, Plane};
use crate::Camera;

pub fn gen_scene() -> Scene {
    let lookfrom = Vec3::new(0.0, 3.0, 22.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
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
    let mesh_material = Metal {
        albedo: Color::new(1.0, 1.0, 1.0),
        fuzz: 0.0,
    };
    objects.push(Box::new(Plane::new(
                Vec3::new(0.0, -10.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                Lambertian {
                    albedo: Checker {
                        odd_color: Color::new(0.2, 0.2, 0.2),
                        even_color: Color::new(0.8, 0.8, 0.8),
                        scale: 0.5,
                    }
                }
    )));
    objects.push(Box::new(Mesh::load(String::from("./assets/teapot2.obj"),
        mesh_material
    )));
    let skybox = ImageTexture::load(String::from("./assets/textures/sky.png"));
    Scene::new(cam, objects, Some(skybox), Color::zero())
}
