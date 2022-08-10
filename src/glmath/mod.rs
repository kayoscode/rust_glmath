mod vectorable;
pub mod vec2;
pub mod vec3;
pub mod vec4;

use std::ops::{Mul, Div, DivAssign};
use vec2::Vec2;
use vec3::Vec3;
use vec4::Vec4;
use self::vectorable::Vectorable;

pub type Vec2f = Vec2<f32>;
pub type Vec3f = Vec3<f32>;
pub type Vec4f = Vec4<f32>;

pub trait StandardVec<T: Vectorable<T>> 
    where Self: Mul<Output = T> + DivAssign<T> + Div<T, Output = Self> + Sized + Copy,
    T: Mul<Output = T> + Div<Output = T>
{
    fn length(&self) -> T {
        self.length_sq().sqrt()
    }
    fn length_sq(&self) -> T;

    fn normalize(&mut self) {
        self.div_assign(self.length());
    }

    fn get_normalized(&self) -> Self {
        self.div(self.length())
    }

    fn angle_between(&self, other: &Self) -> T {
        let len = self.length();
        let other_len = other.length();
        let combined_len = len * other_len;

        let dot = *self * *other;

        let intermediate_value = dot / combined_len;
        T::acos(&intermediate_value)
    }
}

pub trait TwoDimVec<T: PartialOrd + Copy> 
{
    fn x(&self) -> &T;
    fn y(&self) -> &T;

    fn xy(&self) -> Vec2<T>;
    fn yx(&self) -> Vec2<T>;
}

pub trait ThreeDimVec<T: PartialOrd + Copy> : TwoDimVec<T> {
    fn z(&self) -> &T;

    fn xyz(&self) -> Vec3<T>;
    fn yxz(&self) -> Vec3<T>;
    fn zxy(&self) -> Vec3<T>;
    fn xzy(&self) -> Vec3<T>;
    fn yzx(&self) -> Vec3<T>;
    fn zyx(&self) -> Vec3<T>;
}

pub trait FourDimVec<T: PartialOrd + Copy> : ThreeDimVec<T> {
    fn w(&self) -> &T;

    fn xyzw(&self) -> Vec4<T>;
    fn yxzw(&self) -> Vec4<T>;
    fn zxyw(&self) -> Vec4<T>;
    fn xzyw(&self) -> Vec4<T>;
    fn yzxw(&self) -> Vec4<T>;
    fn zyxw(&self) -> Vec4<T>;

    fn zywx(&self) -> Vec4<T>;
    fn yzwx(&self) -> Vec4<T>;
    fn wzyx(&self) -> Vec4<T>;
    fn zwyx(&self) -> Vec4<T>;
    fn ywzx(&self) -> Vec4<T>;
    fn wyzx(&self) -> Vec4<T>;

    fn wxzy(&self) -> Vec4<T>;
    fn xwzy(&self) -> Vec4<T>;
    fn zwxy(&self) -> Vec4<T>;
    fn wzxy(&self) -> Vec4<T>;
    fn xzwy(&self) -> Vec4<T>;
    fn zxwy(&self) -> Vec4<T>;

    fn yxwz(&self) -> Vec4<T>;
    fn xywz(&self) -> Vec4<T>;
    fn wyxz(&self) -> Vec4<T>;
    fn ywxz(&self) -> Vec4<T>;
    fn xwyz(&self) -> Vec4<T>;
    fn wxyz(&self) -> Vec4<T>;
}