use std::convert::Infallible;

use async_trait::async_trait;
use cucumber::gherkin::Step;
use cucumber::{given, then, when, World, WorldInit};

use ray_tracer::graphics::{canvas, canvas_to_ppm, color, pixel_at, write_pixel, Canvas, Color};

#[derive(Debug, WorldInit)]
struct CanvasWorld {
    canvas: Option<Canvas>,
    paint_colors: Vec<Color>,
    output: Option<String>,
}

#[async_trait(?Send)]
impl World for CanvasWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            canvas: Option::None,
            paint_colors: Vec::with_capacity(3),
            output: Option::None,
        })
    }
}

#[given(regex = r"^c ← canvas\((\d+), (\d+)\)$")]
fn create_canvas(world: &mut CanvasWorld, width: usize, height: usize) {
    world.canvas = Some(canvas(width, height));
}

#[given(regex = r"^\w+\d* ← color\((-?\d+.?\d*), (-?\d+.?\d*), (-?\d+.?\d*)\)$")]
fn parse_color(world: &mut CanvasWorld, red: f64, green: f64, blue: f64) {
    world.paint_colors.push(color(red, green, blue));
}

#[when(regex = r"^write_pixel\(c, (\d+), (\d+), (\w+)\)$")]
fn when_write_pixel(world: &mut CanvasWorld, x: usize, y: usize, color: String) {
    let canvas = world
        .canvas
        .as_mut()
        .unwrap_or_else(|| panic!("Canvas not created"));

    let paint_color = match color.as_str() {
        "red" | "c1" => *world
            .paint_colors
            .first()
            .unwrap_or_else(|| panic!("Color not available")),
        "c2" => *world
            .paint_colors
            .get(1)
            .unwrap_or_else(|| panic!("Color not available")),
        "c3" => *world
            .paint_colors
            .get(2)
            .unwrap_or_else(|| panic!("Color not available")),
        _ => panic!("Unknown paint color"),
    };

    write_pixel(canvas, x, y, paint_color);
}

#[when(regex = r"^ppm ← canvas_to_ppm\(c\)$")]
fn when_write_to_ppm(world: &mut CanvasWorld) {
    let canvas = world
        .canvas
        .as_ref()
        .unwrap_or_else(|| panic!("Canvas not created"));
    world.output = Some(canvas_to_ppm(canvas));
}

#[then(regex = r"^c.(\w+) = (\d+)$")]
fn assert_canvas_size(world: &mut CanvasWorld, dimension: String, value: usize) {
    let canvas = world
        .canvas
        .as_ref()
        .unwrap_or_else(|| panic!("Canvas not created"));

    let dimension = match dimension.as_str() {
        "width" => canvas.width,
        "height" => canvas.height,
        _ => panic!("Unsupported dimension: {}", dimension),
    };

    assert_eq!(dimension, value);
}

#[then(regex = r"^every pixel of c is color\(0, 0, 0\)$")]
fn assert_default_canvas_color_is_black(world: &mut CanvasWorld) {
    let canvas = world
        .canvas
        .as_ref()
        .unwrap_or_else(|| panic!("Canvas not created"));

    canvas.pixels.iter().for_each(|pixel| {
        assert_eq!(pixel.red, 0.0);
        assert_eq!(pixel.green, 0.0);
        assert_eq!(pixel.blue, 0.0);
    });
}

#[then(regex = r"^pixel_at\(c, (\d), (\d)\) = red$")]
fn assert_pixel_painted(world: &mut CanvasWorld, x: usize, y: usize) {
    let canvas = world
        .canvas
        .as_ref()
        .unwrap_or_else(|| panic!("Canvas not created"));

    let paint_color = *world
        .paint_colors
        .get(0)
        .unwrap_or_else(|| panic!("Color not parsed correctly"));

    assert_eq!(pixel_at(canvas, x, y), paint_color);
}

#[then(regex = r"^lines 1-3 of ppm are$")]
fn assert_ppm_header_is_correct(world: &mut CanvasWorld, step: &Step) {
    let actual_ppm_output = world
        .output
        .as_ref()
        .unwrap_or_else(|| panic!("Failed to get PPM output"));

    let docstring = step
        .docstring()
        .unwrap_or_else(|| panic!("Missing docstring"));

    // workaround gherkin parsing the doc string with a
    // newline character at the beginning and the end
    let expected_ppm_header = &docstring[1..(docstring.chars().count() - 1)];

    assert!(actual_ppm_output.starts_with(expected_ppm_header));
}

#[then(regex = r"^lines 4-6 of ppm are$")]
fn assert_ppm_body_is_correct(world: &mut CanvasWorld, step: &Step) {
    let docstring = step
        .docstring()
        .unwrap_or_else(|| panic!("Missing docstring"));

    // workaround gherkin parsing the doc string with a
    // newline character at the beginning and the end
    let expected_ppm_body = &docstring[1..(docstring.chars().count() - 1)];

    let actual_ppm_output = world
        .output
        .as_ref()
        .unwrap_or_else(|| panic!("Failed to get PPM output"));

    assert!(actual_ppm_output.contains(expected_ppm_body));
}

#[then(regex = r"^ppm ends with a newline character$")]
fn assert_ppm_body_ends_with_newline(world: &mut CanvasWorld) {
    let actual_ppm_output = world
        .output
        .as_ref()
        .unwrap_or_else(|| panic!("Failed to get PPM output"));

    assert!(actual_ppm_output.ends_with('\n'));
}

fn main() {
    use cucumber::{writer, WriterExt as _};
    use std::fs;

    fs::create_dir(dbg!(format!("{}/reports", env!("CARGO_MANIFEST_DIR")))).unwrap_or(());

    let file = fs::File::create(dbg!(format!(
        "{}/reports/canvas.xml",
        env!("CARGO_MANIFEST_DIR")
    )))
    .unwrap();

    futures::executor::block_on(
        CanvasWorld::cucumber()
            .with_writer(
                writer::Basic::stdout()
                    .summarized()
                    .tee::<CanvasWorld, _>(writer::JUnit::for_tee(file, 0))
                    .normalized(),
            )
            .run("tests/features/canvas.feature"),
    );
}
