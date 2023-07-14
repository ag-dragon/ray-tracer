use crate::vectors::Vec3;
use crate::ray::Ray;
use num::traits::Float;
use std::ops::AddAssign;

pub struct Camera<T: Float> {
    pub aspect_ratio: T,
    pub viewport_height: T,
    pub viewport_width : T,
    pub focal_length: T,

    pub origin: Vec3<T>,
    pub horizontal: Vec3<T>,
    pub vertical: Vec3<T>,
    pub lower_left_corner: Vec3<T>,
}

impl<T: Float + Copy + AddAssign> Camera<T> {
    pub fn new() -> Self {
        let aspect_ratio = T::from(16.0).unwrap() / T::from(9.0).unwrap();
        let viewport_height = T::from(2.0).unwrap();
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = T::from(1.0).unwrap();

        let origin = Vec3::<T>::zero();
        let mut horizontal = Vec3::<T>::zero();
        horizontal.x += viewport_width;
        let mut vertical = Vec3::<T>::zero();
        vertical.y += viewport_height;
        let mut temp = Vec3::<T>::zero();
        temp.z += focal_length;
        let lower_left_corner = origin - horizontal/T::from(2.0).unwrap()
            - vertical/T::from(2.0).unwrap() - temp;

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

    pub fn get_ray(&self, u: T, v: T) -> Ray<T> {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + self.horizontal*u
                + self.vertical*v - self.origin,
        }
    }
}
