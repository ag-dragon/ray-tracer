use crate::shape::{HitRecord, Hittable};
use crate::vectors::Vec3;
use crate::ray::Ray;
use crate::material::Material;

pub struct Plane<M: Material> {
    pub position: Vec3<f64>,
    pub normal: Vec3<f64>,
    pub material: M,
}

impl<M: Material> Plane<M> {
    pub fn new(position: Vec3<f64>, normal: Vec3<f64>, material: M) -> Self {
        Self { position, normal, material }
    }
}

impl<M: Material + Send + Sync> Hittable for Plane<M> {
    fn hit(&self, ray: &Ray, (t_min, t_max): (f64, f64)) -> Option<HitRecord> {
        // This function feels messy
        let d = self.normal.dot(ray.direction);
        if d.abs() > 1e-6 {
            let dist = self.position - ray.origin;
            let t = dist.dot(self.normal) / d;
            if t > t_min && t < t_max {
                let point = ray.origin + (ray.direction * t);

                let mut e1 = self.normal.cross(Vec3::new(1.0, 0.0, 0.0)).normalized();
                if e1 == Vec3::<f64>::zero() {
                    e1 = self.normal.cross(Vec3::new(0.0, 0.0, 1.0)).normalized();
                }
                let e2 = self.normal.cross(e1).normalized();
                let u = e1.dot(point);
                let v = e2.dot(point);

                let front_face = ray.direction.dot(self.normal) < 0.0;

                Some(HitRecord {
                    point,
                    normal: if front_face { self.normal} else { -self.normal},
                    material: &self.material,
                    t,
                    u,
                    v,
                    front_face,
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}
