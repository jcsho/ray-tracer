use crate::tuples::{Float, Vector};

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl<'a, 'b> std::ops::Add<&'b Vector> for &'a Point {
    type Output = Point;

    fn add(self, other: &'b Vector) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<'a, 'b> std::ops::Sub<&'b Point> for &'a Point {
    type Output = Vector;

    fn sub(self, other: &'b Point) -> Self::Output {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<'a, 'b> std::ops::Sub<&'b Vector> for &'a Point {
    type Output = Point;

    fn sub(self, other: &'b Vector) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Neg for Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        Point {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
