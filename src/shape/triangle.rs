use crate::shape::{HitRecord, Hittable};
use crate::vectors::Vec3;
use crate::ray::Ray;
use crate::material::Material;

pub struct Triangle<M: Material> {
    pub vertices: [Vec3<f64>; 3],
    pub material: M,
}

impl<M: Material> Triangle<M> {
    pub fn new(vertices: [Vec3<f64>; 3], material: M) -> Self {
        Self { vertices, material }
    }
}

impl<M: Material + Send + Sync> Hittable for Triangle<M> {
    fn hit(&self, ray: &Ray, (t_min, t_max): (f64, f64)) -> Option<HitRecord> {
        // Ray-triangle intersection from scratchapixel
        let v0v1 = self.vertices[1] - self.vertices[0];
        let v0v2 = self.vertices[2] - self.vertices[0];
        let p = ray.direction.cross(v0v2);
        let det = v0v1.dot(p);

        if det.abs() < std::f64::EPSILON {
            return None;
        }

        let inv_det = 1.0 / det;

        let s = ray.origin - self.vertices[0];
        let u = s.dot(p) * inv_det;
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = s.cross(v0v1);
        let v = ray.direction.dot(q) * inv_det;
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = v0v2.dot(q) * inv_det;

        if t > t_min && t < t_max {
            let point = ray.origin + (ray.direction * t);
            let normal = v0v1.cross(v0v2).normalized();
            let front_face = ray.direction.dot(normal) < 0.0;
            Some(HitRecord {
                point,
                normal,
                material: &self.material,
                t,
                u: 0.0,
                v: 0.0,
                front_face,
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
    }

    #[test]
    fn test_hit() {
    }
}
