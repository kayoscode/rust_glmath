use std::{ops::{Add, AddAssign, Neg, Sub, SubAssign, Mul, MulAssign, Rem}, fmt::Display};

use crate::glmath::*;

#[derive(Debug, Copy, Clone, Default)]
pub struct Vec3<T: PartialOrd + Copy> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T: PartialOrd + Copy + Display> Display for Vec3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

impl
    <T: PartialOrd + Copy + Vectorable<T> + 
        std::ops::Mul<Output = T> + 
        std::ops::Add<Output = T> + 
        std::ops::Div<Output = T> + 
        std::ops::DivAssign<T>> 
        StandardVec<T> for Vec3<T> 
{
    /// Computes the squared length of the vector2.
    fn length_sq(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl<T: PartialOrd + Copy> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3::<T> { x, y, z }
    }
}

impl<T: PartialOrd + Copy + Vectorable<T>> PartialEq for Vec3<T> 
    where Vec3<T>: StandardVec<T>,
    T: Mul<Output = T> + Div<Output = T>
{
    fn eq(&self, other: &Self) -> bool {
        self.length() == other.length()
    }
}

impl<T: PartialOrd + Copy + Vectorable<T>> PartialOrd for Vec3<T>
    where Vec3<T>: StandardVec<T>,
    T: Mul<Output = T> + Div<Output = T>
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let len = self.length();
        let other_len = other.length();

        len.partial_cmp(&other_len)
    }
}

impl<T: PartialOrd + Copy + std::ops::Neg<Output = T>> Neg for Vec3<T> {
    type Output = Vec3<T>;

    fn neg(self) -> Self::Output {
        Vec3::<T> {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl<T: PartialOrd + Copy + Add<Output = T>> Add<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, rhs: Vec3<T>) -> Self::Output {
        Vec3::<T> {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl<T: PartialOrd + Copy + AddAssign> AddAssign for Vec3<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T: PartialOrd + Copy + Sub<Output = T>> Sub<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, rhs: Vec3<T>) -> Self::Output {
        Vec3::<T> {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl<T: PartialOrd + Copy + SubAssign> SubAssign for Vec3<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

// Scalar multiples.
impl<T: PartialOrd + Copy + Mul<Output = T>> Mul<T> for Vec3<T> {
    type Output = Vec3<T>;

    fn mul(self, rhs: T) -> Self::Output {
         Vec3::<T> {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }       
    }
}

impl<T: PartialOrd + Copy + MulAssign> MulAssign<T> for Vec3<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

// Scalar divides.
impl<T: PartialOrd + Copy + Div<Output = T>> Div<T> for Vec3<T> {
    type Output = Vec3<T>;

    fn div(self, rhs: T) -> Self::Output {
         Vec3::<T> {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs
        }       
    }
}

impl<T: PartialOrd + Copy + DivAssign> DivAssign<T> for Vec3<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

// Dot product.
impl<T: PartialOrd + Copy + 
    Mul<Output = T> + 
    Add<Output = T>> Mul<Vec3<T>> for Vec3<T> 
{
    type Output = T;

    fn mul(self, rhs: Vec3<T>) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

// Cross product.
impl<T: PartialOrd + Copy + Mul<Output = T> + Sub<Output = T>> Rem<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;

    fn rem(self, rhs: Vec3<T>) -> Self::Output {
        Vec3::<T> {
            x: self.y * rhs.z - self.z * rhs.y, 
            y: self.z * rhs.x - self.x * rhs.z, 
            z: self.x * rhs.y - self.y * rhs.x
        }
    }
}

impl<T: PartialOrd + Copy> TwoDimVec<T> for Vec3<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }

    fn xy(&self) -> Vec2<T> {
        Vec2::<T>::new(self.x, self.y)
    }

    fn yx(&self) -> Vec2<T> {
        Vec2::<T>::new(self.y, self.x)
    }
}

impl<T: PartialOrd + Copy> ThreeDimVec<T> for Vec3<T> {
    fn z(&self) -> &T {
        &self.z
    }

    fn xyz(&self) -> Vec3<T> {
        self.clone()
    }

    fn yxz(&self) -> Vec3<T> {
        Vec3::<T> { x: self.y, y: self.x, z: self.z }
    }

    fn zxy(&self) -> Vec3<T> {
        Vec3::<T> { x: self.z, y: self.x, z: self.y }
    }

    fn xzy(&self) -> Vec3<T> {
        Vec3::<T> { x: self.x, y: self.z, z: self.y }
    }

    fn yzx(&self) -> Vec3<T> {
        Vec3::<T> { x: self.y, y: self.z, z: self.x }
    }

    fn zyx(&self) -> Vec3<T> {
        Vec3::<T> { x: self.z, y: self.y, z: self.x }
    }
}