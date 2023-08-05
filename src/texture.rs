pub mod solid_color;
pub mod checker;
pub mod uv;
pub use self::solid_color::SolidColor;
pub use self::checker::Checker;
pub use self::uv::UVTexture;
use crate::vectors::{Vec3, Color};

pub trait Texture {
    fn color(&self, u: f64, v: f64, p: Vec3<f64>) -> Color;
}
