use std::mem::discriminant;

/// Base type to represent the coordinate system
/// (uses the left-handed coordinate system)
#[derive(Debug)]
pub struct TupleData {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
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
    Tuple::Point(TupleData { x, y, z, w: 1.0 })
}

pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
    Tuple::Vector(TupleData { x, y, z, w: 0.0 })
}
