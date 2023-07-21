use crate::scene::Scene;
use crate::vectors::{Vec3, Color};
use crate::material::{Lambertian, Metal, Dielectric};
use crate::shape::{Hittable, Sphere};

use rand::{thread_rng, Rng};

pub fn gen_scene() -> Scene {
    let mut rng = thread_rng();

    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();
    let mat_ground = Lambertian { albedo: Color::new(0.8, 0.8, 0.0) };
    objects.push(Box::new(Sphere::new(
                Vec3::new(0.0, -1000.0, 0.0),
                1000.0,
                mat_ground
    )));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen();
            let center = Vec3::new(f64::from(a) + 0.9*rng.gen::<f64>(), 0.2, f64::from(b) + 0.9*rng.gen::<f64>());

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    objects.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Lambertian {
                            albedo: Color::random() * Color::random()
                        }
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    objects.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Metal {
                            albedo: Color::random_range(0.5, 1.0),
                            fuzz: rng.gen_range(0.0..0.5)
                        }
                    )));
                } else {
                    // glass
                    objects.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Dielectric {
                            ir: 1.5
                        }
                    )));
                }
            }
        }
    }
    objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Dielectric {
            ir: 1.5
        }
    )));
    objects.push(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Lambertian {
            albedo: Color::new(0.4, 0.2, 0.1)
        }
    )));
    objects.push(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Metal {
            albedo: Color::new(0.7, 0.6, 0.5),
            fuzz: 0.0
        }
    )));
    Scene::new(objects)
}
