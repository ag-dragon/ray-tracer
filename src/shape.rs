pub mod sphere;
pub use self::sphere::Sphere;

use crate::material::Material;
use crate::vectors::Vec3;
use crate::ray::Ray;
use num::traits::Float;
use std::rc::Rc;

#[derive(Copy, Clone)]
pub struct HitRecord<'material, T: Float> {
    pub point: Vec3<T>,
    pub normal: Vec3<T>,
    pub material: &'material dyn Material<T>,
    pub t: T,
    pub front_face: bool,
}

pub trait Hittable<T: Float> {
    fn hit(&self, ray: &Ray<T>, t_range: (T, T)) -> Option<HitRecord<T>>;
}
