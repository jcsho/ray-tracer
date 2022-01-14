use std::convert::Infallible;

use async_trait::async_trait;
use cucumber::{given, then, World, WorldInit};

use ray_tracer::tuples::{point, vector, Tuple, TupleData};

#[derive(Debug, WorldInit)]
struct TupleWorld {
    input1: Option<Tuple>,
    input2: Option<Tuple>,
}

#[async_trait(?Send)]
impl World for TupleWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            input1: Option::None,
            input2: Option::None,
        })
    }
}

#[given(regex = r"^a ← tuple\((-?\d+.?\d*), (-?\d+.?\d*), (-?\d+.?\d*), (-?\d+.?\d*)\)$")]
fn parse_single_input(world: &mut TupleWorld, x: f64, y: f64, z: f64, w: f64) {
    // workarounds cucumber parses 1.0 and 0.0 as 0 and 1
    if (w as usize) == 1 {
        world.input1 = Some(point(x, y, z));
    } else if (w as usize) == 0 {
        world.input1 = Some(vector(x, y, z));
    } else {
        panic!("Unknown `w` value: {}", w);
    }
}

#[given(regex = r"^a(\d) ← tuple\((-?\d+.?\d*), (-?\d+.?\d*), (-?\d+.?\d*), (-?\d+.?\d*)\)$")]
fn parse_multiple_inputs(
    world: &mut TupleWorld,
    num: usize,
    x: isize,
    y: isize,
    z: isize,
    w: isize,
) {
    let tuple: Tuple;
    // workarounds cucumber parses 1.0 and 0.0 as 0 and 1
    if (w as usize) == 1 {
        tuple = point(x as f64, y as f64, z as f64);
    } else if (w as usize) == 0 {
        tuple = vector(x as f64, y as f64, z as f64);
    } else {
        panic!("Unknown `w` value: {}", w);
    }

    match num {
        1 => world.input1 = Some(tuple),
        2 => world.input2 = Some(tuple),
        _ => panic!("Unsupported input value"),
    }
}

#[then(regex = r"^a.(x|y|z|w) = (-?\d+.\d+)$")]
fn assert_tuple_values(world: &mut TupleWorld, property: String, expected_value: f64) {
    let tuple = world
        .input1
        .as_ref()
        .unwrap_or_else(|| panic!("Failed to construct tuple from input"));

    let actual_value = match tuple {
        Tuple::Point(td) | Tuple::Vector(td) => match property.as_str() {
            "x" => &td.x,
            "y" => &td.y,
            "z" => &td.z,
            "w" => &td.w,
            _ => panic!("Unknown property value \"{}\"", property),
        },
    };

    assert_eq!(*actual_value, expected_value);
}

#[then(regex = r"^a (is|is not) a (point|vector)$")]
fn assert_tuple_type(
    world: &mut TupleWorld,
    compare_operator: String,
    expected_tuple_type: String,
) {
    let tuple = world
        .input1
        .as_ref()
        .unwrap_or_else(|| panic!("Failed to construct tuple from input"));

    match compare_operator.as_str() {
        "is" => match expected_tuple_type.as_str() {
            "point" => assert_eq!(*tuple, point(0.0, 0.0, 0.0)),
            "vector" => assert_eq!(*tuple, vector(0.0, 0.0, 0.0)),
            _ => panic!("Unknown tuple type: {}", expected_tuple_type),
        },
        "is not" => match expected_tuple_type.as_str() {
            "point" => assert_ne!(*tuple, point(0.0, 0.0, 0.0)),
            "vector" => assert_ne!(*tuple, vector(0.0, 0.0, 0.0)),
            _ => panic!("Unknown tuple type: {}", expected_tuple_type),
        },
        _ => panic!("Unknown operator: {}", compare_operator),
    }
}

#[given(regex = r"^(?:p|v) ← (point|vector)\((-?\d+.?\d*), (-?\d+.?\d*), (-?\d+.?\d*)\)$")]
fn create_one_tuple_shortcut(world: &mut TupleWorld, tuple_type: String, x: f64, y: f64, z: f64) {
    match tuple_type.as_str() {
        "point" => world.input1 = Some(point(x, y, z)),
        "vector" => world.input1 = Some(vector(x, y, z)),
        _ => panic!("Unknown tuple type: {}", tuple_type),
    }
}

#[given(regex = r"^t(\d) ← (point|vector)\((-?\d+.?\d*), (-?\d+.?\d*), (-?\d+.?\d*)\)$")]
fn create_multiple_tuples_shortcut(
    world: &mut TupleWorld,
    num: usize,
    tuple_type: String,
    x: f64,
    y: f64,
    z: f64,
) {
    let tuple = match tuple_type.as_str() {
        "point" => Some(point(x, y, z)),
        "vector" => Some(vector(x, y, z)),
        _ => panic!("Unknown tuple type: {}", tuple_type),
    };

    match num {
        1 => world.input1 = tuple,
        2 => world.input2 = tuple,
        _ => panic!("Unsupported number value"),
    }
}

#[then(regex = r"^(p|v) = tuple\((-?\d+.?\d*), (-?\d+.?\d*), (-?\d+.?\d*), (-?\d+.?\d*)\)")]
fn assert_tuples_shortcut(
    world: &mut TupleWorld,
    tuple_type: String,
    expected_x: f64,
    expected_y: f64,
    expected_z: f64,
    expected_w: f64,
) {
    let tuple = world
        .input1
        .as_ref()
        .unwrap_or_else(|| panic!("Failed to construct tuple from input"));

    let TupleData { x, y, z, w } = match tuple_type.as_str() {
        "p" => match tuple {
            Tuple::Point(td) => td,
            _ => panic!("Incorrect tuple type. Expected Point but found Vector"),
        },
        "v" => match tuple {
            Tuple::Vector(td) => td,
            _ => panic!("Incorrect tuple type. Expected Vector but found Point"),
        },
        _ => panic!("Unknown tuple type: {}", tuple_type),
    };

    assert_eq!(x, &expected_x);
    assert_eq!(y, &expected_y);
    assert_eq!(z, &expected_z);
    assert_eq!(w, &expected_w);
}

#[then(
    regex = r"^\w\d ([+-]) \w\d = tuple\((-?\d+.?\d*), (-?\d+.?\d*), (-?\d+.?\d*), (-?\d+.?\d*)\)$"
)]
fn assert_tuples_operation(
    world: &mut TupleWorld,
    operator: String,
    expected_x: f64,
    expected_y: f64,
    expected_z: f64,
    expected_w: f64,
) {
    let tuple1 = world
        .input1
        .as_ref()
        .unwrap_or_else(|| panic!("Failed to construct tuple from input"));

    let tuple2 = world
        .input2
        .as_ref()
        .unwrap_or_else(|| panic!("Failed to construct tuple from input"));

    let result = match operator.as_str() {
        "+" => tuple1 + tuple2,
        "-" => tuple1 - tuple2,
        _ => panic!("Unexpected operator: {}", operator),
    };

    match result {
        Ok(t) => match t {
            Tuple::Point(td) | Tuple::Vector(td) => {
                assert_eq!(td.x, expected_x);
                assert_eq!(td.y, expected_y);
                assert_eq!(td.z, expected_z);
                assert_eq!(td.w, expected_w);
            }
        },
        _ => panic!("Unexpected operation"),
    }
}

fn main() {
    use cucumber::{writer, WriterExt as _};
    use std::fs;

    fs::create_dir(dbg!(format!("{}/reports", env!("CARGO_MANIFEST_DIR")))).unwrap_or(());

    let file = fs::File::create(dbg!(format!(
        "{}/reports/tuples.xml",
        env!("CARGO_MANIFEST_DIR")
    )))
    .unwrap();

    futures::executor::block_on(
        TupleWorld::cucumber()
            .with_writer(
                writer::Basic::stdout()
                    .summarized()
                    .tee::<TupleWorld, _>(writer::JUnit::for_tee(file, 0))
                    .normalized(),
            )
            .run("tests/features/tuples.feature"),
    );
}
