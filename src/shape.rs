pub mod sphere;
pub use self::sphere::Sphere;

use crate::vectors::Vec3;
use crate::ray::Ray;
use num::traits::Float;

#[derive(Copy, Clone, Debug)]
pub struct HitRecord<T: Float> {
    point: Vec3<T>,
    normal: Vec3<T>,
    t: T,

}

pub trait Hittable<T: Float> {
    fn hit(&self, ray: &Ray<T>, t_range: (T, T)) -> Option<HitRecord<T>>;
}
