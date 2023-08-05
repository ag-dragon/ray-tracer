use crate::texture::Texture;
use crate::vectors::{Vec3, Color};

pub struct Checker {
    pub odd_color: Color,
    pub even_color: Color,
    pub scale: f64,
}

impl Texture for Checker {
    fn color(&self, u: f64, v: f64, p: Vec3<f64>) -> Color {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            self.odd_color
        } else {
            self.even_color
        }
    }
}
