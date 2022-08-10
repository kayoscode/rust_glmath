pub trait Vectorable<T> {
    fn sqrt(&self) -> T;
    fn acos(&self) -> T;
}

impl Vectorable<f32> for f32 {
    fn sqrt(&self) -> f32 {
        f32::sqrt(*self)
    }

    fn acos(&self) -> f32 {
        f32::acos(*self)
    }
}

impl Vectorable<f64> for f64 {
    fn sqrt(&self) -> f64 {
        f64::sqrt(*self)
    }

    fn acos(&self) -> f64 {
        f64::acos(*self)
    }
}

impl Vectorable<i32> for i32 {
    fn sqrt(&self) -> i32 {
        f64::sqrt(*self as f64) as i32
    }

    fn acos(&self) -> i32 {
        f64::acos(*self as f64) as i32
    }
}

impl Vectorable<u32> for u32 {
    fn sqrt(&self) -> u32 {
        f64::sqrt(*self as f64) as u32
    }

    fn acos(&self) -> u32 {
        f64::acos(*self as f64) as u32
    }
}

impl Vectorable<i64> for i64 {
    fn sqrt(&self) -> i64 {
        f64::sqrt(*self as f64) as i64
    }

    fn acos(&self) -> i64 {
        f64::acos(*self as f64) as i64
    }
}

impl Vectorable<u64> for u64 {
    fn sqrt(&self) -> u64 {
        f64::sqrt(*self as f64) as u64
    }

    fn acos(&self) -> u64 {
        f64::acos(*self as f64) as u64
    }
}