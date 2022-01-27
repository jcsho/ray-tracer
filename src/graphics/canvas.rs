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

const PPM_VERSION: &str = "P3";
const MAX_COLOR_VALUE: u8 = 255;

pub fn canvas_to_ppm(canvas: &Canvas) -> String {
    let mut ppm_file = format!("{}\n", PPM_VERSION);
    ppm_file.push_str(&format!(
        "{width} {height}\n",
        width = canvas.width,
        height = canvas.height
    ));
    ppm_file.push_str(&format!("{}\n", MAX_COLOR_VALUE));

    ppm_file
}
