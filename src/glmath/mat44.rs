use std::{fmt::Display, ops::{Add, AddAssign, SubAssign, Sub, MulAssign, Neg}};

use crate::glmath::*;

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Mat44<T: PartialOrd + Copy> {
    pub data: [[T; 4]; 4]
}

impl<T: Vectorable<T> + PartialOrd + Copy> Mat44<T> {
    /// Returns an identity matrix.
    pub fn new() -> Mat44<T> {
        Self::IDENTITY
    }

    /// Constructs a matrix from axis values.
    pub fn from_axes(x: Vec4<T>, y: Vec4<T>, z: Vec4<T>, w: Vec4<T>) -> Mat44<T> {
        Mat44::<T> {
            data: [
                [ x.x, x.y, x.z, x.w ],
                [ y.x, y.y, y.z, y.w ],
                [ z.x, z.y, z.z, z.w ],
                [ w.x, w.y, w.z, w.w ]
            ]
        }
    }

    pub const IDENTITY: Mat44<T> = Mat44::<T> {
        data: [
            [ T::ONE, T::ZERO, T::ZERO, T::ZERO ],
            [ T::ZERO, T::ONE, T::ZERO, T::ZERO ],
            [ T::ZERO, T::ZERO, T::ONE, T::ZERO ],
            [ T::ZERO, T::ZERO, T::ZERO, T::ONE ],
        ]
    };

    pub const ZERO: Mat44<T> = Mat44::<T> {
        data: [
            [ T::ZERO, T::ZERO, T::ZERO, T::ZERO ],
            [ T::ZERO, T::ZERO, T::ZERO, T::ZERO ],
            [ T::ZERO, T::ZERO, T::ZERO, T::ZERO ],
            [ T::ZERO, T::ZERO, T::ZERO, T::ZERO ]
        ]
    };
}

impl<T: PartialOrd + Copy + Display> Display for Mat44<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}, {}]\n[{}, {}, {}]\n[{}, {}, {}]", 
            self.data[0][0], self.data[1][0], self.data[2][0], 
            self.data[0][1], self.data[1][1], self.data[2][1], 
            self.data[0][2], self.data[1][2], self.data[2][2])
    }
}

impl<T: PartialOrd + Copy + Neg<Output = T>> Neg for Mat44<T> {
    type Output = Mat44<T>;

    fn neg(self) -> Self::Output {
        Mat44::<T> {
            data:[
                [ -self.data[0][0], -self.data[0][1], -self.data[0][2], -self.data[0][3] ],
                [ -self.data[1][0], -self.data[1][1], -self.data[1][2], -self.data[1][3] ],
                [ -self.data[2][0], -self.data[2][1], -self.data[2][2], -self.data[2][3] ],
                [ -self.data[3][0], -self.data[3][1], -self.data[3][2], -self.data[3][3] ]
            ]
        }
    }
}

impl<T: PartialOrd + Copy + Add<Output = T>> Add<Mat44<T>> for Mat44<T> {
    type Output = Mat44<T>;

    fn add(self, rhs: Mat44<T>) -> Self::Output {
        Mat44::<T> {
            data: [
                [
                    self.data[0][0] + rhs.data[0][0],
                    self.data[0][1] + rhs.data[0][1],
                    self.data[0][2] + rhs.data[0][2],
                    self.data[0][2] + rhs.data[0][2]
                ],
                [
                    self.data[1][0] + rhs.data[1][0],
                    self.data[1][1] + rhs.data[1][1],
                    self.data[1][2] + rhs.data[1][2],
                    self.data[1][3] + rhs.data[1][3]
                ],
                [
                    self.data[2][0] + rhs.data[2][0],
                    self.data[2][1] + rhs.data[2][1],
                    self.data[2][2] + rhs.data[2][2],
                    self.data[2][3] + rhs.data[2][3]
                ],
                [
                    self.data[3][0] + rhs.data[3][0],
                    self.data[3][1] + rhs.data[3][1],
                    self.data[3][2] + rhs.data[3][2],
                    self.data[3][3] + rhs.data[3][3]
                ]
            ]
        }
    }
}

impl<T: PartialOrd + Copy + Add<Output = T>> AddAssign for Mat44<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.data[0][0] = self.data[0][0] + rhs.data[0][0];
        self.data[0][1] = self.data[0][1] + rhs.data[0][1];
        self.data[0][2] = self.data[0][2] + rhs.data[0][2];
        self.data[0][3] = self.data[0][3] + rhs.data[0][3];
        self.data[1][0] = self.data[1][0] + rhs.data[1][0];
        self.data[1][1] = self.data[1][1] + rhs.data[1][1];
        self.data[1][2] = self.data[1][2] + rhs.data[1][2];
        self.data[1][3] = self.data[1][3] + rhs.data[1][3];
        self.data[2][0] = self.data[2][0] + rhs.data[2][0];
        self.data[2][1] = self.data[2][1] + rhs.data[2][1];
        self.data[2][2] = self.data[2][2] + rhs.data[2][2];
        self.data[2][3] = self.data[2][3] + rhs.data[2][3];
        self.data[3][0] = self.data[3][0] + rhs.data[3][0];
        self.data[3][1] = self.data[3][1] + rhs.data[3][1];
        self.data[3][2] = self.data[3][2] + rhs.data[3][2];
        self.data[3][3] = self.data[3][3] + rhs.data[3][3];
    }
}

impl<T: PartialOrd + Copy + Sub<Output = T>> Sub<Mat44<T>> for Mat44<T> {
    type Output = Mat44<T>;

    fn sub(self, rhs: Mat44<T>) -> Self::Output {
        Mat44::<T> {
            data: [
                [
                    self.data[0][0] - rhs.data[0][0],
                    self.data[0][1] - rhs.data[0][1],
                    self.data[0][2] - rhs.data[0][2],
                    self.data[0][2] - rhs.data[0][2]
                ],
                [
                    self.data[1][0] - rhs.data[1][0],
                    self.data[1][1] - rhs.data[1][1],
                    self.data[1][2] - rhs.data[1][2],
                    self.data[1][3] - rhs.data[1][3]
                ],
                [
                    self.data[2][0] - rhs.data[2][0],
                    self.data[2][1] - rhs.data[2][1],
                    self.data[2][2] - rhs.data[2][2],
                    self.data[2][3] - rhs.data[2][3]
                ],
                [
                    self.data[3][0] - rhs.data[3][0],
                    self.data[3][1] - rhs.data[3][1],
                    self.data[3][2] - rhs.data[3][2],
                    self.data[3][3] - rhs.data[3][3]
                ]
            ]
        }
    }
}

impl<T: PartialOrd + Copy + Sub<Output = T>> SubAssign for Mat44<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.data[0][0] = self.data[0][0] - rhs.data[0][0];
        self.data[0][1] = self.data[0][1] - rhs.data[0][1];
        self.data[0][2] = self.data[0][2] - rhs.data[0][2];
        self.data[0][3] = self.data[0][3] - rhs.data[0][3];
        self.data[1][0] = self.data[1][0] - rhs.data[1][0];
        self.data[1][1] = self.data[1][1] - rhs.data[1][1];
        self.data[1][2] = self.data[1][2] - rhs.data[1][2];
        self.data[1][3] = self.data[1][3] - rhs.data[1][3];
        self.data[2][0] = self.data[2][0] - rhs.data[2][0];
        self.data[2][1] = self.data[2][1] - rhs.data[2][1];
        self.data[2][2] = self.data[2][2] - rhs.data[2][2];
        self.data[2][3] = self.data[2][3] - rhs.data[2][3];
        self.data[3][0] = self.data[3][0] - rhs.data[3][0];
        self.data[3][1] = self.data[3][1] - rhs.data[3][1];
        self.data[3][2] = self.data[3][2] - rhs.data[3][2];
        self.data[3][3] = self.data[3][3] - rhs.data[3][3];
    }
}

impl<T: PartialOrd + Copy + Mul<Output = T> + Add<Output = T>>
    Mul<Mat44<T>> for Mat44<T> 
{
    type Output = Mat44<T>;

    fn mul(self, rhs: Mat44<T>) -> Self::Output {
        let t00 = self.data[0][0] * rhs.data[0][0] + self.data[1][0] * rhs.data[0][1] + self.data[2][0] * rhs.data[0][2] + self.data[3][0] * rhs.data[0][3];
        let t01 = self.data[0][1] * rhs.data[0][0] + self.data[1][1] * rhs.data[0][1] + self.data[2][1] * rhs.data[0][2] + self.data[3][1] * rhs.data[0][3];
        let t02 = self.data[0][2] * rhs.data[0][0] + self.data[1][2] * rhs.data[0][1] + self.data[2][2] * rhs.data[0][2] + self.data[3][2] * rhs.data[0][3];
        let t03 = self.data[0][3] * rhs.data[0][0] + self.data[1][3] * rhs.data[0][1] + self.data[2][3] * rhs.data[0][2] + self.data[3][3] * rhs.data[0][3];
        let t10 = self.data[0][0] * rhs.data[1][0] + self.data[1][0] * rhs.data[1][1] + self.data[2][0] * rhs.data[1][2] + self.data[3][0] * rhs.data[1][3];
        let t11 = self.data[0][1] * rhs.data[1][0] + self.data[1][1] * rhs.data[1][1] + self.data[2][1] * rhs.data[1][2] + self.data[3][1] * rhs.data[1][3];
        let t12 = self.data[0][2] * rhs.data[1][0] + self.data[1][2] * rhs.data[1][1] + self.data[2][2] * rhs.data[1][2] + self.data[3][2] * rhs.data[1][3];
        let t13 = self.data[0][3] * rhs.data[1][0] + self.data[1][3] * rhs.data[1][1] + self.data[2][3] * rhs.data[1][2] + self.data[3][3] * rhs.data[1][3];
        let t20 = self.data[0][0] * rhs.data[2][0] + self.data[1][0] * rhs.data[2][1] + self.data[2][0] * rhs.data[2][2] + self.data[3][0] * rhs.data[2][3];
        let t21 = self.data[0][1] * rhs.data[2][0] + self.data[1][1] * rhs.data[2][1] + self.data[2][1] * rhs.data[2][2] + self.data[3][1] * rhs.data[2][3];
        let t22 = self.data[0][2] * rhs.data[2][0] + self.data[1][2] * rhs.data[2][1] + self.data[2][2] * rhs.data[2][2] + self.data[3][2] * rhs.data[2][3];
        let t23 = self.data[0][3] * rhs.data[2][0] + self.data[1][3] * rhs.data[2][1] + self.data[2][3] * rhs.data[2][2] + self.data[3][3] * rhs.data[2][3];
        let t30 = self.data[0][0] * rhs.data[3][0] + self.data[1][0] * rhs.data[3][1] + self.data[2][0] * rhs.data[3][2] + self.data[3][0] * rhs.data[3][3];
        let t31 = self.data[0][1] * rhs.data[3][0] + self.data[1][1] * rhs.data[3][1] + self.data[2][1] * rhs.data[3][2] + self.data[3][1] * rhs.data[3][3];
        let t32 = self.data[0][2] * rhs.data[3][0] + self.data[1][2] * rhs.data[3][1] + self.data[2][2] * rhs.data[3][2] + self.data[3][2] * rhs.data[3][3];
        let t33 = self.data[0][3] * rhs.data[3][0] + self.data[1][3] * rhs.data[3][1] + self.data[2][3] * rhs.data[3][2] + self.data[3][3] * rhs.data[3][3];

        Mat44::<T> {
            data: [
                [ t00, t01, t02, t03 ],
                [ t10, t11, t12, t13 ],
                [ t20, t21, t22, t23 ],
                [ t30, t31, t32, t33 ]
            ]
        }
    }
}

impl<T: PartialOrd + Copy + Mul<Output = T> + Add<Output = T>>
    MulAssign for Mat44<T> 
{
    fn mul_assign(&mut self, rhs: Self) {
        let t00 = self.data[0][0] * rhs.data[0][0] + self.data[1][0] * rhs.data[0][1] + self.data[2][0] * rhs.data[0][2] + self.data[3][0] * rhs.data[0][3];
        let t01 = self.data[0][1] * rhs.data[0][0] + self.data[1][1] * rhs.data[0][1] + self.data[2][1] * rhs.data[0][2] + self.data[3][1] * rhs.data[0][3];
        let t02 = self.data[0][2] * rhs.data[0][0] + self.data[1][2] * rhs.data[0][1] + self.data[2][2] * rhs.data[0][2] + self.data[3][2] * rhs.data[0][3];
        let t03 = self.data[0][3] * rhs.data[0][0] + self.data[1][3] * rhs.data[0][1] + self.data[2][3] * rhs.data[0][2] + self.data[3][3] * rhs.data[0][3];
        let t10 = self.data[0][0] * rhs.data[1][0] + self.data[1][0] * rhs.data[1][1] + self.data[2][0] * rhs.data[1][2] + self.data[3][0] * rhs.data[1][3];
        let t11 = self.data[0][1] * rhs.data[1][0] + self.data[1][1] * rhs.data[1][1] + self.data[2][1] * rhs.data[1][2] + self.data[3][1] * rhs.data[1][3];
        let t12 = self.data[0][2] * rhs.data[1][0] + self.data[1][2] * rhs.data[1][1] + self.data[2][2] * rhs.data[1][2] + self.data[3][2] * rhs.data[1][3];
        let t13 = self.data[0][3] * rhs.data[1][0] + self.data[1][3] * rhs.data[1][1] + self.data[2][3] * rhs.data[1][2] + self.data[3][3] * rhs.data[1][3];
        let t20 = self.data[0][0] * rhs.data[2][0] + self.data[1][0] * rhs.data[2][1] + self.data[2][0] * rhs.data[2][2] + self.data[3][0] * rhs.data[2][3];
        let t21 = self.data[0][1] * rhs.data[2][0] + self.data[1][1] * rhs.data[2][1] + self.data[2][1] * rhs.data[2][2] + self.data[3][1] * rhs.data[2][3];
        let t22 = self.data[0][2] * rhs.data[2][0] + self.data[1][2] * rhs.data[2][1] + self.data[2][2] * rhs.data[2][2] + self.data[3][2] * rhs.data[2][3];
        let t23 = self.data[0][3] * rhs.data[2][0] + self.data[1][3] * rhs.data[2][1] + self.data[2][3] * rhs.data[2][2] + self.data[3][3] * rhs.data[2][3];
        let t30 = self.data[0][0] * rhs.data[3][0] + self.data[1][0] * rhs.data[3][1] + self.data[2][0] * rhs.data[3][2] + self.data[3][0] * rhs.data[3][3];
        let t31 = self.data[0][1] * rhs.data[3][0] + self.data[1][1] * rhs.data[3][1] + self.data[2][1] * rhs.data[3][2] + self.data[3][1] * rhs.data[3][3];
        let t32 = self.data[0][2] * rhs.data[3][0] + self.data[1][2] * rhs.data[3][1] + self.data[2][2] * rhs.data[3][2] + self.data[3][2] * rhs.data[3][3];
        let t33 = self.data[0][3] * rhs.data[3][0] + self.data[1][3] * rhs.data[3][1] + self.data[2][3] * rhs.data[3][2] + self.data[3][3] * rhs.data[3][3];

        self.data[0][0] = t00;
        self.data[0][1] = t01;
        self.data[0][2] = t02;
        self.data[0][3] = t03;
        self.data[1][0] = t10;
        self.data[1][1] = t11;
        self.data[1][2] = t12;
        self.data[1][3] = t13;
        self.data[2][0] = t20;
        self.data[2][1] = t21;
        self.data[2][2] = t22;
        self.data[2][3] = t23;
        self.data[3][0] = t30;
        self.data[3][1] = t31;
        self.data[3][2] = t32;
        self.data[3][3] = t33;
    }
}

impl<T: PartialOrd + Copy + Mul<Output = T> + Add<Output = T>>
    Mul<Vec4<T>> for Mat44<T> 
{
    type Output = Vec4<T>;

    fn mul(self, rhs: Vec4<T>) -> Self::Output {
        let x = self.data[0][0] * rhs.x + self.data[1][0] * rhs.y + self.data[2][0] * rhs.z + self.data[3][0] * rhs.w;
        let y = self.data[0][1] * rhs.x + self.data[1][1] * rhs.y + self.data[2][1] * rhs.z + self.data[3][1] * rhs.w;
        let z = self.data[0][2] * rhs.x + self.data[1][2] * rhs.y + self.data[2][2] * rhs.z + self.data[3][2] * rhs.w;
        let w = self.data[0][3] * rhs.x + self.data[1][3] * rhs.y + self.data[2][3] * rhs.z + self.data[3][3] * rhs.w;

        Vec4::<T> {
            x, y, z, w
        }
    }
}

/// Implement standard matrix functions.
impl<T: PartialOrd + Copy + Vectorable<T> +
    Div<Output = T> + Mul<Output = T> + Sub<Output = T> + Add<Output = T> +
    Neg<Output = T> + AddAssign<T> + SubAssign<T>>
    StandardMat<T> for Mat44<T> 
{
    fn transpose(&mut self) {
        let t00 = self.data[0][0];
        let t01 = self.data[1][0];
        let t02 = self.data[2][0];
        let t03 = self.data[3][0];
        let t10 = self.data[0][1];
        let t11 = self.data[1][1];
        let t12 = self.data[2][1];
        let t13 = self.data[3][1];
        let t20 = self.data[0][2];
        let t21 = self.data[1][2];
        let t22 = self.data[2][2];
        let t23 = self.data[3][2];
        let t30 = self.data[0][3];
        let t31 = self.data[1][3];
        let t32 = self.data[2][3];
        let t33 = self.data[3][3];

        self.data[0][0] = t00;
        self.data[0][1] = t01;
        self.data[0][2] = t02;
        self.data[0][3] = t03;
        self.data[1][0] = t10;
        self.data[1][1] = t11;
        self.data[1][2] = t12;
        self.data[1][3] = t13;
        self.data[2][0] = t20;
        self.data[2][1] = t21;
        self.data[2][2] = t22;
        self.data[2][3] = t23;
        self.data[3][0] = t30;
        self.data[3][1] = t31;
        self.data[3][2] = t32;
        self.data[3][3] = t33;
    }

    fn invert(&mut self) {
        let determinant = self.det();

        if determinant != T::ZERO {
            let determinant_inv = T::ONE / determinant;

            // First row
            let t00 =  det33(self.data[1][1], self.data[1][2], self.data[1][3], self.data[2][1], self.data[2][2], self.data[2][3], self.data[3][1], self.data[3][2], self.data[3][3]);
            let t01 = -det33(self.data[1][0], self.data[1][2], self.data[1][3], self.data[2][0], self.data[2][2], self.data[2][3], self.data[3][0], self.data[3][2], self.data[3][3]);
            let t02 =  det33(self.data[1][0], self.data[1][1], self.data[1][3], self.data[2][0], self.data[2][1], self.data[2][3], self.data[3][0], self.data[3][1], self.data[3][3]);
            let t03 = -det33(self.data[1][0], self.data[1][1], self.data[1][2], self.data[2][0], self.data[2][1], self.data[2][2], self.data[3][0], self.data[3][1], self.data[3][2]);

            // Second row
            let t10 = -det33(self.data[0][1], self.data[0][2], self.data[0][3], self.data[2][1], self.data[2][2], self.data[2][3], self.data[3][1], self.data[3][2], self.data[3][3]);
            let t11 =  det33(self.data[0][0], self.data[0][2], self.data[0][3], self.data[2][0], self.data[2][2], self.data[2][3], self.data[3][0], self.data[3][2], self.data[3][3]);
            let t12 = -det33(self.data[0][0], self.data[0][1], self.data[0][3], self.data[2][0], self.data[2][1], self.data[2][3], self.data[3][0], self.data[3][1], self.data[3][3]);
            let t13 =  det33(self.data[0][0], self.data[0][1], self.data[0][2], self.data[2][0], self.data[2][1], self.data[2][2], self.data[3][0], self.data[3][1], self.data[3][2]);

            // Third row
            let t20 =  det33(self.data[0][1], self.data[0][2], self.data[0][3], self.data[1][1], self.data[1][2], self.data[1][3], self.data[3][1], self.data[3][2], self.data[3][3]);
            let t21 = -det33(self.data[0][0], self.data[0][2], self.data[0][3], self.data[1][0], self.data[1][2], self.data[1][3], self.data[3][0], self.data[3][2], self.data[3][3]);
            let t22 =  det33(self.data[0][0], self.data[0][1], self.data[0][3], self.data[1][0], self.data[1][1], self.data[1][3], self.data[3][0], self.data[3][1], self.data[3][3]);
            let t23 = -det33(self.data[0][0], self.data[0][1], self.data[0][2], self.data[1][0], self.data[1][1], self.data[1][2], self.data[3][0], self.data[3][1], self.data[3][2]);

            // Fourth row
            let t30 = -det33(self.data[0][1], self.data[0][2], self.data[0][3], self.data[1][1], self.data[1][2], self.data[1][3], self.data[2][1], self.data[2][2], self.data[2][3]);
            let t31 =  det33(self.data[0][0], self.data[0][2], self.data[0][3], self.data[1][0], self.data[1][2], self.data[1][3], self.data[2][0], self.data[2][2], self.data[2][3]);
            let t32 = -det33(self.data[0][0], self.data[0][1], self.data[0][3], self.data[1][0], self.data[1][1], self.data[1][3], self.data[2][0], self.data[2][1], self.data[2][3]);
            let t33 =  det33(self.data[0][0], self.data[0][1], self.data[0][2], self.data[1][0], self.data[1][1], self.data[1][2], self.data[2][0], self.data[2][1], self.data[2][2]);

            // Transpose and divide by the determinant
            self.data[0][0] = t00 * determinant_inv;
            self.data[1][1] = t11 * determinant_inv;
            self.data[2][2] = t22 * determinant_inv;
            self.data[3][3] = t33 * determinant_inv;
            self.data[0][1] = t10 * determinant_inv;
            self.data[1][0] = t01 * determinant_inv;
            self.data[2][0] = t02 * determinant_inv;
            self.data[0][2] = t20 * determinant_inv;
            self.data[1][2] = t21 * determinant_inv;
            self.data[2][1] = t12 * determinant_inv;
            self.data[0][3] = t30 * determinant_inv;
            self.data[3][0] = t03 * determinant_inv;
            self.data[1][3] = t31 * determinant_inv;
            self.data[3][1] = t13 * determinant_inv;
            self.data[3][2] = t23 * determinant_inv;
            self.data[2][3] = t32 * determinant_inv;
        }
    }

    fn det(&self) -> T {
        let mut f = self.data[0][0] * ((self.data[1][1] * self.data[2][2] * self.data[3][3] + self.data[1][2] * self.data[2][3] * self.data[3][1] + self.data[1][3] * self.data[2][1] * self.data[3][2])
        - self.data[1][3] * self.data[2][2] * self.data[3][1]
        - self.data[1][1] * self.data[2][3] * self.data[3][2]
        - self.data[1][2] * self.data[2][1] * self.data[3][3]);

        f -= self.data[0][1] * ((self.data[1][0] * self.data[2][2] * self.data[3][3] + self.data[1][2] * self.data[2][3] * self.data[3][0] + self.data[1][3] * self.data[2][0] * self.data[3][2])
            - self.data[1][3] * self.data[2][2] * self.data[3][0]
            - self.data[1][0] * self.data[2][3] * self.data[3][2]
            - self.data[1][2] * self.data[2][0] * self.data[3][3]);

        f += self.data[0][2] * ((self.data[1][0] * self.data[2][1] * self.data[3][3] + self.data[1][1] * self.data[2][3] * self.data[3][0] + self.data[1][3] * self.data[2][0] * self.data[3][1])
            - self.data[1][3] * self.data[2][1] * self.data[3][0]
            - self.data[1][0] * self.data[2][3] * self.data[3][1]
            - self.data[1][1] * self.data[2][0] * self.data[3][3]);

        f -= self.data[0][3] * ((self.data[1][0] * self.data[2][1] * self.data[3][2] + self.data[1][1] * self.data[2][2] * self.data[3][0] + self.data[1][2] * self.data[2][0] * self.data[3][1])
            - self.data[1][2] * self.data[2][1] * self.data[3][0]
            - self.data[1][0] * self.data[2][2] * self.data[3][1]
            - self.data[1][1] * self.data[2][0] * self.data[3][2]);

        f
    }
}        

fn det33<T: PartialOrd + Copy + Mul<Output = T> + 
    Sub<Output = T> + Add<Output = T>>
    (t00: T, t01: T, t02: T,
     t10: T, t11: T, t12: T,
     t20: T, t21: T, t22: T) -> T
{
    return t00 * (t11 * t22 - t12 * t21)
        + t01 * (t12 * t20 - t10 * t22)
        + t02 * (t10 * t21 - t11 * t20);
}

impl<T: PartialOrd + Copy + Vectorable<T> +
    Mul<Output = T> + Add<Output = T> + Sub<Output = T> +
    AddAssign<T>> StandardMat44<T> for Mat44<T> 
{
    fn scale(&mut self, scale: Vec3<T>) {
        self.data[0][0] = self.data[0][0] * scale.x;
        self.data[0][1] = self.data[0][1] * scale.x;
        self.data[0][2] = self.data[0][2] * scale.x;
        self.data[0][3] = self.data[0][3] * scale.x;
        self.data[1][0] = self.data[1][0] * scale.y;
        self.data[1][1] = self.data[1][1] * scale.y;
        self.data[1][2] = self.data[1][2] * scale.y;
        self.data[1][3] = self.data[1][3] * scale.y;
        self.data[2][0] = self.data[2][0] * scale.z;
        self.data[2][1] = self.data[2][1] * scale.z;
        self.data[2][2] = self.data[2][2] * scale.z;
        self.data[2][3] = self.data[2][3] * scale.z;
    }

    fn translate(&mut self, pos: Vec3<T>) {
        self.data[3][0] += self.data[0][0] * pos.x + self.data[1][0] * pos.y + self.data[2][0] * pos.z;
        self.data[3][1] += self.data[0][1] * pos.x + self.data[1][1] * pos.y + self.data[2][1] * pos.z;
        self.data[3][2] += self.data[0][2] * pos.x + self.data[1][2] * pos.y + self.data[2][2] * pos.z;
        self.data[3][3] += self.data[0][3] * pos.x + self.data[1][3] * pos.y + self.data[2][3] * pos.z;
    }

    fn rotate(&mut self, axis: Vec3<T>, angle: T) {
        let c = angle.cos();
        let s = angle.sin();

        let oneminusc = T::ONE - c;
        let xy = axis.x * axis.y;
        let yz = axis.y * axis.z;
        let xz = axis.x * axis.z;
        let xs = axis.x * s;
        let ys = axis.y * s;
        let zs = axis.z * s;

        let f00 = axis.x * axis.x * oneminusc + c;
        let f01 = xy * oneminusc + zs;
        let f02 = xz * oneminusc - ys;

        let f10 = xy * oneminusc - zs;
        let f11 = axis.y * axis.y * oneminusc + c;
        let f12 = yz * oneminusc + xs;

        let f20 = xz * oneminusc + ys;
        let f21 = yz * oneminusc - xs;
        let f22 = axis.z * axis.z * oneminusc + c;

        let t00 = self.data[0][0] * f00 + self.data[1][0] * f01 + self.data[2][0] * f02;
        let t01 = self.data[0][1] * f00 + self.data[1][1] * f01 + self.data[2][1] * f02;
        let t02 = self.data[0][2] * f00 + self.data[1][2] * f01 + self.data[2][2] * f02;
        let t03 = self.data[0][3] * f00 + self.data[1][3] * f01 + self.data[2][3] * f02;
        let t10 = self.data[0][0] * f10 + self.data[1][0] * f11 + self.data[2][0] * f12;
        let t11 = self.data[0][1] * f10 + self.data[1][1] * f11 + self.data[2][1] * f12;
        let t12 = self.data[0][2] * f10 + self.data[1][2] * f11 + self.data[2][2] * f12;
        let t13 = self.data[0][3] * f10 + self.data[1][3] * f11 + self.data[2][3] * f12;

        self.data[2][0] = self.data[0][0] * f20 + self.data[1][0] * f21 + self.data[2][0] * f22;
        self.data[2][1] = self.data[0][1] * f20 + self.data[1][1] * f21 + self.data[2][1] * f22;
        self.data[2][2] = self.data[0][2] * f20 + self.data[1][2] * f21 + self.data[2][2] * f22;
        self.data[2][3] = self.data[0][3] * f20 + self.data[1][3] * f21 + self.data[2][3] * f22;
        self.data[0][0] = t00;
        self.data[0][1] = t01;
        self.data[0][2] = t02;
        self.data[0][3] = t03;
        self.data[1][0] = t10;
        self.data[1][1] = t11;
        self.data[1][2] = t12;
        self.data[1][3] = t13;
    }
}
