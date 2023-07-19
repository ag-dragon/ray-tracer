use crate::vectors::Vec3;
use crate::ray::Ray;

pub struct Camera {
    pub origin: Vec3<f64>,
    pub horizontal: Vec3<f64>,
    pub vertical: Vec3<f64>,
    pub lower_left_corner: Vec3<f64>,
    pub u: Vec3<f64>,
    pub v: Vec3<f64>,
    pub w: Vec3<f64>,
    pub lens_radius: f64,
}

impl Camera {
    pub fn new(lookfrom: Vec3<f64>, lookat: Vec3<f64>, vup: Vec3<f64>,
        vfov: f64, aspect_ratio: f64, aperture: f64, focus_dist: f64) -> Self {
        let theta = vfov.to_radians();
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).normalized();
        let u = vup.cross(w).normalized();
        let v = w.cross(u);

        let focal_length = 1.0;

        let origin = lookfrom;
        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - w*focus_dist;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            w,
            lens_radius: aperture / 2.0
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = Vec3::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + self.horizontal*s
                + self.vertical*t - self.origin - offset,
        }
    }
}
