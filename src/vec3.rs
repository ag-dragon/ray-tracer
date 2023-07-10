use std::ops::{{
    Add, AddAssign,
    Div, DivAssign,
    Index, IndexMut,
    Mul, MulAssign, Neg,
    Sub, SubAssign,
}};

trait Sqrt {
    fn sqrt(self) -> Self;
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
