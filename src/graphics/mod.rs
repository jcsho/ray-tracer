pub use crate::graphics::canvas::{
    canvas_to_ppm,
    pixel_at,
    write_pixel,
    Canvas};
pub use crate::graphics::color::Color;
use crate::Float;

mod canvas;
mod color;

pub fn color(red: f64, green: f64, blue: f64) -> Color {
    Color {
        red: Float::from(red),
        green: Float::from(green),
        blue: Float::from(blue),
    }
}

pub fn canvas(width: usize, height: usize) -> Canvas {
    // fill canvas with black
    let default_color = color(0.0, 0.0, 0.0);
    Canvas {
        width,
        height,
        pixels: vec![default_color; width * height],
    }
}
