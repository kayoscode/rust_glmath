pub trait Vectorable<T> {
    fn sqrt(&self) -> T;
    fn acos(&self) -> T;
    fn sin(&self) -> T;
    fn cos(&self) -> T;

    fn atan2(a: T, b: T) -> T;
    fn asin(&self) -> T;

    fn max(a: Self, b: Self) -> T;

    const ZERO: T;
    const ONE: T;
    const TWO: T;
    const HALF: T;
    const QUARTER: T;
    const PI: T;
}

impl Vectorable<f32> for f32 {
    fn sqrt(&self) -> f32 {
        f32::sqrt(*self)
    }

    fn acos(&self) -> f32 {
        f32::acos(*self)
    }

    fn sin(&self) -> f32 {
        f32::sin(*self)
    }

    fn cos(&self) -> f32 {
        f32::cos(*self)
    }

    fn atan2(a: f32, b: f32) -> f32 {
        f32::atan2(a, b)
    }

    fn asin(&self) -> f32 {
        f32::asin(*self)
    }

    fn max(a: Self, b: Self) -> f32 {
        if a > b { a } else { b }
    }

    const ZERO: f32 = 0.0;
    const ONE: f32 = 1.0;
    const TWO: f32 = 2.0;
    const HALF: f32 = 0.5;
    const QUARTER: f32 = 0.25;
    const PI: f32 = 3.1415926;
}

impl Vectorable<f64> for f64 {
    fn sqrt(&self) -> f64 {
        f64::sqrt(*self)
    }

    fn acos(&self) -> f64 {
        f64::acos(*self)
    }

    fn sin(&self) -> f64 {
        f64::sin(*self)
    }

    fn cos(&self) -> f64 {
        f64::cos(*self)
    }

    fn atan2(a: f64, b: f64) -> f64 {
        f64::atan2(a, b)
    }

    fn asin(&self) -> f64 {
        f64::asin(*self)
    }

    fn max(a: Self, b: Self) -> f64 {
        if a > b { a } else { b }
    }

    const ZERO: f64 = 0.0;
    const ONE: f64 = 1.0;
    const TWO: f64 = 2.0;
    const HALF: f64 = 0.5;
    const QUARTER: f64 = 0.25;
    const PI: f64 = 3.1415926;
}
