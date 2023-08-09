pub mod weekend;
pub mod triple_sphere;
pub mod sky;
pub mod triangle_test;
pub mod mesh_test;
pub mod cube;
use crate::shape::{Hittable, HitRecord};
use crate::ray::Ray;
use crate::Camera;
use crate::texture::ImageTexture;

pub struct Scene {
    pub camera: Camera,
    pub objects: Vec<Box<dyn Hittable>>,
    pub skybox: Option<ImageTexture>,
}

impl Scene {
    pub fn new(camera: Camera, objects: Vec<Box<dyn Hittable>>, skybox: Option<ImageTexture>) -> Self {
        Self { camera, objects, skybox }
    }
}

impl Hittable for Scene {
    fn hit(&self, ray: &Ray, (t_min, t_max): (f64, f64)) -> Option<HitRecord> {
        let mut result = None;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if let Some(temp_rec) = object.hit(ray, (t_min, closest_so_far)) {
                result = Some(temp_rec);
                closest_so_far = temp_rec.t;
            }
        }

        result
    }
}
