use crate::shape::{HitRecord, Hittable};
use crate::vectors::Vec3;
use crate::ray::Ray;
use crate::material::Material;
use crate::shape::triangle::Triangle;

pub struct Mesh<M: Material> {
    pub triangles: Vec<Triangle<M>>,
}

impl<M: Material> Mesh<M> {
    pub fn new(triangles: Vec<Triangle<M>>) -> Self {
        Self { triangles }
    }
}

impl<M: Material + Send + Sync> Hittable for Mesh<M> {
    fn hit(&self, ray: &Ray, (t_min, t_max): (f64, f64)) -> Option<HitRecord> {
        for triangle in &self.triangles {
            if let Some(h) = triangle.hit(ray, (t_min, t_max)) {
                return Some(h);
            }
        }
        None
    }
}
