use crate::tuples::float::Float;
use std::ops::{Add, Sub};

/// Base type to represent the coordinate system
/// (uses the left-handed coordinate system)
#[derive(Debug)]
pub struct TupleData {
    pub x: Float,
    pub y: Float,
    pub z: Float,
    pub w: Float,
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
