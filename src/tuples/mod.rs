pub use crate::tuples::tuple_data::TupleData;
use float::Float;
use std::mem::discriminant;
use std::ops::{Add, Neg, Sub};

mod float;
mod tuple_data;

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

/// Short-form constructor for point tuple
pub fn point(x: f64, y: f64, z: f64) -> Tuple {
    Tuple::Point(TupleData {
        x: Float::from(x),
        y: Float::from(y),
        z: Float::from(z),
        w: Float::from(1.0),
    })
}

/// Short-form constructor for vector tuple
pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
    Tuple::Vector(TupleData {
        x: Float::from(x),
        y: Float::from(y),
        z: Float::from(z),
        w: Float::from(0.0),
    })
}
