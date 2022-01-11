use std::mem::discriminant;

/// Base type to represent the coordinate system
/// (uses the left-handed coordinate system)
#[derive(Debug)]
pub enum Tuple {
    Point { x: f64, y: f64, z: f64, w: f64 },
    Vector { x: f64, y: f64, z: f64, w: f64 },
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
}

pub fn point(x: f64, y: f64, z: f64) -> Tuple {
    Tuple::Point { x, y, z, w: 1.0 }
}

pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
    Tuple::Vector { x, y, z, w: 0.0 }
}
