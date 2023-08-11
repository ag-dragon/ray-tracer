use crate::material::{Scatter, Material};
use crate::vectors::Color;
use crate::ray::Ray;
use crate::shape::HitRecord;
use crate::texture::Texture;

#[derive(Clone)]
pub struct DiffuseLight<T: Texture> {
    pub emit: T,
}

impl<T: Texture> Material for DiffuseLight<T> {
    fn scatter(&self, _: &Ray, _: &HitRecord) -> Option<Scatter> {
        None
    }

    fn emitted(&self, u: f64, v: f64) -> Color {
        self.emit.color(u, v)
    }
}
