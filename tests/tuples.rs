use std::convert::Infallible;

use async_trait::async_trait;
use cucumber::{given, then, World, WorldInit};

use ray_tracer::tuples::{point, vector, Tuple};

#[derive(Debug, WorldInit)]
struct TupleWorld {
    input: Option<Tuple>,
}

#[async_trait(?Send)]
impl World for TupleWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            input: Option::None,
        })
    }
}

#[given(regex = r"^a ← tuple\((-?\d+.\d+), (-?\d+.\d+), (-?\d+.\d+), (-?\d+.\d+)\)$")]
fn parse_tuple(world: &mut TupleWorld, x: f64, y: f64, z: f64, w: f64) {
    // workarounds cucumber parses 1.0 and 0.0 as 0 and 1
    if (w as usize) == 1 {
        world.input = Some(Tuple::Point { x, y, z, w });
    } else if (w as usize) == 0 {
        world.input = Some(Tuple::Vector { x, y, z, w });
    } else {
        panic!("Unknown `w` value: {}", w);
    }
}

#[then(regex = r"^a.(x|y|z|w) = (-?\d+.\d+)$")]
fn assert_tuple_values(world: &mut TupleWorld, property: String, expected_value: f64) {
    let tuple = world
        .input
        .as_ref()
        .unwrap_or_else(|| panic!("Failed to construct tuple from input"));

    let actual_value = match tuple {
        Tuple::Point { x, y, z, w } => match property.as_str() {
            "x" => x,
            "y" => y,
            "z" => z,
            "w" => w,
            _ => panic!("Unknown property value \"{}\"", property),
        },
        Tuple::Vector { x, y, z, w } => match property.as_str() {
            "x" => x,
            "y" => y,
            "z" => z,
            "w" => w,
            _ => panic!("Unknown property value \"{}\"", property),
        },
    };

    assert_eq!(actual_value, &expected_value);
}

#[then(regex = r"^a (is|is not) a (point|vector)$")]
fn assert_tuple_type(
    world: &mut TupleWorld,
    compare_operator: String,
    expected_tuple_type: String,
) {
    let tuple = world
        .input
        .as_ref()
        .unwrap_or_else(|| panic!("Failed to construct tuple from input"));

    match compare_operator.as_str() {
        "is" => match expected_tuple_type.as_str() {
            "point" => assert_eq!(tuple, &point(0.0, 0.0, 0.0)),
            "vector" => assert_eq!(tuple, &vector(0.0, 0.0, 0.0)),
            _ => panic!("Unknown tuple type: {}", expected_tuple_type),
        },
        "is not" => match expected_tuple_type.as_str() {
            "point" => assert_ne!(tuple, &point(0.0, 0.0, 0.0)),
            "vector" => assert_ne!(tuple, &vector(0.0, 0.0, 0.0)),
            _ => panic!("Unknown tuple type: {}", expected_tuple_type),
        },
        _ => panic!("Unknown operator: {}", compare_operator),
    }
}

#[given(regex = r"^(?:p|v) ← (point|vector)\((-?\d+.?\d?), (-?\d+.?\d?), (-?\d+.?\d?)\)$")]
fn create_tuples_shortcut(world: &mut TupleWorld, tuple_type: String, x: f64, y: f64, z: f64) {
    match tuple_type.as_str() {
        "point" => world.input = Some(point(x, y, z)),
        "vector" => world.input = Some(vector(x, y, z)),
        _ => panic!("Unknown tuple type: {}", tuple_type),
    }
}

#[then(regex = r"^(p|v) = tuple\((-?\d+.?\d?), (-?\d+.?\d?), (-?\d+.?\d?), (-?\d+.?\d?)\)")]
fn assert_tuples_shortcut(
    world: &mut TupleWorld,
    tuple_type: String,
    expected_x: f64,
    expected_y: f64,
    expected_z: f64,
    expected_w: f64,
) {
    let tuple = world
        .input
        .as_ref()
        .unwrap_or_else(|| panic!("Failed to construct tuple from input"));

    match tuple_type.as_str() {
        "p" => match tuple {
            Tuple::Point { x, y, z, w } => {
                assert_eq!(x, &expected_x);
                assert_eq!(y, &expected_y);
                assert_eq!(z, &expected_z);
                assert_eq!(w, &expected_w);
            }
            Tuple::Vector { .. } => panic!("Incorrect tuple type. Expected Point but found Vector"),
        },
        "v" => match tuple {
            Tuple::Point { .. } => panic!("Incorrect tuple type. Expected Vector but found Point"),
            Tuple::Vector { x, y, z, w } => {
                assert_eq!(x, &expected_x);
                assert_eq!(y, &expected_y);
                assert_eq!(z, &expected_z);
                assert_eq!(w, &expected_w);
            }
        },
        _ => panic!("Unknown tuple type: {}", tuple_type),
    }
}

fn main() {
    use cucumber::writer;
    use std::fs;

    fs::create_dir(dbg!(format!("{}/reports", env!("CARGO_MANIFEST_DIR")))).unwrap_or(());

    let file = fs::File::create(dbg!(format!(
        "{}/reports/tuples.xml",
        env!("CARGO_MANIFEST_DIR")
    )))
    .unwrap();

    futures::executor::block_on(
        TupleWorld::cucumber()
            .with_writer(writer::JUnit::new(file, 0))
            .run("tests/features/tuples.feature"),
    );
}
