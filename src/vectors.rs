use num::traits::{Float, Num};
use rand::{thread_rng, Rng};
use rand::prelude::Distribution;
use rand::distributions::{Standard, uniform::SampleUniform};
use std::cmp::PartialOrd;
use std::ops::{
    Add, AddAssign,
    Div, DivAssign,
    Index, IndexMut,
    Mul, MulAssign, Neg,
    Sub, SubAssign,
};

pub type Color<T> = Vec3<T>;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vec3<T: Num + Copy> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Num + Copy> Vec3<T> {
    #[inline(always)]
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    #[inline(always)]
    pub fn zero() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        }
    }

    #[inline(always)]
    pub fn one() -> Self {
        Self {
            x: T::one(),
            y: T::one(),
            z: T::one(),
        }
    }

    // Returns random vec3 over distribution
    // (see rand distribution documentaiton)
    pub fn random() -> Self where
        Standard: Distribution<T> {
        let mut rng = thread_rng();
        Self {
            x: rng.gen(),
            y: rng.gen(),
            z: rng.gen()
        }
    }

    // Returns random vec3 within given range
    pub fn random_range(min: T, max: T) -> Self where 
        Standard: Distribution<T>,
        T: PartialOrd + SampleUniform {
        let mut rng = thread_rng();
        Self {
            x: rng.gen_range(min..max),
            y: rng.gen_range(min..max),
            z: rng.gen_range(min..max)
        }
    }

    pub fn random_in_unit_sphere() -> Self where 
        Standard: Distribution<T>,
        T: PartialOrd + SampleUniform + Neg<Output = T> {
        loop {
            let p = Vec3::<T>::random_range(
                -T::one(),
                T::one()
            );
            if p.length_squared() < T::one() {
                return p;
            }
        }
    }

    pub fn random_unit() -> Self where
        Standard: Distribution<T>,
        T: PartialOrd + SampleUniform
            + Neg<Output = T>  + Float {
        Vec3::<T>::random_in_unit_sphere().normalized()
    }

    #[inline(always)]
    pub fn length(self) -> T where
        T: Float {
        self.length_squared().sqrt()
    }

    #[inline(always)]
    pub fn length_squared(self) -> T {
        self.x*self.x + self.y*self.y + self.z*self.z
    }

    #[inline(always)]
    pub fn dot(self, rhs: Self) -> T {
        self.x*rhs.x + self.y*rhs.y + self.z*rhs.z
    }

    #[inline(always)]
    pub fn cross(self, rhs: Self) -> Self {
        Self {
            x: self.y*rhs.z - self.z*rhs.y,
            y: self.z*rhs.x - self.x*rhs.z,
            z: self.x*rhs.y - self.y*rhs.x,
        }
    }

    #[inline(always)]
    pub fn normalized(self) -> Self where
        T: Float {
        self / self.length()
    }
}

impl<T: Num + Copy> Add<Vec3<T>> for Vec3<T> {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: Num + Copy> Add<T> for Vec3<T> {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: T) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl<T: Num + Copy> AddAssign<Vec3<T>> for Vec3<T> where {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<T: Num + Copy> AddAssign<T> for Vec3<T> {
    #[inline(always)]
    fn add_assign(&mut self, rhs: T) {
        *self = *self + rhs;
    }
}

impl<T: Num + Copy> Sub<Vec3<T>> for Vec3<T> {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: Num + Copy> Sub<T> for Vec3<T> {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: T) -> Self::Output {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

impl<T: Num + Copy> SubAssign<Vec3<T>> for Vec3<T> {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<T: Num + Copy> SubAssign<T> for Vec3<T> {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: T) {
        *self = *self - rhs;
    }
}

impl<T: Num + Copy> Mul<Vec3<T>> for Vec3<T> {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl<T: Num + Copy> Mul<T> for Vec3<T> {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T: Num + Copy> MulAssign<Vec3<T>> for Vec3<T> {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<T: Num + Copy> MulAssign<T> for Vec3<T> {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs;
    }
}

impl<T: Num + Copy + Neg<Output = T>> Neg for Vec3<T> {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T: Num + Copy> Div<Vec3<T>> for Vec3<T> {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl<T: Num + Copy> Div<T> for Vec3<T> {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<T: Num + Copy> DivAssign<Vec3<T>> for Vec3<T> {
    #[inline(always)]
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl<T: Num + Copy> DivAssign<T> for Vec3<T> {
    #[inline(always)]
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs;
    }
}

impl<T: Num + Copy> Index<usize> for Vec3<T> {
    type Output = T;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            i => panic!("index {i} out of bounds"),
        }
    }
}

impl<T: Num + Copy> IndexMut<usize> for Vec3<T> {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            i => panic!("index {i} out of bounds"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v1.x, 1.0);
        assert_eq!(v1.y, 2.0);
        assert_eq!(v1.z, 3.0);

        let v2 = Vec3::new(3, 2, 1);
        assert_eq!(v2.x, 3);
        assert_eq!(v2.y, 2);
        assert_eq!(v2.z, 1);
    }

    #[test]
    fn test_zero() {
        let v: Vec3<f64> = Vec3::zero();
        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 0.0);
        assert_eq!(v.z, 0.0);
    }

    #[test]
    fn test_one() {
        let v: Vec3<f64> = Vec3::one();
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 1.0);
        assert_eq!(v.z, 1.0);
    }

    #[test]
    fn test_length() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.length(), 14.0_f32.sqrt());
    }

    #[test]
    fn test_length_squared() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.length_squared(), 14.0);
    }

    #[test]
    fn test_dot() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.0, 3.0, 4.0);
        assert_eq!(v1.dot(v2), 20.0);
    }

    #[test]
    fn test_cross() {
        let v1 = Vec3::new(2.0, 3.0, 4.0);
        let v2 = Vec3::new(5.0, 6.0, 7.0);
        let res = v1.cross(v2);
        assert_eq!(res.x, -3.0);
        assert_eq!(res.y, 6.0);
        assert_eq!(res.z, -3.0);
    }

    #[test]
    fn test_normalized() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let norm = v.normalized();
        assert_eq!(norm.length(), 1.0);
    }

    #[test]
    fn test_add() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let res = v1 + v2;
        assert_eq!(res.x, 5.0);
        assert_eq!(res.y, 7.0);
        assert_eq!(res.z, 9.0);

        let v = Vec3::new(1.0, 2.0, 3.0);
        let res = v + 1.0;
        assert_eq!(res.x, 2.0);
        assert_eq!(res.y, 3.0);
        assert_eq!(res.z, 4.0);
    }

    #[test]
    fn test_add_assign() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        v1 += v2;
        assert_eq!(v1.x, 5.0);
        assert_eq!(v1.y, 7.0);
        assert_eq!(v1.z, 9.0);

        let mut v = Vec3::new(1.0, 2.0, 3.0);
        v += 1.0;
        assert_eq!(v.x, 2.0);
        assert_eq!(v.y, 3.0);
        assert_eq!(v.z, 4.0);
    }

    #[test]
    fn test_sub() {
        let v1 = Vec3::new(4.0, 5.0, 6.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);
        let res = v1 - v2;
        assert_eq!(res.x, 3.0);
        assert_eq!(res.y, 3.0);
        assert_eq!(res.z, 3.0);

        let v = Vec3::new(1.0, 2.0, 3.0);
        let res = v - 1.0;
        assert_eq!(res.x, 0.0);
        assert_eq!(res.y, 1.0);
        assert_eq!(res.z, 2.0);
    }

    #[test]
    fn test_sub_assign() {
        let mut v1 = Vec3::new(4.0, 5.0, 6.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);
        v1 -= v2;
        assert_eq!(v1.x, 3.0);
        assert_eq!(v1.y, 3.0);
        assert_eq!(v1.z, 3.0);

        let mut v = Vec3::new(1.0, 2.0, 3.0);
        v -= 1.0;
        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 1.0);
        assert_eq!(v.z, 2.0);
    }

    #[test]
    fn test_mul() {
        let v1 = Vec3::new(4.0, 5.0, 6.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);
        let res = v1 * v2;
        assert_eq!(res.x, 4.0);
        assert_eq!(res.y, 10.0);
        assert_eq!(res.z, 18.0);

        let v = Vec3::new(1.0, 2.0, 3.0);
        let res = v * 2.0;
        assert_eq!(res.x, 2.0);
        assert_eq!(res.y, 4.0);
        assert_eq!(res.z, 6.0);
    }

    #[test]
    fn test_mul_assign() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        v1 *= v2;
        assert_eq!(v1.x, 4.0);
        assert_eq!(v1.y, 10.0);
        assert_eq!(v1.z, 18.0);

        let mut v = Vec3::new(1.0, 2.0, 3.0);
        v *= 2.0;
        assert_eq!(v.x, 2.0);
        assert_eq!(v.y, 4.0);
        assert_eq!(v.z, 6.0);
    }

    #[test]
    fn test_neg() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let res = -v;
        assert_eq!(res.x, -1.0);
        assert_eq!(res.y, -2.0);
        assert_eq!(res.z, -3.0);
    }

    #[test]
    fn test_div() {
        let v1 = Vec3::new(4.0, 6.0, 9.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);
        let res = v1 / v2;
        assert_eq!(res.x, 4.0);
        assert_eq!(res.y, 3.0);
        assert_eq!(res.z, 3.0);

        let v = Vec3::new(2.0, 4.0, 6.0);
        let res = v / 2.0;
        assert_eq!(res.x, 1.0);
        assert_eq!(res.y, 2.0);
        assert_eq!(res.z, 3.0);
    }

    #[test]
    fn test_div_assign() {
        let mut v1 = Vec3::new(4.0, 6.0, 9.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);
        v1 /= v2;
        assert_eq!(v1.x, 4.0);
        assert_eq!(v1.y, 3.0);
        assert_eq!(v1.z, 3.0);

        let mut v = Vec3::new(2.0, 4.0, 6.0);
        v /= 2.0;
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn test_index() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v[0], 1.0);
        assert_eq!(v[1], 2.0);
        assert_eq!(v[2], 3.0);
    }

    #[test]
    fn test_index_mut() {
        let mut v = Vec3::new(0.0, 0.0, 0.0);

        v[0] = 1.0;
        v[1] = 2.0;
        v[2] = 3.0;

        assert_eq!(v[0], 1.0);
        assert_eq!(v[1], 2.0);
        assert_eq!(v[2], 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }
}
