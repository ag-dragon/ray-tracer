use crate::texture::Texture;
use crate::vectors::{Vec3, Color};

pub struct UVTexture;

impl Texture for UVTexture {
    fn color(&self, u: f64, v: f64, p: Vec3<f64>) -> Color {
        Color::new(u, v, 0.0)
    }
}
