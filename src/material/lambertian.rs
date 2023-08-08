use crate::material::{Scatter, Material};
use crate::vectors::Vec3;
use crate::ray::Ray;
use crate::shape::HitRecord;
use crate::texture::Texture;

#[derive(Clone)]
pub struct Lambertian<T: Texture> {
    pub albedo: T,
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(&self, _: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let scatter_direction = rec.normal + Vec3::random_unit();

        let scatter_direction = if scatter_direction.near_zero() {
            rec.normal
        } else {
            scatter_direction
        };

        Some( Scatter {
            scattered: Ray::new(rec.point, scatter_direction),
            attenuation: self.albedo.color(rec.u, rec.v),
        })
    }
}
