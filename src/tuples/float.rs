/// New-type wrapper for f64
/// implements comparisons against f64
#[derive(Copy, Clone, Debug)]
pub struct Float(f64);

/// Constant for floating-point number comparisons
/// precision is enough for ray-tracer
const EPSILON: f64 = 0.00001;

impl From<f64> for Float {
    fn from(value: f64) -> Self {
        Float(value)
    }
}

impl PartialEq<Self> for Float {
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() < EPSILON
    }
}

impl PartialEq<f64> for Float {
    fn eq(&self, other: &f64) -> bool {
        (self.0 - other).abs() < EPSILON
    }
}

impl std::ops::Add<Self> for Float {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Float(self.0 + rhs.0)
    }
}

impl std::ops::Sub<Self> for Float {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Float(self.0 - rhs.0)
    }
}

impl std::ops::Neg for Float {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Float(-self.0)
    }
}

impl std::ops::Mul<Self> for Float {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Float(self.0 * rhs.0)
    }
}

impl std::ops::Div<Self> for Float {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Float(self.0 / rhs.0)
    }
}

impl Float {
    pub fn pow(&self, exponent: i32) -> Self {
        Float(self.0.powi(exponent))
    }

    pub fn sqrt(&self) -> Self {
        Float(self.0.sqrt())
    }
}
