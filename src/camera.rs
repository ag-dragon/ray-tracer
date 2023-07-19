use crate::vectors::Vec3;
use crate::ray::Ray;

pub struct Camera {
    pub aspect_ratio: f64,
    pub viewport_height: f64,
    pub viewport_width : f64,
    pub focal_length: f64,

    pub origin: Vec3<f64>,
    pub horizontal: Vec3<f64>,
    pub vertical: Vec3<f64>,
    pub lower_left_corner: Vec3<f64>,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Vec3::<f64>::zero();
        let mut horizontal = Vec3::<f64>::zero();
        horizontal.x += viewport_width;
        let mut vertical = Vec3::<f64>::zero();
        vertical.y += viewport_height;
        let mut temp = Vec3::<f64>::zero();
        temp.z += focal_length;
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - temp;

        Self {
            aspect_ratio,
            viewport_height,
            viewport_width,
            focal_length,
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + self.horizontal*u
                + self.vertical*v - self.origin,
        }
    }
}
