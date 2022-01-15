pub use crate::tuples::float::Float;
pub use crate::tuples::point::Point;
pub use crate::tuples::vector::Vector;

mod float;
mod point;
mod vector;

/// Short-form constructor for point tuple
pub fn point(x: f64, y: f64, z: f64) -> Point {
    Point {
        x: Float::from(x),
        y: Float::from(y),
        z: Float::from(z),
    }
}

/// Short-form constructor for vector tuple
pub fn vector(x: f64, y: f64, z: f64) -> Vector {
    Vector {
        x: Float::from(x),
        y: Float::from(y),
        z: Float::from(z),
    }
}
