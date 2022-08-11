use std::{ops::{Add, MulAssign, AddAssign, SubAssign, Sub, Neg}, fmt::Display};
use crate::glmath::*;

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Quat<T: PartialOrd + Copy> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T
}

impl
    <T: PartialOrd + Copy + Vectorable<T> + 
        std::ops::Add<Output = T> +
        std::ops::AddAssign<T> +
        std::ops::Sub<Output = T> +
        std::ops::Mul<Output = T> + 
        std::ops::Div<Output = T> + 
        Neg<Output = T> +
        std::ops::DivAssign<T>> 
        StandardQuat<T> for Quat<T> 
{
    fn from_euler_angles(euler: Vec3<T>) -> Quat<T> {
        let yaw = euler.z;
        let pitch = euler.y;
        let roll = euler.x;

        let cy = (yaw * T::HALF).cos();
        let sy = (yaw * T::HALF).sin();
        let cp = (pitch * T::HALF).cos();
        let sp = (pitch * T::HALF).sin();
        let cr = (roll * T::HALF).cos();
        let sr = (roll * T::HALF).sin();

        let mut result = Quat::<T> {
            w: cr * cp * cy + sr * sp * sy,
            x: sr * cp * cy - cr * sp * sy,
            y: cr * sp * cy + sr * cp * sy,
            z: cr * cp * sy - sr * sp * cy
        };

        result.normalize();
        result
    }

    /// The inverse divides each component by its squared len negating
    /// each term except x.
    fn invert(&mut self) {
        let mag = self.length_sq();

        self.x = self.x / mag;
        self.y = -self.y / mag;
        self.z = -self.z / mag;
        self.w = -self.w / mag;
    }

    /// Computes the squared length of the vector2.
    fn length_sq(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    fn from_axis_angle(axis: Vec3<T>, angle: T) -> Quat<T> {
        let s = (angle / T::TWO).sin();

        Quat::<T> {
            x: axis.x * s,
            y: axis.y * s,
            z: axis.z * s,
            w: (angle / T::TWO).cos()
        }
    }

    fn to_matrix(&self) -> Mat44<T> {
        let mut matrix = Mat44::<T>::ZERO;

        let xy = self.x * self.y;
        let xz = self.x * self.z;
        let xw = self.x * self.w;
        let yz = self.y * self.z;
        let yw = self.y * self.w;
        let zw = self.z * self.w;
        let x_sq = self.x * self.x;
        let y_sq = self.y * self.y;
        let z_sq = self.z * self.z;

        matrix.data[0][0] = T::ONE - T::TWO * (y_sq + z_sq);
        matrix.data[0][1] = T::TWO * (xy - zw);
        matrix.data[0][2] = T::TWO * (xz + yw);
        
        matrix.data[1][0] = T::TWO * (xy + zw);
        matrix.data[1][1] = T::ONE - T::TWO * (x_sq + z_sq);
        matrix.data[1][2] = T::TWO * (yz - xw);

        matrix.data[2][0] = T::TWO * (xz - yw);
        matrix.data[2][1] = T::TWO * (yz + xw);
        matrix.data[2][2] = T::ONE - T::TWO * (x_sq + y_sq);

        matrix.data[3][3] = T::ONE;

        return matrix;
    }

    fn to_euler(&self) -> Vec3<T> {
        let yaw;
        let mut pitch;
        let roll;

        // Roll x
        let sinr_cosp = T::TWO * (self.w * self.x + self.y * self.z);
        let cosr_cosp = T::ONE - T::TWO * (self.x * self.x + self.y * self.y);
        roll = T::atan2(sinr_cosp, cosr_cosp);

        // Pitch (y-axis rotation)
        let sinp = T::TWO * (self.w * self.y - self.z * self.x);

        if sinp >= T::ONE || sinp <= -T::ONE {
            pitch = T::PI / T::TWO;
        
            if sinp < T::ZERO {
                pitch = -pitch;
            }
        }
        else {
            pitch = sinp.asin();
        }

        // Yaw (z-axis rotation)
        let siny_cosp = T::TWO * (self.w * self.z + self.x * self.y);
        let cosy_cosp = T::ONE - T::TWO * (self.y * self.y + self.z * self.z);
        yaw = T::atan2(siny_cosp, cosy_cosp);

        Vec3::<T> {
            x: roll,
            y: pitch,
            z: yaw
        }
    }

    fn rotate(&mut self, axis: Vec3<T>, angle: T) {
        let mut matrix = self.to_matrix();
        matrix.rotate(axis, angle);

        let trace = matrix.data[0][0] + matrix.data[1][1] + matrix.data[2][2]; 

        if trace > T::ZERO {
            let s = T::HALF / (trace + T::ONE).sqrt();
            self.w = T::QUARTER / s;
            self.x = (matrix.data[2][1] - matrix.data[1][2]) * s;
            self.y = (matrix.data[0][2] - matrix.data[2][0]) * s;
            self.z = (matrix.data[1][0] - matrix.data[0][1]) * s;
        } else {
            if matrix.data[0][0] > matrix.data[1][1] && matrix.data[0][0] > matrix.data[2][2] {
                let s = T::TWO * (T::ONE + matrix.data[0][0] - matrix.data[1][1] - matrix.data[2][2]).sqrt();
                self.w = (matrix.data[2][1] - matrix.data[1][2]) / s;
                self.x = T::QUARTER * s;
                self.y = (matrix.data[0][1] + matrix.data[1][0]) / s;
                self.z = (matrix.data[0][2] + matrix.data[2][0]) / s;
            } else if matrix.data[1][1] > matrix.data[2][2] {
                let s = T::TWO * (T::ONE + matrix.data[1][1] - matrix.data[0][0] - matrix.data[2][2]).sqrt();
                self.w = (matrix.data[0][2] - matrix.data[2][0]) / s;
                self.x = (matrix.data[0][1] + matrix.data[1][0]) / s;
                self.y = T::QUARTER * s;
                self.z = (matrix.data[1][2] + matrix.data[2][1]) / s;
            }
            else {
                let s = T::TWO * (T::ONE + matrix.data[2][2] - matrix.data[0][0] - matrix.data[1][1]).sqrt();
                self.w = (matrix.data[1][0] - matrix.data[0][1]) / s;
                self.x = (matrix.data[0][2] + matrix.data[2][0]) / s;
                self.y = (matrix.data[1][2] + matrix.data[2][1]) / s;
                self.z = T::QUARTER * s;
            }
        }

        self.normalize();
    }

    fn from_matrix(rot_mat: &Mat44<T>) -> Quat<T> {
        let m00 = rot_mat.data[0][0];
        let m01 = rot_mat.data[0][1]; 
        let m02 = rot_mat.data[0][2];

        let m10 = rot_mat.data[1][0];
        let m11 = rot_mat.data[1][1];
        let m12 = rot_mat.data[1][2];

        let m20 = rot_mat.data[2][0]; 
        let m21 = rot_mat.data[2][1];
        let m22 = rot_mat.data[2][2];

        let mut s;
        let tr = m00 + m11 + m22;

        let x: T;
        let y: T;
        let z: T;
        let w: T;

        if tr >= T::ZERO
        {
            s = (tr + T::ONE).sqrt();
            w = s * T::HALF;
            s = T::HALF / s;

            x = (m21 - m12) * s;
            y = (m02 - m20) * s;
            z = (m10 - m01) * s;
        }
        else
        {
            let max = T::max(T::max(m00, m11), m22);

            if max == m00
            {
                s = (m00 - (m11 + m22) + T::ONE).sqrt();
                x = s * T::HALF;
                s = T::HALF / s;
                y = (m01 + m10) * s;
                z = (m20 + m02) * s;
                w = (m21 - m12) * s;
            }
            else if max == m11
            {
                s = (m11 - (m22 + m00) + T::ONE).sqrt();
                y = s * T::HALF;
                s = T::HALF / s;
                z = (m12 + m21) * s;
                x = (m01 + m10) * s;
                w = (m02 - m20) * s;
            }
            else
            {
                s = (m22 - (m00 + m11) + T::ONE).sqrt();
                z = s * T::HALF;
                s = T::HALF / s;
                x = (m20 + m02) * s;
                y = (m12 + m21) * s;
                w = (m10 - m01) * s;
            }
        }

        Quat::<T> { 
            x, y, z, w
        }
    }

    fn slerp(a: Quat<T>, b: Quat<T>, blend: T) -> Quat<T> {
        let dot = a.w * b.w + a.x * b.x + a.y * b.y + a.z * b.z;
        let blend_i = T::ONE - blend;

        let mut result = Quat::<T>::ZERO;

        if dot < T::ZERO
        {
            result.w = blend_i * a.w + blend * -b.w;
            result.x = blend_i * a.x + blend * -b.x;
            result.y = blend_i * a.y + blend * -b.y;
            result.z = blend_i * a.z + blend * -b.z;
        }
        else
        {
            result.w = blend_i * a.w + blend * b.w;
            result.x = blend_i * a.x + blend * b.x;
            result.y = blend_i * a.y + blend * b.y;
            result.z = blend_i * a.z + blend * b.z;
        }

        result.normalize();
        result
    }

    fn forward(&self) -> Vec3<T> {
        *self * -Vec3::<T>::Z
    }

    fn up(&self) -> Vec3<T> {
        *self * Vec3::<T>::Y
    }

    fn right(&self) -> Vec3<T> {
        *self * Vec3::<T>::X
    }

    fn look_rotation(&mut self, f: Vec3<T>, u: Vec3<T>) {
        let forward = f.get_normalized();
        let up = u.get_normalized();

        let right = forward % up;

        let rot = Mat44::<T>::from_axes(
            Vec4::<T>::new(right.x, right.y, right.z, T::ZERO),
            Vec4::<T>::new(up.x, up.y, up.z, T::ZERO),
            Vec4::<T>::new(forward.x, forward.y, forward.z, T::ZERO),
            Vec4::<T>::new(T::ZERO, T::ZERO, T::ZERO, T::ONE),
        );

        *self = Quat::<T>::from_matrix(&rot);
        self.normalize();
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
impl<T: Vectorable<T> + PartialOrd + Copy + Mul<Output = T> +
    std::ops::Add<Output = T> +
    std::ops::AddAssign<T> +
    std::ops::Sub<Output = T> +
    std::ops::Mul<Output = T> + 
    std::ops::Div<Output = T> + 
    Neg<Output = T>> Mul<Vec3<T>> for Quat<T> 
    where Self: StandardQuat<T>
{
    type Output = Vec3<T>;

    fn mul(self, rhs: Vec3<T>) -> Self::Output {
        let trans = self.to_matrix();
        let v = trans * Vec4::<T>::new(rhs.x, rhs.y, rhs.z, T::ZERO);

        Vec3::<T> {
            x: v.x,
            y: v.y,
            z: v.z
        }
    }
}

// Transform Vec4.
impl<T: Vectorable<T> + PartialOrd + Copy + Mul<Output = T> + 
        std::ops::Add<Output = T> +
        std::ops::AddAssign<T> +
        std::ops::Sub<Output = T> +
        std::ops::Mul<Output = T> + 
        std::ops::Div<Output = T> + 
        StandardMat44<T> +
        Neg<Output = T>>
    Mul<Vec4<T>> for Quat<T> 
    where Self: StandardQuat<T>
{
    type Output = Vec4<T>;

    fn mul(self, rhs: Vec4<T>) -> Self::Output {
        let trans = self.to_matrix();
        trans * rhs
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