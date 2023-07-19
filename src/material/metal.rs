use crate::material::{Scatter, Material};
use crate::vectors::Color;
use crate::vectors::Vec3;
use crate::ray::Ray;
use crate::shape::HitRecord;

pub struct Metal {
    pub albedo: Color,
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let reflected = ray_in.direction.normalized().reflect(rec.normal);
        let scattered = Ray::new(rec.point, reflected);
        if scattered.direction.dot(rec.normal) > 0.0 {
            return Some( Scatter {
                scattered,
                attenuation: self.albedo
            });
        } else {
            return None;
        }
    }
}
