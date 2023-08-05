use crate::texture::Texture;
use crate::vectors::{Vec3, Color};

pub struct SolidColor {
    pub color: Color,
}

impl Texture for SolidColor {
    fn color(&self, u: f64, v: f64, p: Vec3<f64>) -> Color {
        self.color
    }
}
