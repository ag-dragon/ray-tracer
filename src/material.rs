use crate::vectors::Color;
use crate::vectors::Vec3;
use crate::ray::Ray;
use crate::shape::HitRecord;

pub struct Scatter {
    pub scattered: Ray,
    pub attenuation: Color
}

pub trait Material {
    fn scatter(&self, rec: &HitRecord) -> Option<Scatter>;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, rec: &HitRecord) -> Option<Scatter> {
        let scatter_direction = rec.normal + Vec3::random_unit();

        let scatter_direction = if scatter_direction.near_zero() {
            rec.normal
        } else {
            scatter_direction
        };

        Some( Scatter {
            scattered: Ray::new(rec.point, scatter_direction),
            attenuation: self.albedo
        })
    }
}
