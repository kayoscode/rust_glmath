use std::{fmt::Display, ops::{Add, AddAssign, SubAssign, Sub, MulAssign, Neg}};

use crate::glmath::*;

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Mat22<T: PartialOrd + Copy> {
    data: [[T; 2]; 2]
}

impl<T: Vectorable<T> + PartialOrd + Copy> Mat22<T> {
    /// Returns an identity matrix.
    pub fn new() -> Mat22<T> {
        Self::IDENTITY
    }

    /// Constructs a matrix from axis values.
    pub fn from_axes(x: Vec2<T>, y: Vec2<T>) -> Mat22<T> {
        Mat22::<T> {
            data: [
                [ x.x, x.y ],
                [ y.x, y.y ]
            ]
        }
    }

    pub const IDENTITY: Mat22<T> = Mat22::<T> {
        data: [
            [ T::ONE, T::ZERO ],
            [ T::ZERO, T::ONE ]
        ]
    };

    pub const ZERO: Mat22<T> = Mat22::<T> {
        data: [
            [ T::ZERO, T::ZERO ],
            [ T::ZERO, T::ZERO ]
        ]
    };
}

impl<T: PartialOrd + Copy + Display> Display for Mat22<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]\n[{}, {}]", self.data[0][0],
                self.data[1][0], self.data[0][1], 
                self.data[1][1])
    }
}

impl<T: PartialOrd + Copy + Neg<Output = T>> Neg for Mat22<T> {
    type Output = Mat22<T>;

    fn neg(self) -> Self::Output {
        Mat22::<T> {
            data: [
                [ -self.data[0][0], -self.data[0][1] ],
                [ -self.data[1][0], -self.data[1][1] ]
            ]
        }
    }
}

impl<T: PartialOrd + Copy + Add<Output = T>> Add<Mat22<T>> for Mat22<T> {
    type Output = Mat22<T>;

    fn add(self, rhs: Mat22<T>) -> Self::Output {
        Mat22::<T> {
            data: [
                [
                    self.data[0][0] + rhs.data[0][0],
                    self.data[0][1] + rhs.data[0][1]
                ],
                [
                    self.data[1][0] + rhs.data[1][0],
                    self.data[1][1] + rhs.data[1][1]
                ]
            ]
        }
    }
}

impl<T: PartialOrd + Copy + AddAssign<T>> AddAssign for Mat22<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.data[0][0] += rhs.data[0][0];
        self.data[0][1] += rhs.data[0][1];
        self.data[0][1] += rhs.data[1][0];
        self.data[1][1] += rhs.data[1][1];
    }
}

impl<T: PartialOrd + Copy + Sub<Output = T>> Sub<Mat22<T>> for Mat22<T> {
    type Output = Mat22<T>;

    fn sub(self, rhs: Mat22<T>) -> Self::Output {
        Mat22::<T> {
            data: [
                [
                    self.data[0][0] - rhs.data[0][0],
                    self.data[0][1] - rhs.data[0][1]
                ],
                [
                    self.data[1][0] - rhs.data[1][0],
                    self.data[1][1] - rhs.data[1][1]
                ]
            ]
        }
    }
}

impl<T: PartialOrd + Copy + SubAssign<T>> SubAssign for Mat22<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.data[0][0] -= rhs.data[0][0];
        self.data[0][1] -= rhs.data[0][1];
        self.data[0][1] -= rhs.data[1][0];
        self.data[1][1] -= rhs.data[1][1];
    }
}

impl<T: PartialOrd + Copy + Mul<Output = T> + Add<Output = T>>
    Mul<Mat22<T>> for Mat22<T> 
{
    type Output = Mat22<T>;

    fn mul(self, rhs: Mat22<T>) -> Self::Output {
        let m00 = self.data[0][0] * rhs.data[0][0] + self.data[1][0] * rhs.data[0][1];
        let m01 = self.data[0][1] * rhs.data[0][0] + self.data[1][1] * rhs.data[0][1];
        let m10 = self.data[0][0] * rhs.data[1][0] + self.data[1][0] * rhs.data[1][1];
        let m11 = self.data[0][1] * rhs.data[1][0] + self.data[1][1] * rhs.data[1][1];
    
        Mat22::<T> {
            data: [
                [m00, m01],
                [m10, m11]
            ]
        }
    }
}

impl<T: PartialOrd + Copy + Mul<Output = T> + Add<Output = T>>
    MulAssign for Mat22<T> 
{
    fn mul_assign(&mut self, rhs: Self) {
        let m00 = self.data[0][0] * rhs.data[0][0] + self.data[1][0] * rhs.data[0][1];
        let m01 = self.data[0][1] * rhs.data[0][0] + self.data[1][1] * rhs.data[0][1];
        let m10 = self.data[0][0] * rhs.data[1][0] + self.data[1][0] * rhs.data[1][1];
        let m11 = self.data[0][1] * rhs.data[1][0] + self.data[1][1] * rhs.data[1][1];

        self.data[0][0] = m00;
        self.data[0][1] = m01;
        self.data[1][0] = m10;
        self.data[1][1] = m11;
    }
}

impl<T: PartialOrd + Copy + Mul<Output = T> + Add<Output = T>>
    Mul<Vec2<T>> for Mat22<T> 
{
    type Output = Vec2<T>;

    fn mul(self, rhs: Vec2<T>) -> Self::Output {
        let x = self.data[0][0] * rhs.x + self.data[1][0] * rhs.y;
        let y = self.data[0][1] * rhs.x + self.data[1][1] * rhs.y;

        Vec2::<T> {
            x, y
        }
    }
}

/// Implement standard matrix functions.
impl<T: PartialOrd + Copy + Vectorable<T> +
    Div<Output = T> + Mul<Output = T> + Sub<Output = T> +
    Neg<Output = T>>
    StandardMat<T> for Mat22<T> 
{
    fn transpose(&mut self) {
        let m01 = self.data[1][0];
        let m10 = self.data[0][1];

        self.data[0][1] = m01;
        self.data[1][0] = m10;
    }

    fn invert(&mut self) {
        let determinant = self.det();

        if determinant != T::ZERO {
            let determinant_inv = (T::ONE) / determinant;

            let t00 =  self.data[1][1] * determinant_inv;
            let t01 = -self.data[0][1] * determinant_inv;
            let t11 =  self.data[0][0] * determinant_inv;
            let t10 = -self.data[1][0] * determinant_inv;

            self.data[0][0] = t00;
            self.data[0][1] = t01;
            self.data[1][0] = t10;
            self.data[1][1] = t11;
        }
    }

    fn det(&self) -> T {
        return self.data[0][0] * self.data[1][1] - 
            self.data[0][1] * self.data[1][0];
    }
}