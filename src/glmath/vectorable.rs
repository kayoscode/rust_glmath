pub trait Vectorable<T> {
    fn sqrt(&self) -> T;
    fn acos(&self) -> T;

    fn zero() -> T;
    fn one() -> T;
}

impl Vectorable<f32> for f32 {
    fn sqrt(&self) -> f32 {
        f32::sqrt(*self)
    }

    fn acos(&self) -> f32 {
        f32::acos(*self)
    }

    fn zero() -> f32 {
        0.0
    }

    fn one() -> f32 {
        1.0
    }
}

impl Vectorable<f64> for f64 {
    fn sqrt(&self) -> f64 {
        f64::sqrt(*self)
    }

    fn acos(&self) -> f64 {
        f64::acos(*self)
    }

    fn zero() -> f64 {
        0.0
    }

    fn one() -> f64 {
        1.0
    }
}

impl Vectorable<i32> for i32 {
    fn sqrt(&self) -> i32 {
        f64::sqrt(*self as f64) as i32
    }

    fn acos(&self) -> i32 {
        f64::acos(*self as f64) as i32
    }

    fn zero() -> i32 {
        0
    }

    fn one() -> i32 {
        1
    }
}

impl Vectorable<i64> for i64 {
    fn sqrt(&self) -> i64 {
        f64::sqrt(*self as f64) as i64
    }

    fn acos(&self) -> i64 {
        f64::acos(*self as f64) as i64
    }

    fn zero() -> i64 {
        0
    }
    
    fn one() -> i64 {
        1
    }
}