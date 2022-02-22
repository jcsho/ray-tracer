use crate::graphics::Color;
use crate::Float;

use rayon::prelude::*;

#[derive(Debug)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>,
}

pub fn write_pixel(canvas: &mut Canvas, x: usize, y: usize, color: Color) {
    let index = x + (canvas.width * y);
    if index > canvas.pixels.len() {
        return;
    }
    canvas.pixels[index] = color;
}

pub fn pixel_at(canvas: &Canvas, x: usize, y: usize) -> Color {
    let index = x + (canvas.width * y);
    canvas.pixels[index]
}

const PPM_VERSION: &str = "P3";
const MIN_SRGB_VALUE: u8 = 0;
const MAX_SRGB_VALUE: u8 = 255;

fn scale_to_srgb(value: Float) -> u8 {
    let result = (value * MAX_SRGB_VALUE).round() as i64;

    if result <= MIN_SRGB_VALUE as i64 {
        return MIN_SRGB_VALUE;
    } else if result >= MAX_SRGB_VALUE as i64 {
        return MAX_SRGB_VALUE;
    }

    result as u8
}

fn convert_srgb_channel_to_ppm_string(
    color_channel: &u8,
    mut line_length: usize,
) -> (String, usize) {
    const PPM_MAX_LINE_LENGTH: u8 = 70;

    let mut result = String::new();

    let color_channel_string_length = color_channel.to_string().chars().count();

    if line_length + color_channel_string_length > (PPM_MAX_LINE_LENGTH - 1) as usize {
        result.push('\n');
        line_length = 0;
    } else if line_length > 0 {
        result.push(' ');
        line_length += 1;
    }

    result.push_str(&color_channel.to_string());
    line_length += color_channel_string_length;
    (result, line_length)
}

fn build_ppm_string_from_pixels(pixels: &[Color], width: &usize) -> (String, usize) {
    let srgb_pixels = pixels
        .into_par_iter()
        .flat_map_iter(|color| {
            color
                .into_array()
                .iter()
                .map(|channel| scale_to_srgb(*channel))
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<u8>>();

    // each row of pixels is now 3X as long after flattening the color channels
    let flattened_width: usize = width * 3;

    srgb_pixels.iter().enumerate().fold(
        (String::new(), 0),
        |(mut body, mut line_length), (index, color_channel)| {
            if index % flattened_width == 0 {
                body.push('\n');
                line_length = 0;
            }

            let (channel_as_string, new_line_length) =
                convert_srgb_channel_to_ppm_string(color_channel, line_length);
            body.push_str(&channel_as_string);
            (body, new_line_length)
        },
    )
}

pub fn canvas_to_ppm(canvas: &Canvas) -> String {
    let mut ppm_file = format!(
        "{ppm_version}\n{width} {height}\n{rgb_value_limit}",
        ppm_version = PPM_VERSION,
        width = canvas.width,
        height = canvas.height,
        rgb_value_limit = MAX_SRGB_VALUE,
    );

    let (body, _) = build_ppm_string_from_pixels(&canvas.pixels, &canvas.width);

    ppm_file.push_str(&body);

    // ensure file ends with a line break for compatibility with all PPM3 readers
    ppm_file.push('\n');

    ppm_file
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_srgb_scaling_returns_correct_value() {
        assert_eq!(scale_to_srgb(Float::from(0.5)), 128);
        assert_eq!(scale_to_srgb(Float::from(0.15)), 38);
        assert_eq!(scale_to_srgb(Float::from(0.7)), 179);
        assert_eq!(scale_to_srgb(Float::from(0.91)), 232);
    }

    #[test]
    fn test_srgb_scaling_clamps_out_of_range_values() {
        assert_eq!(scale_to_srgb(Float::from(-5.0)), 0);
        assert_eq!(scale_to_srgb(Float::from(2.5)), 255);
        assert_eq!(scale_to_srgb(Float::from(-0.01)), 0);
        assert_eq!(scale_to_srgb(Float::from(1.01)), 255);
    }
}
