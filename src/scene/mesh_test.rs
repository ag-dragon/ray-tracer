use crate::scene::Scene;
use crate::vectors::{Vec3, Color};
use crate::material::{Lambertian, Metal};
use crate::texture::{SolidColor, ImageTexture, Checker};
use crate::shape::{Hittable, Triangle, Mesh, Sphere};
use crate::Camera;

pub fn gen_scene() -> Scene {
    let lookfrom = Vec3::new(0.0, 3.0, 4.0);
    let lookat = Vec3::new(0.0, 1.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let aspect_ratio = 3.0 / 2.0;
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        60.0,
        aspect_ratio,
        0.0,
        5.0
    );

    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();
    let mesh_material = Metal {
        albedo: Color::new(1.0, 1.0, 1.0),
        fuzz: 0.0,
    };
    let floor_material = Lambertian { albedo: SolidColor {
        color: Color::new(0.2, 0.2, 0.2)
    }};
    objects.push(Box::new(Mesh::load(String::from("./teapot.obj"), mesh_material)));
    objects.push(Box::new(Triangle::new([
                Vec3::new(4.0, 0.0, -4.0),
                Vec3::new(-4.0, 0.0, -4.0),
                Vec3::new(-4.0, 0.0, 4.0)
                ],
                floor_material.clone()
    )));
    objects.push(Box::new(Triangle::new([
                Vec3::new(4.0, 0.0, -4.0),
                Vec3::new(-4.0, 0.0, 4.0),
                Vec3::new(4.0, 0.0, 4.0)
                ],
                floor_material.clone()
    )));
    let skybox = ImageTexture::load(String::from("./examples/textures/sky.png"));
    Scene::new(cam, objects, Some(skybox))
}