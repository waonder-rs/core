pub trait One: Sized {
    const ONE: Self;
}

impl One for f32 {
    const ONE: f32 = 1.0f32;
}

impl One for f64 {
    const ONE: f64 = 1.0f64;
}

// 2d norm 2.
pub trait Norm: Sized {
    type Output;

    fn norm(vector: &[Self]) -> Self::Output;
}

impl Norm for f32 {
    type Output = f32;

    fn norm(v: &[f32]) -> f32 {
        let mut n = 0.0;
        for x in v.iter() {
            n += x*x;
        }

        n.sqrt()
    }
}

impl Norm for f64 {
    type Output = f64;

    fn norm(v: &[f64]) -> f64 {
        let mut n = 0.0;
        for x in v.iter() {
            n += x*x;
        }

        n.sqrt()
    }
}

/// Type on which the trigonometrics operation are defined.
pub trait Trigonometric: Sized {
    type Output;

    fn sin(&self) -> Self::Output;

    fn cos(&self) -> Self::Output;

    fn tan(&self) -> Self::Output;

    fn asin(x: Self::Output) -> Self;

    fn acos(x: Self::Output) -> Self;

    fn atan(x: Self::Output) -> Self;
}

impl Trigonometric for f32 {
    type Output = f32;

    fn sin(&self) -> f32 {
        f32::sin(*self)
    }

    fn cos(&self) -> f32 {
        f32::cos(*self)
    }

    fn tan(&self) -> f32 {
        f32::cos(*self)
    }

    fn asin(f: f32) -> f32 {
        f32::asin(f)
    }

    fn acos(f: f32) -> f32 {
        f32::acos(f)
    }

    fn atan(f: f32) -> f32 {
        f32::atan(f)
    }
}

impl Trigonometric for f64 {
    type Output = f64;

    fn sin(&self) -> f64 {
        f64::sin(*self)
    }

    fn cos(&self) -> f64 {
        f64::cos(*self)
    }

    fn tan(&self) -> f64 {
        f64::tan(*self)
    }

    fn asin(f: f64) -> f64 {
        f64::asin(f)
    }

    fn acos(f: f64) -> f64 {
        f64::acos(f)
    }

    fn atan(f: f64) -> f64 {
        f64::atan(f)
    }
}
