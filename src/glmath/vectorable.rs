pub trait Vectorable<T> {
    fn sqrt(&self) -> T;
    fn acos(&self) -> T;
    fn sin(&self) -> T;
    fn cos(&self) -> T;

    const ZERO: T;
    const ONE: T;
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

    const ZERO: f32 = 0.0;
    const ONE: f32 = 1.0;
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

    const ZERO: f64 = 0.0;
    const ONE: f64 = 1.0;
}
