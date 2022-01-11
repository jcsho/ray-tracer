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
