use crate::texture::Texture;
use crate::vectors::Color;
use image::GenericImageView;
use num::clamp;

pub struct ImageTexture {
    pub image: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

impl ImageTexture {
    pub fn load(filepath: String) -> ImageTexture {
       let image_file = image::open(filepath).unwrap(); 
       let width = image_file.dimensions().0;
       let height = image_file.dimensions().1;

       let mut image: Vec<u8> = vec![];
       for y in 0..height {
           for x in 0..width {
               let p = image_file.get_pixel(x, y);
               image.push(p.0[0]);
               image.push(p.0[1]);
               image.push(p.0[2]);
           }
       }

       ImageTexture {
           image,
           width,
           height
       }
    }
}

impl Texture for ImageTexture {
    fn color(&self, u: f64, v: f64) -> Color {
        let s = (clamp(u, 0.0, 1.0) * (self.width as f64)) as u32;
        let t = (clamp(1.0-v, 0.0, 1.0) * (self.height as f64)) as u32;
        let color_scale = 1.0 / 255.0;
        Color::new(
            self.image[((t*self.width*3 + s*3)+0) as usize] as f64 * color_scale,
            self.image[((t*self.width*3 + s*3)+1) as usize] as f64 * color_scale,
            self.image[((t*self.width*3 + s*3)+2) as usize] as f64 * color_scale
        )
    }
}
