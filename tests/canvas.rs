use std::convert::Infallible;

use async_trait::async_trait;
use cucumber::{given, then, when, World, WorldInit};

use ray_tracer::graphics::{canvas, color, pixel_at, write_pixel, Canvas, Color};

#[derive(Debug, WorldInit)]
struct CanvasWorld {
    canvas: Option<Canvas>,
    paint_color: Option<Color>,
}

#[async_trait(?Send)]
impl World for CanvasWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            canvas: Option::None,
            paint_color: Option::None,
        })
    }
}

#[given(regex = r"^c ← canvas\((\d+), (\d+)\)$")]
fn create_canvas(world: &mut CanvasWorld, width: usize, height: usize) {
    world.canvas = Some(canvas(width, height));
}

#[given(regex = r"^\w+ ← color\((\d+), (\d+), (\d+)\)$")]
fn parse_color(world: &mut CanvasWorld, red: f64, green: f64, blue: f64) {
    world.paint_color = Some(color(red, green, blue));
}

#[when(regex = r"^write_pixel\(c, (\d+), (\d+), \w+\)$")]
fn paint_pixel(world: &mut CanvasWorld, x: usize, y: usize) {
    let canvas = world
        .canvas
        .as_mut()
        .unwrap_or_else(|| panic!("Canvas not created"));

    let paint_color = world
        .paint_color
        .unwrap_or_else(|| panic!("Color not parsed correctly"));

    write_pixel(canvas, x, y, paint_color);
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

    let paint_color = world
        .paint_color
        .unwrap_or_else(|| panic!("Color not parsed correctly"));

    assert_eq!(pixel_at(canvas, x, y), paint_color);
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
