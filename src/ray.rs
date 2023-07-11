use num::traits::Float;

use crate::vectors::Vec3;
use crate::vectors::Color;

#[derive(Copy, Clone, Debug)]
pub struct Ray<T: Float> {
    pub origin: Vec3<T>,
    pub direction: Vec3<T>,
}

impl<T: Float> Ray<T> {
    pub fn new(origin: Vec3<T>, direction: Vec3<T>) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: T) -> Vec3<T> {
        self.origin + (self.direction*t)
    }

    pub fn color(&self) -> Color {
        let mut c = Vec3::<T>::zero();
        c.z = T::from(-1.0).unwrap();
        let mut t = self.hit_sphere(c, 0.5);
        if t > 0.0 {
            let mut n = (self.at(T::from(t).unwrap()) - c).normalized();
            n += Vec3::<T>::one();
            n *= T::from(0.5).unwrap();
            return Color::new(
                num::cast(n.x * T::from(255.999).unwrap()).unwrap(),
                num::cast(n.y * T::from(255.999).unwrap()).unwrap(),
                num::cast(n.z * T::from(255.999).unwrap()).unwrap()
                )
        }
        let unit_direction = self.direction.normalized();
        t = num::cast((unit_direction.y + T::from(1.0).unwrap()) * T::from(0.5).unwrap()).unwrap();

        let res = Vec3::<T>::one()*(T::from(1.0).unwrap() - T::from(t).unwrap())
            + Vec3::new(T::from(0.5).unwrap(), T::from(0.7).unwrap(), T::from(1.0).unwrap())*T::from(t).unwrap();
        Color::new(
            num::cast(res.x * T::from(255.999).unwrap()).unwrap(),
            num::cast(res.y * T::from(255.999).unwrap()).unwrap(),
            num::cast(res.z * T::from(255.999).unwrap()).unwrap()
        )
    }

    fn hit_sphere(&self, center: Vec3<T>, radius: f64) -> f64 {
        let oc = self.origin - center;
        let a = self.direction.length_squared();
        let half_b = oc.dot(self.direction);
        let c = oc.length_squared() - T::from(radius*radius).unwrap();
        let discriminant = half_b*half_b - a*c;
        if (discriminant < T::from(0.0).unwrap()) { -1.0 } else {
            num::cast((-half_b - discriminant.sqrt()) / a).unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let r = Ray::new(
            Vec3::new(1.0, 2.0, 3.0),
            Vec3::new(4.0, 5.0, 6.0)
        );
        assert_eq!(r.origin.x, 1.0);
        assert_eq!(r.origin.y, 2.0);
        assert_eq!(r.origin.z, 3.0);
        assert_eq!(r.direction.x, 4.0);
        assert_eq!(r.direction.y, 5.0);
        assert_eq!(r.direction.z, 6.0);
    }

    #[test]
    fn test_at() {
        let r = Ray::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 2.0, 3.0)
        );
        let p = r.at(0.5);
        assert_eq!(p.x, 0.5);
        assert_eq!(p.y, 1.0);
        assert_eq!(p.z, 1.5);
    }
}
