use crate::texture::Texture;
use crate::vector::Color;

#[derive(Clone)]
pub struct UVTexture;

impl Texture for UVTexture {
    fn color(&self, u: f64, v: f64) -> Color {
        Color::new(u, v, 0.0)
    }
}
