use std::{ops::{Add, MulAssign, AddAssign, SubAssign, Sub, Neg}, fmt::Display};
use crate::glmath::*;

#[derive(Debug, Copy, Clone, Default)]
pub struct Quat<T: PartialOrd + Copy> {
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
        StandardQuat<T> for Quat<T> 
{
    /// Computes the squared length of the vector2.
    fn length_sq(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }
}

impl<T: PartialOrd + Copy + Display> Display for Quat<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}, {}, {}]", self.x, self.y, self.z, self.w)
    }
}

impl<T: PartialOrd + Copy + Vectorable<T>> Quat<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Quat<T> {
        Quat::<T> { x, y, z, w }
    }

    pub fn from_euler_angles(axis: Vec3<T>) -> Quat<T> {
        let _yaw = axis.z;
        let _pitch = axis.y;
        let _roll = axis.x;

        //T cy = (T)cos(yaw * 0.5);
        //T sy = (T)sin(yaw * 0.5);
        //T cp = (T)cos(pitch * 0.5);
        //T sp = (T)sin(pitch * 0.5);
        //T cr = (T)cos(roll * 0.5);
        //T sr = (T)sin(roll * 0.5);

        //this->w = cr * cp * cy + sr * sp * sy;
        //this->x = sr * cp * cy - cr * sp * sy;
        //this->y = cr * sp * cy + sr * cp * sy;
        //this->z = cr * cp * sy - sr * sp * cy;

        //normalize();

        Quat::<T> {
            x: axis.x,
            y: axis.y,
            z: axis.z,
            w: axis.x
        }
    }

    pub const ZERO: Quat<T> = Quat::<T> {
        x: T::ZERO,
        y: T::ZERO,
        z: T::ZERO,
        w: T::ZERO
    };

    pub const IDENTITY: Quat<T> = Quat::<T> {
        x: T::ZERO,
        y: T::ZERO,
        z: T::ZERO,
        w: T::ONE
    };

    pub fn set_identity(&mut self) {
        *self = Self::IDENTITY;
    }
}

impl<T: PartialOrd + Copy + Neg<Output = T>> Neg for Quat<T> {
    type Output = Quat<T>;

    fn neg(self) -> Self::Output {
        Quat::<T> {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w
        }
    }
}

impl<T: PartialOrd + Copy + Add<Output = T>> Add<Quat<T>> for Quat<T> {
    type Output = Quat<T>;

    fn add(self, rhs: Quat<T>) -> Self::Output {
        Quat::<T> {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w
        }
    }
}

impl<T: PartialOrd + Copy + AddAssign> AddAssign for Quat<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}

impl<T: PartialOrd + Copy + Sub<Output = T>> Sub<Quat<T>> for Quat<T> {
    type Output = Quat<T>;

    fn sub(self, rhs: Quat<T>) -> Self::Output {
        Quat::<T> {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w
        }
    }
}

impl<T: PartialOrd + Copy + SubAssign> SubAssign for Quat<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self.w -= rhs.w;
    }
}

// Scalar multiples.
impl<T: PartialOrd + Copy + Mul<Output = T>> Mul<T> for Quat<T> {
    type Output = Quat<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Quat::<T> {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs
        }
    }
}

impl<T: PartialOrd + Copy + MulAssign> MulAssign<T> for Quat<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self.w *= rhs;
    }
}

// Scalar divides.
impl<T: PartialOrd + Copy + Div<Output = T>> Div<T> for Quat<T> {
    type Output = Quat<T>;

    fn div(self, rhs: T) -> Self::Output {
        Quat::<T> {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs
        }
    }
}

impl<T: PartialOrd + Copy + DivAssign> DivAssign<T> for Quat<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
        self.w /= rhs;
    }
}

// Quat * Quat
impl<T: PartialOrd + Copy + Mul<Output = T> +
    Add<Output = T> + Sub<Output = T>> Mul<Quat<T>> for Quat<T> 
{
    type Output = Quat<T>;

    fn mul(self, rhs: Quat<T>) -> Self::Output {
        Quat::<T> {
            x: self.x * rhs.w + self.w * rhs.x + self.y * rhs.z - self.z * rhs.y, 
            y: self.y * rhs.w + self.w * rhs.y + self.z * rhs.x - self.x * rhs.z, 
            z: self.z * rhs.w + self.w * rhs.z + self.x * rhs.y - self.y * rhs.x,
            w: self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z
        }
    }
}

impl<T: PartialOrd + Copy + Mul<Output = T> +
    Add<Output = T> + Sub<Output = T>> MulAssign for Quat<T> 
{
    fn mul_assign(&mut self, rhs: Self) {
        self.x = self.x * rhs.w + self.w * rhs.x + self.y * rhs.z - self.z * rhs.y;
        self.y = self.y * rhs.w + self.w * rhs.y + self.z * rhs.x - self.x * rhs.z;
        self.z = self.z * rhs.w + self.w * rhs.z + self.x * rhs.y - self.y * rhs.x;
        self.w = self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z;
    }
}

// Transform Vec3.
impl<T: Vectorable<T> + PartialOrd + Copy + Mul<Output = T>> Mul<Vec3<T>> for Quat<T> {
    type Output = Vec3<T>;

    // TODO: transform into a matrix and perform math.
    fn mul(self, _rhs: Vec3<T>) -> Self::Output {
        Vec3::new(self.x, self.y, self.z)
    }
}

// Transform Vec4.
impl<T: Vectorable<T> + PartialOrd + Copy + Mul<Output = T>> Mul<Vec4<T>> for Quat<T> {
    type Output = Vec4<T>;

    // TODO: transform into a matrix and perform math.
    fn mul(self, _rhs: Vec4<T>) -> Self::Output {
        Vec4::new(self.x, self.y, self.z, self.w)
    }
}

impl<T: PartialOrd + Copy> QuaternionSwizzle<T> for Quat<T> {
    fn w(&self) -> T {
        self.w
    }

    fn xyzw(&self) -> Quat<T> {
        self.clone()
    }

    fn yxzw(&self) -> Quat<T> {
        Quat::<T> { x: self.y, y: self.x, z: self.z, w: self.w }
    }

    fn zxyw(&self) -> Quat<T> {
        Quat::<T> { x: self.z, y: self.x, z: self.y, w: self.w }
    }

    fn xzyw(&self) -> Quat<T> {
        Quat::<T> { x: self.x, y: self.z, z: self.y, w: self.w }
    }

    fn yzxw(&self) -> Quat<T> {
        Quat::<T> { x: self.y, y: self.z, z: self.x, w: self.w }
    }

    fn zyxw(&self) -> Quat<T> {
        Quat::<T> { x: self.z, y: self.y, z: self.x, w: self.w }
    }

    fn zywx(&self) -> Quat<T> {
        Quat::<T> { x: self.z, y: self.y, z: self.w, w: self.x }
    }

    fn yzwx(&self) -> Quat<T> {
        Quat::<T> { x: self.y, y: self.z, z: self.w, w: self.x }
    }

    fn wzyx(&self) -> Quat<T> {
        Quat::<T> { x: self.w, y: self.z, z: self.y, w: self.x }
    }

    fn zwyx(&self) -> Quat<T> {
        Quat::<T> { x: self.w, y: self.z, z: self.y, w: self.x }
    }

    fn ywzx(&self) -> Quat<T> {
        Quat::<T> { x: self.y, y: self.w, z: self.z, w: self.x }
    }

    fn wyzx(&self) -> Quat<T> {
        Quat::<T> { x: self.w, y: self.y, z: self.z, w: self.x }
    }

    fn wxzy(&self) -> Quat<T> {
        Quat::<T> { x: self.w, y: self.x, z: self.z, w: self.y }
    }

    fn xwzy(&self) -> Quat<T> {
        Quat::<T> { x: self.x, y: self.w, z: self.z, w: self.y }
    }

    fn zwxy(&self) -> Quat<T> {
        Quat::<T> { x: self.z, y: self.w, z: self.x, w: self.y }
    }

    fn wzxy(&self) -> Quat<T> {
        Quat::<T> { x: self.w, y: self.z, z: self.x, w: self.y }
    }

    fn xzwy(&self) -> Quat<T> {
        Quat::<T> { x: self.x, y: self.z, z: self.w, w: self.y }
    }

    fn zxwy(&self) -> Quat<T> {
        Quat::<T> { x: self.z, y: self.x, z: self.w, w: self.y }
    }

    fn yxwz(&self) -> Quat<T> {
        Quat::<T> { x: self.y, y: self.x, z: self.w, w: self.z }
    }

    fn xywz(&self) -> Quat<T> {
        Quat::<T> { x: self.x, y: self.y, z: self.w, w: self.z }
    }

    fn wyxz(&self) -> Quat<T> {
        Quat::<T> { x: self.w, y: self.y, z: self.x, w: self.z }
    }

    fn ywxz(&self) -> Quat<T> {
        Quat::<T> { x: self.y, y: self.w, z: self.x, w: self.z }
    }

    fn xwyz(&self) -> Quat<T> {
        Quat::<T> { x: self.x, y: self.w, z: self.y, w: self.z }
    }

    fn wxyz(&self) -> Quat<T> {
        Quat::<T> { x: self.w, y: self.x, z: self.y, w: self.z }
    }
}