use std::mem::discriminant;
use std::ops::Add;

/// Base type to represent the coordinate system
/// (uses the left-handed coordinate system)
#[derive(Debug)]
pub struct TupleData {
    pub x: Float,
    pub y: Float,
    pub z: Float,
    pub w: Float,
}

/// Type class to represent the two variations of Tuple
#[derive(Debug)]
pub enum Tuple {
    Point(TupleData),
    Vector(TupleData),
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
}

pub fn point(x: f64, y: f64, z: f64) -> Tuple {
    Tuple::Point(TupleData {
        x: Float::from(x),
        y: Float::from(y),
        z: Float::from(z),
        w: Float::from(1.0),
    })
}

pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
    Tuple::Vector(TupleData {
        x: Float::from(x),
        y: Float::from(y),
        z: Float::from(z),
        w: Float::from(0.0),
    })
}

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

impl Add<Self> for Float {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Float::from(self.0 + rhs.0)
    }
}
