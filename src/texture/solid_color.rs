use crate::texture::Texture;
use crate::vectors::Color;

pub struct SolidColor {
    pub color: Color,
}

impl Texture for SolidColor {
    fn color(&self, _u: f64, _v: f64) -> Color {
        self.color
    }
}
