use std::{fmt::Display, ops::{Add, AddAssign, SubAssign, Sub, MulAssign, Neg}};

use crate::glmath::*;

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Mat33<T: PartialOrd + Copy> {
    data: [[T; 3]; 3]
}

impl<T: Vectorable<T> + PartialOrd + Copy> Mat33<T> {
    /// Returns an identity matrix.
    pub fn new() -> Mat33<T> {
        Self::IDENTITY
    }

    /// Constructs a matrix from axis values.
    pub fn from_axes(x: Vec3<T>, y: Vec3<T>, z: Vec3<T>) -> Mat33<T> {
        Mat33::<T> {
            data: [
                [ x.x, x.y, x.z ],
                [ y.x, y.y, y.z ],
                [ z.x, z.y, z.z ]
            ]
        }
    }

    pub const IDENTITY: Mat33<T> = Mat33::<T> {
        data: [
            [ T::ONE, T::ZERO, T::ZERO ],
            [ T::ZERO, T::ONE, T::ZERO ],
            [ T::ZERO, T::ZERO, T::ONE ]
        ]
    };

    pub const ZERO: Mat33<T> = Mat33::<T> {
        data: [
            [ T::ZERO, T::ZERO, T::ZERO ],
            [ T::ZERO, T::ZERO, T::ZERO ],
            [ T::ZERO, T::ZERO, T::ZERO ]
        ]
    };
}

impl<T: PartialOrd + Copy + Display> Display for Mat33<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}, {}]\n[{}, {}, {}]\n[{}, {}, {}]", 
            self.data[0][0], self.data[1][0], self.data[2][0], 
            self.data[0][1], self.data[1][1], self.data[2][1], 
            self.data[0][2], self.data[1][2], self.data[2][2])
    }
}

impl<T: PartialOrd + Copy + Neg<Output = T>> Neg for Mat33<T> {
    type Output = Mat33<T>;

    fn neg(self) -> Self::Output {
        Mat33::<T> {
            data:[
                [ -self.data[0][0], -self.data[0][1], -self.data[0][2] ],
                [ -self.data[1][0], -self.data[1][1], -self.data[1][2] ],
                [ -self.data[2][0], -self.data[2][1], -self.data[2][2] ]
            ]
        }
    }
}

impl<T: PartialOrd + Copy + Add<Output = T>> Add<Mat33<T>> for Mat33<T> {
    type Output = Mat33<T>;

    fn add(self, rhs: Mat33<T>) -> Self::Output {
        Mat33::<T> {
            data: [
                [
                    self.data[0][0] + rhs.data[0][0],
                    self.data[0][1] + rhs.data[0][1],
                    self.data[0][2] + rhs.data[0][2],
                ],
                [
                    self.data[1][0] + rhs.data[1][0],
                    self.data[1][1] + rhs.data[1][1],
                    self.data[1][2] + rhs.data[1][2]
                ],
                [
                    self.data[2][0] + rhs.data[2][0],
                    self.data[2][1] + rhs.data[2][1],
                    self.data[2][2] + rhs.data[2][2]
                ]
            ]
        }
    }
}

impl<T: PartialOrd + Copy + Add<Output = T>> AddAssign for Mat33<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.data[0][0] = self.data[0][0] + rhs.data[0][0];
		self.data[0][1] = self.data[0][1] + rhs.data[0][1];
		self.data[0][2] = self.data[0][2] + rhs.data[0][2];
		self.data[1][0] = self.data[1][0] + rhs.data[1][0];
		self.data[1][1] = self.data[1][1] + rhs.data[1][1];
		self.data[1][2] = self.data[1][2] + rhs.data[1][2];
		self.data[2][0] = self.data[2][0] + rhs.data[2][0];
		self.data[2][1] = self.data[2][1] + rhs.data[2][1];
		self.data[2][2] = self.data[2][2] + rhs.data[2][2];
    }
}

impl<T: PartialOrd + Copy + Sub<Output = T>> Sub<Mat33<T>> for Mat33<T> {
    type Output = Mat33<T>;

    fn sub(self, rhs: Mat33<T>) -> Self::Output {

        Mat33::<T> {
            data: [
                [
                    self.data[0][0] - rhs.data[0][0],
                    self.data[0][1] - rhs.data[0][1],
                    self.data[0][2] - rhs.data[0][2],
                ],
                [
                    self.data[1][0] - rhs.data[1][0],
                    self.data[1][1] - rhs.data[1][1],
                    self.data[1][2] - rhs.data[1][2]
                ],
                [
                    self.data[2][0] - rhs.data[2][0],
                    self.data[2][1] - rhs.data[2][1],
                    self.data[2][2] - rhs.data[2][2]
                ]
            ]
        }
    }
}

impl<T: PartialOrd + Copy + Sub<Output = T>> SubAssign for Mat33<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.data[0][0] = self.data[0][0] - rhs.data[0][0];
		self.data[0][1] = self.data[0][1] - rhs.data[0][1];
		self.data[0][2] = self.data[0][2] - rhs.data[0][2];
		self.data[1][0] = self.data[1][0] - rhs.data[1][0];
		self.data[1][1] = self.data[1][1] - rhs.data[1][1];
		self.data[1][2] = self.data[1][2] - rhs.data[1][2];
		self.data[2][0] = self.data[2][0] - rhs.data[2][0];
		self.data[2][1] = self.data[2][1] - rhs.data[2][1];
		self.data[2][2] = self.data[2][2] - rhs.data[2][2];
    }
}

impl<T: PartialOrd + Copy + Mul<Output = T> + Add<Output = T>>
    Mul<Mat33<T>> for Mat33<T> 
{
    type Output = Mat33<T>;

    fn mul(self, rhs: Mat33<T>) -> Self::Output {
		let m00 = self.data[0][0] * rhs.data[0][0] + self.data[1][0] * rhs.data[0][1] + self.data[2][0] * rhs.data[0][2];
		let m01 = self.data[0][1] * rhs.data[0][0] + self.data[1][1] * rhs.data[0][1] + self.data[2][1] * rhs.data[0][2];
		let m02 = self.data[0][2] * rhs.data[0][0] + self.data[1][2] * rhs.data[0][1] + self.data[2][2] * rhs.data[0][2];
		let m10 = self.data[0][0] * rhs.data[1][0] + self.data[1][0] * rhs.data[1][1] + self.data[2][0] * rhs.data[1][2];
		let m11 = self.data[0][1] * rhs.data[1][0] + self.data[1][1] * rhs.data[1][1] + self.data[2][1] * rhs.data[1][2];
		let m12 = self.data[0][2] * rhs.data[1][0] + self.data[1][2] * rhs.data[1][1] + self.data[2][2] * rhs.data[1][2];
		let m20 = self.data[0][0] * rhs.data[2][0] + self.data[1][0] * rhs.data[2][1] + self.data[2][0] * rhs.data[2][2];
		let m21 = self.data[0][1] * rhs.data[2][0] + self.data[1][1] * rhs.data[2][1] + self.data[2][1] * rhs.data[2][2];
		let m22 = self.data[0][2] * rhs.data[2][0] + self.data[1][2] * rhs.data[2][1] + self.data[2][2] * rhs.data[2][2];

        Mat33::<T> {
            data: [
                [ m00, m01, m02 ],
                [ m10, m11, m12 ],
                [ m20, m21, m22 ]
            ]
        }
    }
}

impl<T: PartialOrd + Copy + Mul<Output = T> + Add<Output = T>>
    MulAssign for Mat33<T> 
{
    fn mul_assign(&mut self, rhs: Self) {
		let m00 = self.data[0][0] * rhs.data[0][0] + self.data[1][0] * rhs.data[0][1] + self.data[2][0] * rhs.data[0][2];
		let m01 = self.data[0][1] * rhs.data[0][0] + self.data[1][1] * rhs.data[0][1] + self.data[2][1] * rhs.data[0][2];
		let m02 = self.data[0][2] * rhs.data[0][0] + self.data[1][2] * rhs.data[0][1] + self.data[2][2] * rhs.data[0][2];
		let m10 = self.data[0][0] * rhs.data[1][0] + self.data[1][0] * rhs.data[1][1] + self.data[2][0] * rhs.data[1][2];
		let m11 = self.data[0][1] * rhs.data[1][0] + self.data[1][1] * rhs.data[1][1] + self.data[2][1] * rhs.data[1][2];
		let m12 = self.data[0][2] * rhs.data[1][0] + self.data[1][2] * rhs.data[1][1] + self.data[2][2] * rhs.data[1][2];
		let m20 = self.data[0][0] * rhs.data[2][0] + self.data[1][0] * rhs.data[2][1] + self.data[2][0] * rhs.data[2][2];
		let m21 = self.data[0][1] * rhs.data[2][0] + self.data[1][1] * rhs.data[2][1] + self.data[2][1] * rhs.data[2][2];
		let m22 = self.data[0][2] * rhs.data[2][0] + self.data[1][2] * rhs.data[2][1] + self.data[2][2] * rhs.data[2][2];

        self.data[0][0] = m00;
		self.data[0][1] = m01;
		self.data[0][2] = m02;
		self.data[1][0] = m10;
		self.data[1][1] = m11;
		self.data[1][2] = m12;
		self.data[2][0] = m20;
		self.data[2][1] = m21;
		self.data[2][2] = m22;
    }
}

impl<T: PartialOrd + Copy + Mul<Output = T> + Add<Output = T>>
    Mul<Vec3<T>> for Mat33<T> 
{
    type Output = Vec3<T>;

    fn mul(self, rhs: Vec3<T>) -> Self::Output {
        let x = self.data[0][0] * rhs.x + self.data[1][0] * rhs.y + self.data[2][0] * rhs.z;
		let y = self.data[0][1] * rhs.x + self.data[1][1] * rhs.y + self.data[2][1] * rhs.z;
		let z = self.data[0][2] * rhs.x + self.data[1][2] * rhs.y + self.data[2][2] * rhs.z;

        Vec3::<T> {
            x, y, z
        }
    }
}

/// Implement standard matrix functions.
impl<T: PartialOrd + Copy + Vectorable<T> +
    Div<Output = T> + Mul<Output = T> + Sub<Output = T> + Add<Output = T> +
    Neg<Output = T>>
    StandardMat<T> for Mat33<T> 
{
    fn transpose(&mut self) {
        let m00 = self.data[0][0];
		let m01 = self.data[1][0];
		let m02 = self.data[2][0];
		let m10 = self.data[0][1];
		let m11 = self.data[1][1];
		let m12 = self.data[2][1];
		let m20 = self.data[0][2];
		let m21 = self.data[1][2];
		let m22 = self.data[2][2];

		self.data[0][0] = m00;
		self.data[0][1] = m01;
		self.data[0][2] = m02;
		self.data[1][0] = m10;
		self.data[1][1] = m11;
		self.data[1][2] = m12;
		self.data[2][0] = m20;
		self.data[2][1] = m21;
		self.data[2][2] = m22;
    }

    fn invert(&mut self) {
        let determinant = self.det();

        if determinant != T::ZERO {
            let determinant_inv = T::ONE / determinant;

			let t00 = self.data[1][1] * self.data[2][2] - self.data[1][2]* self.data[2][1];
			let t01 = -self.data[1][0] * self.data[2][2] + self.data[1][2] * self.data[2][0];
			let t02 = self.data[1][0] * self.data[2][1] - self.data[1][1] * self.data[2][0];
			let t10 = -self.data[0][1] * self.data[2][2] + self.data[0][2] * self.data[2][1];
			let t11 = self.data[0][0] * self.data[2][2] - self.data[0][2] * self.data[2][0];
			let t12 = -self.data[0][0] * self.data[2][1] + self.data[0][1] * self.data[2][0];
			let t20 = self.data[0][1] * self.data[1][2] - self.data[0][2] * self.data[1][1];
			let t21 = -self.data[0][0] * self.data[1][2] + self.data[0][2] * self.data[1][0];
			let t22 = self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0];

			self.data[0][0] = t00 * determinant_inv;
			self.data[1][1] = t11 * determinant_inv;
			self.data[2][2] = t22 * determinant_inv;
			self.data[0][1] = t10 * determinant_inv;
			self.data[1][0] = t01 * determinant_inv;
			self.data[2][0] = t02 * determinant_inv;
			self.data[0][2] = t20 * determinant_inv;
			self.data[1][2] = t21 * determinant_inv;
			self.data[2][1] = t12 * determinant_inv;
        }
    }

    fn det(&self) -> T {
        self.data[0][0] * (self.data[1][1] * self.data[2][2] - self.data[1][2] * self.data[2][1])
            + self.data[0][1] * (self.data[1][2] * self.data[2][0] - self.data[1][0] * self.data[2][2])
            + self.data[0][2] * (self.data[1][0] * self.data[2][1] - self.data[1][1] * self.data[2][0])
    }
}