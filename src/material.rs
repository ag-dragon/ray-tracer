
use crate::vectors::Color;
use crate::vectors::Vec3;
use crate::ray::Ray;
use crate::shape::HitRecord;
use num::traits::{Num, Float};
use rand::distributions::uniform::SampleUniform;
use rand::distributions::Standard;
use rand::prelude::Distribution;

pub struct Scatter<T: Float> {
    pub scattered: Ray<T>,
    pub attenuation: Color<T>
}

pub trait Material<T: Float> {
    fn scatter(&self, rec: &HitRecord<T>) -> Option<Scatter<T>>;
}

pub struct Lambertian<T: Num + Copy> {
    pub albedo: Color<T>,
}

impl<T: Float + SampleUniform> Material<T> for Lambertian<T> where Standard: Distribution<T> {
    fn scatter(&self, rec: &HitRecord<T>) -> Option<Scatter<T>> {
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
