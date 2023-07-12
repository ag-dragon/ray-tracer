use crate::shape::{Hittable, HitRecord};
use crate::ray::Ray;
use num::traits::Float;

pub struct Scene<T> {
    pub objects: Vec<Box<dyn Hittable<T>>>,
}

impl<T: Float> Scene<T> {
    pub fn new(objects: Vec<Box<dyn Hittable<T>>>) -> Self {
        Self { objects }
    }
}

impl<T: Float> Hittable<T> for Scene<T> {
    fn hit(&self, ray: &Ray<T>, (t_min, t_max): (T, T)) -> Option<HitRecord<T>> {
        let mut result = None;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if let Some(temp_rec) = object.hit(ray, (t_min, closest_so_far)) {
                result = Some(temp_rec);
                closest_so_far = temp_rec.t;
            }
        }

        result
    }
}
