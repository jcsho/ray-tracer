use crate::float::Float;

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub red: Float,
    pub green: Float,
    pub blue: Float,
}

impl PartialEq<Self> for Color {
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red && self.green == other.green && self.blue == other.blue
    }
}

impl std::ops::Add<Self> for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Color {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl std::ops::Sub<Self> for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Color {
            red: self.red - rhs.red,
            green: self.green - rhs.green,
            blue: self.blue - rhs.blue,
        }
    }
}

impl std::ops::Mul<Self> for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Color {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
        }
    }
}

impl std::ops::Mul<Float> for Color {
    type Output = Self;

    fn mul(self, rhs: Float) -> Self::Output {
        Color {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        }
    }
}
