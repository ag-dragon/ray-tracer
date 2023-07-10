use std::ops::{{
    Add, AddAssign,
    Div, DivAssign,
    Index, IndexMut,
    Mul, MulAssign, Neg,
    Sub, SubAssign,
}};

pub trait Sqrt {
    fn sqrt(self) -> Self;
}

impl Sqrt for f64 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}

impl Sqrt for f32 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T> {
    #[inline(always)]
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    #[inline(always)]
    pub fn length(self) -> T where
        T: Sqrt + Copy + Add<Output = T> + Mul<Output = T> {
        self.length_squared().sqrt()
    }

    #[inline(always)]
    pub fn length_squared(self) -> T where 
        T: Copy + Add<Output = T> + Mul<Output = T> {
        self.x*self.x + self.y*self.y + self.z*self.z
    }

    #[inline(always)]
    pub fn dot(self, rhs: Self) -> T where
        T: Copy + Add<Output = T> + Mul<Output = T> {
        self.x*rhs.x + self.y*rhs.y + self.z*rhs.z
    }

    #[inline(always)]
    pub fn cross(self, rhs: Self) -> Self where
        T: Copy + Sub<Output = T> + Mul<Output = T> {
        Self {
            x: self.y*rhs.z - self.z*rhs.y,
            y: self.z*rhs.x - self.x*rhs.z,
            z: self.x*rhs.y - self.y*rhs.x,
        }
    }

    #[inline(always)]
    pub fn normalized(self) -> Self where
        T: Sqrt + Copy + Add<Output = T> + Mul<Output = T> + Div<Output = T> {
        self / self.length()
    }
}

impl<T> Add<Vec3<T>> for Vec3<T> where
    T: Add<Output = T> {
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

impl<T> Add<T> for Vec3<T> where
    T: Copy + Add<Output = T> {
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

impl<T> AddAssign<Vec3<T>> for Vec3<T> where 
    T: Copy + Add<Output = T> {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<T> AddAssign<T> for Vec3<T> where 
    T: Copy + Add<Output = T> {
    #[inline(always)]
    fn add_assign(&mut self, rhs: T) {
        *self = *self + rhs;
    }
}

impl<T> Sub<Vec3<T>> for Vec3<T> where
    T: Sub<Output = T> {
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

impl<T> Sub<T> for Vec3<T> where
    T: Copy + Sub<Output = T> {
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

impl<T> SubAssign<Vec3<T>> for Vec3<T> where
    T: Copy + Sub<Output = T> {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<T> SubAssign<T> for Vec3<T> where
    T: Copy + Sub<Output = T> {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: T) {
        *self = *self - rhs;
    }
}

impl<T> Mul<Vec3<T>> for Vec3<T> where
    T: Mul<Output = T> {
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

impl<T> Mul<T> for Vec3<T> where
    T: Copy + Mul<Output = T> {
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

impl<T> MulAssign<Vec3<T>> for Vec3<T> where
    T: Copy + Mul<Output = T> {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<T> MulAssign<T> for Vec3<T> where
    T: Copy + Mul<Output = T> {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs;
    }
}

impl<T> Neg for Vec3<T> where
    T: Neg<Output = T> {
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

impl<T> Div<Vec3<T>> for Vec3<T> where
    T: Div<Output = T> {
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

impl<T> Div<T> for Vec3<T> where
    T: Copy + Div<Output = T> {
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

impl<T> DivAssign<Vec3<T>> for Vec3<T> where
    T: Copy + Div<Output = T> {
    #[inline(always)]
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl<T> DivAssign<T> for Vec3<T> where
    T: Copy + Div<Output = T> {
    #[inline(always)]
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs;
    }
}

impl<T> Index<usize> for Vec3<T> {
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

impl<T> IndexMut<usize> for Vec3<T> {
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
