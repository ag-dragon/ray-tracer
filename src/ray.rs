use crate::vectors::Vec3;

use std::ops::{{Add, Mul}};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Ray<T> {
    pub origin: Vec3<T>,
    pub direction: Vec3<T>,
}

impl<T> Ray<T> where 
    T: Copy + Mul<Output = T> + Add<Output = T> { 
    pub fn new(origin: Vec3<T>, direction: Vec3<T>) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: T) -> Vec3<T> {
        self.origin + (self.direction*t)
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
