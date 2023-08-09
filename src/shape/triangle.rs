use crate::shape::{HitRecord, Hittable};
use crate::vectors::Vec3;
use crate::ray::Ray;
use crate::material::Material;

pub struct Triangle<M: Material> {
    pub vertices: [Vec3<f64>; 3],
    pub normals: Option<[Vec3<f64>; 3]>,
    pub texture_cords: Option<[(f64, f64); 3]>,
    pub material: M,
}

impl<M: Material> Triangle<M> {
    pub fn new(vertices: [Vec3<f64>; 3], normals: Option<[Vec3<f64>; 3]>, texture_cords: Option<[(f64, f64); 3]>, material: M) -> Self {
        Self { vertices, normals, texture_cords, material }
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
        let b = s.dot(p) * inv_det;
        if b < 0.0 || b > 1.0 {
            return None;
        }

        let q = s.cross(v0v1);
        let c = ray.direction.dot(q) * inv_det;
        if c < 0.0 || b + c > 1.0 {
            return None;
        }

        let t = v0v2.dot(q) * inv_det;

        if t > t_min && t < t_max {
            let point = ray.origin + (ray.direction * t);
            let a = 1.0 - b - c;
            let normal = match self.normals {
                Some(vert_norms) => {
                    (vert_norms[0]*a + vert_norms[1]*b + vert_norms[2]*c).normalized()
                },
                None => {
                    v0v1.cross(v0v2).normalized()
                }
            };
            let (u, v) = match self.texture_cords {
                Some(vert_tex) => {
                    (
                        vert_tex[0].0*a + vert_tex[1].0*b + vert_tex[2].0*c,
                        vert_tex[0].1*a + vert_tex[1].1*b + vert_tex[2].1*c
                    )
                },
                None => (0.0, 0.0)
            };
            let front_face = ray.direction.dot(normal) < 0.0;
            Some(HitRecord {
                point,
                normal,
                material: &self.material,
                t,
                u,
                v,
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
