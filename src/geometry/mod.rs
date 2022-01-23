use crate::float::Float;
pub use crate::geometry::point::Point;
pub use crate::geometry::vector::Vector;

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

pub fn cross_product(v1: Vector, v2: Vector) -> Vector {
    Vector {
        x: (v1.y * v2.z) - (v1.z * v2.y),
        y: (v1.z * v2.x) - (v1.x * v2.z),
        z: (v1.x * v2.y) - (v1.y * v2.x),
    }
}
