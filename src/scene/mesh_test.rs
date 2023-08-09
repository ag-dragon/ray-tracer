use crate::scene::Scene;
use crate::vectors::{Vec3, Color};
use crate::material::{Metal, Dielectric, Lambertian};
use crate::texture::{ImageTexture, SolidColor};
use crate::shape::{Hittable, Mesh, Sphere};
use crate::Camera;

pub fn gen_scene() -> Scene {
    let lookfrom = Vec3::new(0.0, 3.0, 0.0);
    let lookat = Vec3::new(5.0, -1.0, -8.0);
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
    let mesh_material = Metal {
        albedo: Color::new(1.0, 1.0, 1.0),
        fuzz: 0.0,
    };
    let test_material = Dielectric {
        ir: 1.5
    };
    objects.push(Box::new(Mesh::load(String::from("./examples/duck.obj"),
        test_material
    )));
    objects.push(Box::new(Sphere::new(
                Vec3::new(5.0, -1.0, -8.0),
                1.0,
                Lambertian { albedo: SolidColor { color: Color::new(0.2, 0.2, 0.2) } }
    )));
    let skybox = ImageTexture::load(String::from("./examples/textures/sky.png"));
    Scene::new(cam, objects, Some(skybox))
}
