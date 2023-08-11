use crate::shape::{HitRecord, Hittable};
use crate::vector::Vec3;
use crate::ray::Ray;
use crate::material::Material;

pub struct Sphere<M: Material> {
    pub center: Vec3<f64>,
    pub radius: f64,
    pub material: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Vec3<f64>, radius: f64, material: M) -> Self {
        Self { center, radius, material }
    }

    pub fn get_uv(p: Vec3<f64>) -> (f64, f64) {
        let theta = (-p.y).acos();
        let phi = (-p.z).atan2(p.x) + std::f64::consts::PI;

        (phi / (2.0*std::f64::consts::PI), theta / std::f64::consts::PI)
    }
}

impl<M: Material + Send + Sync> Hittable for Sphere<M> {
    fn hit(&self, ray: &Ray, (t_min, t_max): (f64, f64)) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius*self.radius;

        let discriminant = half_b*half_b - a*c;
        let sqrtd = discriminant.sqrt();

        if discriminant >= 0.0 {
            let mut root = (-half_b - sqrtd) / a;
            if root < t_min || t_max < root {
                root = (-half_b + sqrtd) / a;
                if root < t_min || t_max < root {
                    return None;
                }
            }

            let p = ray.at(root);
            let outward_normal = (p - self.center) / self.radius;
            let front_face = ray.direction.dot(outward_normal) < 0.0;
            let (u, v) = Sphere::<M>::get_uv(outward_normal);
            return Some(HitRecord {
                point: p,
                normal: if front_face { outward_normal } else { -outward_normal },
                material: &self.material,
                t: root,
                u,
                v,
                front_face,
            })
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vectors::Color;
    use crate::material::Lambertian;
    use crate::texture::SolidColor;

    #[test]
    fn test_new() {
        let s = Sphere::new(
            Vec3::new(1.0, 2.0, 3.0),
            0.5,
            Lambertian {
                albedo: SolidColor { color: Color::new(0.5, 0.5, 0.5) }
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
                albedo: SolidColor { color: Color::new(0.5, 0.5, 0.5) }
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
