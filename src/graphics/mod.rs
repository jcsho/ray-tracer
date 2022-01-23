pub use crate::graphics::color::Color;
use crate::Float;

mod color;

pub fn color(red: f64, green: f64, blue: f64) -> Color {
    Color {
        red: Float::from(red),
        green: Float::from(green),
        blue: Float::from(blue),
    }
}
