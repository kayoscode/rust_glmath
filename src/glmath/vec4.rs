use std::{ops::{Add, MulAssign, AddAssign, SubAssign, Sub, Neg}, fmt::Display};
use crate::glmath::*;

#[derive(Debug, Copy, Clone, Default)]
pub struct Vec4<T: PartialOrd + Copy> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T
}

impl
    <T: PartialOrd + Copy + Vectorable<T> + 
        std::ops::Mul<Output = T> + 
        std::ops::Add<Output = T> +
        std::ops::Div<Output = T> + 
        std::ops::DivAssign<T>> 
        StandardVec<T> for Vec4<T> 
{
    /// Computes the squared length of the vector2.
    fn length_sq(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }
}

impl<T: PartialOrd + Copy + Display> Display for Vec4<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}, {}, {}]", self.x, self.y, self.z, self.w)
    }
}

impl<T: PartialOrd + Copy> Vec4<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Vec4<T> {
        Vec4::<T> { x, y, z, w }
    }
}

impl<T: PartialOrd + Copy + Vectorable<T>> PartialEq for Vec4<T> 
    where Vec4<T>: StandardVec<T>,
    T: Mul<Output = T> + Div<Output = T>
{
    fn eq(&self, other: &Self) -> bool {
        self.length() == other.length()
    }
}

impl<T: PartialOrd + Copy + Vectorable<T>> PartialOrd for Vec4<T>
    where Vec4<T>: StandardVec<T>,
    T: Mul<Output = T> + Div<Output = T>
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let len = self.length();
        let other_len = other.length();

        len.partial_cmp(&other_len)
    }
}

impl<T: PartialOrd + Copy + Neg<Output = T>> Neg for Vec4<T> {
    type Output = Vec4<T>;

    fn neg(self) -> Self::Output {
        Vec4::<T> {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w
        }
    }
}

impl<T: PartialOrd + Copy + Add<Output = T>> Add<Vec4<T>> for Vec4<T> {
    type Output = Vec4<T>;

    fn add(self, rhs: Vec4<T>) -> Self::Output {
        Vec4::<T> {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w
        }
    }
}

impl<T: PartialOrd + Copy + AddAssign> AddAssign for Vec4<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}

impl<T: PartialOrd + Copy + Sub<Output = T>> Sub<Vec4<T>> for Vec4<T> {
    type Output = Vec4<T>;

    fn sub(self, rhs: Vec4<T>) -> Self::Output {
        Vec4::<T> {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w
        }
    }
}

impl<T: PartialOrd + Copy + SubAssign> SubAssign for Vec4<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self.w -= rhs.w;
    }
}

// Scalar multiples.
impl<T: PartialOrd + Copy + Mul<Output = T>> Mul<T> for Vec4<T> {
    type Output = Vec4<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vec4::<T> {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs
        }
    }
}

impl<T: PartialOrd + Copy + MulAssign> MulAssign<T> for Vec4<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self.w *= rhs;
    }
}

// Scalar divides.
impl<T: PartialOrd + Copy + Div<Output = T>> Div<T> for Vec4<T> {
    type Output = Vec4<T>;

    fn div(self, rhs: T) -> Self::Output {
        Vec4::<T> {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs
        }
    }
}

impl<T: PartialOrd + Copy + DivAssign> DivAssign<T> for Vec4<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
        self.w /= rhs;
    }
}

// Dot product.
impl<T: PartialOrd + Copy + 
    Mul<Output = T> +
    Add<Output = T>> Mul<Vec4<T>> for Vec4<T> 
{
    type Output = T;

    fn mul(self, rhs: Vec4<T>) -> Self::Output {
        self.x * rhs.x + 
            self.y * rhs.y + 
            self.z * rhs.z + 
            self.w * rhs.w
    }
}

impl<T: PartialOrd + Copy> TwoDimVec<T> for Vec4<T> {
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

impl<T: PartialOrd + Copy> ThreeDimVec<T> for Vec4<T> {
    fn z(&self) -> &T {
        &self.z
    }

    fn xyz(&self) -> Vec3<T> {
        Vec3::<T>::new(self.x, self.y, self.z)
    }

    fn yxz(&self) -> Vec3<T> {
        Vec3::<T>::new(self.y, self.x, self.z)
    }

    fn zxy(&self) -> Vec3<T> {
        Vec3::<T>::new(self.z, self.x, self.y)
    }

    fn xzy(&self) -> Vec3<T> {
        Vec3::<T>::new(self.x, self.z, self.y)
    }

    fn yzx(&self) -> Vec3<T> {
        Vec3::<T>::new(self.y, self.z, self.x)
    }

    fn zyx(&self) -> Vec3<T> {
        Vec3::<T>::new(self.z, self.y, self.x)
    }
}

impl<T: PartialOrd + Copy> FourDimVec<T> for Vec4<T> {
    fn w(&self) -> &T {
        return &self.w;
    }

    fn xyzw(&self) -> Vec4<T> {
        self.clone()
    }

    fn yxzw(&self) -> Vec4<T> {
        Vec4::<T> { x: self.y, y: self.x, z: self.z, w: self.w }
    }

    fn zxyw(&self) -> Vec4<T> {
        Vec4::<T> { x: self.z, y: self.x, z: self.y, w: self.w }
    }

    fn xzyw(&self) -> Vec4<T> {
        Vec4::<T> { x: self.x, y: self.z, z: self.y, w: self.w }
    }

    fn yzxw(&self) -> Vec4<T> {
        Vec4::<T> { x: self.y, y: self.z, z: self.x, w: self.w }
    }

    fn zyxw(&self) -> Vec4<T> {
        Vec4::<T> { x: self.z, y: self.y, z: self.x, w: self.w }
    }

    fn zywx(&self) -> Vec4<T> {
        Vec4::<T> { x: self.z, y: self.y, z: self.w, w: self.x }
    }

    fn yzwx(&self) -> Vec4<T> {
        Vec4::<T> { x: self.y, y: self.z, z: self.w, w: self.x }
    }

    fn wzyx(&self) -> Vec4<T> {
        Vec4::<T> { x: self.w, y: self.z, z: self.y, w: self.x }
    }

    fn zwyx(&self) -> Vec4<T> {
        Vec4::<T> { x: self.w, y: self.z, z: self.y, w: self.x }
    }

    fn ywzx(&self) -> Vec4<T> {
        Vec4::<T> { x: self.y, y: self.w, z: self.z, w: self.x }
    }

    fn wyzx(&self) -> Vec4<T> {
        Vec4::<T> { x: self.w, y: self.y, z: self.z, w: self.x }
    }

    fn wxzy(&self) -> Vec4<T> {
        Vec4::<T> { x: self.w, y: self.x, z: self.z, w: self.y }
    }

    fn xwzy(&self) -> Vec4<T> {
        Vec4::<T> { x: self.x, y: self.w, z: self.z, w: self.y }
    }

    fn zwxy(&self) -> Vec4<T> {
        Vec4::<T> { x: self.z, y: self.w, z: self.x, w: self.y }
    }

    fn wzxy(&self) -> Vec4<T> {
        Vec4::<T> { x: self.w, y: self.z, z: self.x, w: self.y }
    }

    fn xzwy(&self) -> Vec4<T> {
        Vec4::<T> { x: self.x, y: self.z, z: self.w, w: self.y }
    }

    fn zxwy(&self) -> Vec4<T> {
        Vec4::<T> { x: self.z, y: self.x, z: self.w, w: self.y }
    }

    fn yxwz(&self) -> Vec4<T> {
        Vec4::<T> { x: self.y, y: self.x, z: self.w, w: self.z }
    }

    fn xywz(&self) -> Vec4<T> {
        Vec4::<T> { x: self.x, y: self.y, z: self.w, w: self.z }
    }

    fn wyxz(&self) -> Vec4<T> {
        Vec4::<T> { x: self.w, y: self.y, z: self.x, w: self.z }
    }

    fn ywxz(&self) -> Vec4<T> {
        Vec4::<T> { x: self.y, y: self.w, z: self.x, w: self.z }
    }

    fn xwyz(&self) -> Vec4<T> {
        Vec4::<T> { x: self.x, y: self.w, z: self.y, w: self.z }
    }

    fn wxyz(&self) -> Vec4<T> {
        Vec4::<T> { x: self.w, y: self.x, z: self.y, w: self.z }
    }
}