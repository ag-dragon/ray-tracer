use crate::material::{Scatter, Material};
use crate::vectors::Color;
use crate::vectors::Vec3;
use crate::ray::Ray;
use crate::shape::HitRecord;

pub struct Dielectric {
    pub ir: f64
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let refraction_ratio = if rec.front_face { 1.0/self.ir } else { self.ir };
        
        let unit_direction = ray_in.direction.normalized();
        let cos_theta = (-unit_direction).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        let direction = if refraction_ratio * sin_theta > 1.0 {
            unit_direction.reflect(rec.normal)
        } else {
            unit_direction.refract(rec.normal, refraction_ratio)
        };

        Some( Scatter {
            scattered: Ray::new(rec.point, direction),
            attenuation: Color::one()
        })
    }
}
