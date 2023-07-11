use crate::shape::{HitRecord, Hittable};
use crate::vectors::Vec3;
use crate::ray::Ray;
use num::traits::Float;

pub struct Sphere<T: Float> {
    center: Vec3<T>,
    radius: T,
}

impl<T: Float> Sphere<T> {
    pub fn new(center: Vec3<T>, radius: T) -> Self {
        Self { center, radius }
    }
}

impl<T: Float> Hittable<T> for Sphere<T> {
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
            return Some(HitRecord {
                point: p,
                normal: (p - self.center) / self.radius,
                t: root,
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
            0.5
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
            0.5
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
                //assert_eq!(rec.normal, Vec3::new(0.0, 0.0, -1.0));
                assert_eq!(rec.t, 0.5);
            },
            None => assert!(false),
        }
    }
}
