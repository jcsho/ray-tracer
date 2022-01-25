use crate::graphics::Color;

#[derive(Debug)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>,
}

pub fn write_pixel(canvas: &mut Canvas, x: usize, y: usize, color: Color) {
    let index = x + (canvas.width * y);
    canvas.pixels[index] = color;
}

pub fn pixel_at(canvas: &Canvas, x: usize, y: usize) -> Color {
    let index = x + (canvas.width * y);
    canvas.pixels[index]
}
