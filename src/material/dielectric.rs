use crate::material::{Scatter, Material};
use crate::vectors::Color;
use crate::ray::Ray;
use crate::shape::HitRecord;
use rand::{thread_rng, Rng};

#[derive(Clone)]
pub struct Dielectric {
    pub ir: f64
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let refraction_ratio = if rec.front_face { 1.0/self.ir } else { self.ir };
        
        let unit_direction = ray_in.direction.normalized();
        let cos_theta = (-unit_direction).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        let direction = if refraction_ratio * sin_theta > 1.0
            || reflectance(cos_theta, refraction_ratio) > thread_rng().gen() {
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

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0-ref_idx) / (1.0+ref_idx);
    let r0 = r0*r0;
    r0 + (1.0-r0)*(1.0-cosine).powi(5)
}
