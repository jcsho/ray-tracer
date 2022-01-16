pub use crate::tuples::float::Float;
pub use crate::tuples::point::Point;
pub use crate::tuples::vector::Vector;

mod float;
mod point;
mod vector;

pub fn point(x: f64, y: f64, z: f64) -> Point {
    Point {
        x: Float::from(x),
        y: Float::from(y),
        z: Float::from(z),
    }
}

pub fn vector(x: f64, y: f64, z: f64) -> Vector {
    Vector {
        x: Float::from(x),
        y: Float::from(y),
        z: Float::from(z),
    }
}

pub fn magnitude(v: Vector) -> Float {
    (v.x.pow(2) + v.y.pow(2) + v.z.pow(2)).sqrt()
}

pub fn normalize(v: Vector) -> Vector {
    Vector {
        x: v.x / magnitude(v),
        y: v.y / magnitude(v),
        z: v.z / magnitude(v),
    }
}

pub fn dot_product(v1: Vector, v2: Vector) -> Float {
    (v1.x * v2.x) + (v1.y * v2.y) + (v1.z * v2.z)
}
