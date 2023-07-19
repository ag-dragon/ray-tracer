pub mod lambertian;
pub mod metal;
pub use self::lambertian::Lambertian;
pub use self::metal::Metal;

use crate::vectors::Color;
use crate::ray::Ray;
use crate::shape::HitRecord;

pub struct Scatter {
    pub scattered: Ray,
    pub attenuation: Color
}

pub trait Material {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<Scatter>;
}
