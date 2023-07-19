pub mod sphere;
pub use self::sphere::Sphere;

use crate::material::Material;
use crate::vectors::Vec3;
use crate::ray::Ray;

#[derive(Copy, Clone)]
pub struct HitRecord<'material> {
    pub point: Vec3<f64>,
    pub normal: Vec3<f64>,
    pub material: &'material dyn Material,
    pub t: f64,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_range: (f64, f64)) -> Option<HitRecord>;
}
