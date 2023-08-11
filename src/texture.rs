pub mod solid_color;
pub mod image_texture;
pub mod checker;
pub mod uv;
pub use self::solid_color::SolidColor;
pub use self::image_texture::ImageTexture;
pub use self::checker::Checker;
pub use self::uv::UVTexture;
use crate::vector::Color;

pub trait Texture {
    fn color(&self, u: f64, v: f64) -> Color;
}
