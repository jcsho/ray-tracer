use std::mem::discriminant;
use std::ops::{Add, Neg, Sub};

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

impl Tuple {
    fn unwrap(&self) -> &TupleData {
        match self {
            Tuple::Point(td) | Tuple::Vector(td) => td,
        }
    }
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

impl TryFrom<TupleData> for Tuple {
    type Error = ();

    fn try_from(value: TupleData) -> Result<Self, Self::Error> {
        if value.w == 1.0 {
            Ok(Tuple::Point(value))
        } else if value.w == 0.0 {
            Ok(Tuple::Vector(value))
        } else {
            Err(())
        }
    }
}

impl<'a, 'b> Add<&'b Tuple> for &'a Tuple {
    type Output = Result<Tuple, ()>;

    fn add(self, rhs: &'b Tuple) -> Self::Output {
        match Tuple::try_from(self.unwrap() + rhs.unwrap()) {
            Ok(res) => Ok(res),
            _ => Err(()),
        }
    }
}

impl<'a, 'b> Sub<&'b Tuple> for &'a Tuple {
    type Output = Result<Tuple, ()>;

    fn sub(self, rhs: &'b Tuple) -> Self::Output {
        match Tuple::try_from(self.unwrap() - rhs.unwrap()) {
            Ok(res) => Ok(res),
            _ => Err(()),
        }
    }
}

impl<'a> Neg for &'a Tuple {
    type Output = Result<Tuple, ()>;

    fn neg(self) -> Self::Output {
        let td = self.unwrap();
        Tuple::try_from(TupleData {
            x: -td.x,
            y: -td.y,
            z: -td.z,
            w: td.w,
        })
    }
}

impl<'a, 'b> Add<&'b TupleData> for &'a TupleData {
    type Output = TupleData;

    fn add(self, rhs: &'b TupleData) -> Self::Output {
        TupleData {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl<'a, 'b> Sub<&'b TupleData> for &'a TupleData {
    type Output = TupleData;

    fn sub(self, rhs: &'b TupleData) -> Self::Output {
        TupleData {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
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
        Float(self.0 + rhs.0)
    }
}

impl Sub<Self> for Float {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Float(self.0 - rhs.0)
    }
}

impl Neg for Float {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Float(-self.0)
    }
}
