use std::convert::Infallible;

use async_trait::async_trait;
use cucumber::{given, then, World, WorldInit};

use ray_tracer::graphics::{canvas, Canvas};

#[derive(Debug, WorldInit)]
struct CanvasWorld {
    canvas: Option<Canvas>,
}

#[async_trait(?Send)]
impl World for CanvasWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            canvas: Option::None,
        })
    }
}

#[given(regex = r"^c ← canvas\((\d+), (\d+)\)$")]
fn create_canvas(world: &mut CanvasWorld, width: usize, height: usize) {
    world.canvas = Some(canvas(width, height));
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
