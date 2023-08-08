use crate::texture::Texture;
use crate::vectors::Color;

#[derive(Clone)]
pub struct Checker {
    pub odd_color: Color,
    pub even_color: Color,
    pub scale: f64,
}

impl Texture for Checker {
    fn color(&self, u: f64, v: f64) -> Color {
        let s = (u * self.scale).floor();
        let t = (v * self.scale).floor();

        if (s + t) % 2.0 == 0.0 {
            self.even_color
        } else {
            self.odd_color
        }
    }
}
