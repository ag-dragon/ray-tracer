pub mod lambertian;
pub mod metal;
pub mod dielectric;
pub mod diffuse_light;
pub use self::lambertian::Lambertian;
pub use self::metal::Metal;
pub use self::dielectric::Dielectric;
pub use self::diffuse_light::DiffuseLight;

use crate::vector::Color;
use crate::ray::Ray;
use crate::shape::HitRecord;

pub struct Scatter {
    pub scattered: Ray,
    pub attenuation: Color
}

pub trait Material {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<Scatter>;
    fn emitted(&self, _u: f64, _v: f64) -> Color {
        Color::zero()
    }
}
