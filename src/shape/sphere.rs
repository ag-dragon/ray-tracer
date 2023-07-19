use crate::shape::{HitRecord, Hittable};
use crate::vectors::Vec3;
use crate::vectors::Color;
use crate::ray::Ray;
use crate::material::Material;
use crate::material::Lambertian;
use num::traits::Float;

pub struct Sphere<T: Float, M: Material<T>> {
    pub center: Vec3<T>,
    pub radius: T,
    pub material: M,
}

impl<T: Float, M: Material<T>> Sphere<T, M> {
    pub fn new(center: Vec3<T>, radius: T, material: M) -> Self {
        Self { center, radius, material }
    }
}

impl<T: Float, M: Material<T>> Hittable<T> for Sphere<T, M> {
    fn hit(&self, ray: &Ray<T>, (t_min, t_max): (T, T)) -> Option<HitRecord<T>> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius*self.radius;

        let discriminant = half_b*half_b - a*c;
        let sqrtd = discriminant.sqrt();

        if discriminant >= T::zero() {
            let mut root = (-half_b - sqrtd) / a;
            if root < t_min || t_max < root {
                root = (-half_b + sqrtd) / a;
                if root < t_min || t_max < root {
                    return None;
                }
            }

            let p = ray.at(root);
            let outward_normal = (p - self.center) / self.radius;
            let front_face = ray.direction.dot(outward_normal) < T::zero();
            return Some(HitRecord {
                point: p,
                normal: if front_face { outward_normal } else { -outward_normal },
                material: &self.material,
                t: root,
                front_face,
            })
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let s = Sphere::new(
            Vec3::new(1.0, 2.0, 3.0),
            0.5,
            Lambertian {
                albedo: Color::new(0.5, 0.5, 0.5)
            }
        );

        assert_eq!(s.center.x, 1.0);
        assert_eq!(s.center.y, 2.0);
        assert_eq!(s.center.z, 3.0);
        assert_eq!(s.radius, 0.5);
    }

    #[test]
    fn test_hit() {
        let s = Sphere::new(
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            Lambertian {
                albedo: Color::new(0.5, 0.5, 0.5)
            }
        );
        let r = Ray::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0)
        );
        let h = s.hit(&r, (0.0, 2.0));
        assert!(h.is_none());

        let r = Ray::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -1.0)
        );
        let h = s.hit(&r, (0.0, 2.0));
        match h {
            Some(rec) => {
                assert_eq!(rec.point, Vec3::new(0.0, 0.0, -0.5));
                assert_eq!(rec.normal, Vec3::new(0.0, 0.0, 1.0));
                assert_eq!(rec.t, 0.5);
                assert!(rec.front_face);
            },
            None => assert!(false),
        }
    }
}
