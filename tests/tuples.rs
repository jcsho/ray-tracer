use std::convert::Infallible;

use async_trait::async_trait;
use cucumber::{given, then, World, WorldInit};

use ray_tracer::tuples::{magnitude, point, vector, Float, Point, Vector};

#[derive(Debug)]
enum TupleType {
    PointTuple(Point),
    VectorTuple(Vector),
}

type Tuple<'a> = (&'a Float, &'a Float, &'a Float, Float);

fn unwrap_tuple(tuple: &TupleType) -> Tuple {
    match tuple {
        TupleType::PointTuple(Point { x, y, z }) => (x, y, z, Float::from(1.0)),
        TupleType::VectorTuple(Vector { x, y, z }) => (x, y, z, Float::from(0.0)),
    }
}

fn tuple_operation_matrix(
    tuple1: &TupleType,
    tuple2: &TupleType,
    point_to_point_operation: fn(&Point, &Point) -> TupleType,
    point_to_vector_operation: fn(&Point, &Vector) -> TupleType,
    vector_to_point_operation: fn(&Vector, &Point) -> TupleType,
    vector_to_vector_operation: fn(&Vector, &Vector) -> TupleType,
) -> TupleType {
    match tuple1 {
        TupleType::PointTuple(p1) => match tuple2 {
            TupleType::PointTuple(p2) => point_to_point_operation(p1, p2),
            TupleType::VectorTuple(v) => point_to_vector_operation(p1, v),
        },
        TupleType::VectorTuple(v1) => match tuple2 {
            TupleType::PointTuple(p) => vector_to_point_operation(v1, p),
            TupleType::VectorTuple(v2) => vector_to_vector_operation(v1, v2),
        },
    }
}

#[derive(Debug, WorldInit)]
struct TupleWorld {
    input1: Option<TupleType>,
    input2: Option<TupleType>,
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

#[given(regex = r"^a\d? ← tuple\((-?\d+.?\d*), (-?\d+.?\d*), (-?\d+.?\d*), (-?\d+.?\d*)\)$")]
fn parse_tuple_inputs(world: &mut TupleWorld, x: f64, y: f64, z: f64, w: f64) {
    let tuple: TupleType;
    if w == 0.0 {
        tuple = TupleType::VectorTuple(vector(x, y, z));
    } else if w == 1.0 {
        tuple = TupleType::PointTuple(point(x, y, z));
    } else {
        panic!("Unexpected tuple type: {}", w);
    }

    match world.input1 {
        None => world.input1 = Some(tuple),
        _ => world.input2 = Some(tuple),
    }
}

#[then(regex = r"^a.(x|y|z|w) = (-?\d+.\d+)$")]
fn assert_single_tuple_properties(world: &mut TupleWorld, property: String, expected_value: f64) {
    let tuple = world
        .input1
        .as_ref()
        .unwrap_or_else(|| panic!("Failed to construct tuple from input"));

    let (x, y, z, w) = unwrap_tuple(tuple);

    match property.as_str() {
        "x" => assert_eq!(x, &expected_value),
        "y" => assert_eq!(y, &expected_value),
        "z" => assert_eq!(z, &expected_value),
        "w" => assert_eq!(w, expected_value),
        _ => panic!("Unknown property value \"{}\"", property),
    }
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

    let tuple_type = match tuple {
        TupleType::PointTuple(_) => 1.0,
        TupleType::VectorTuple(_) => 0.0,
    };

    match compare_operator.as_str() {
        "is" => match expected_tuple_type.as_str() {
            "point" => assert_eq!(tuple_type, 1.0),
            "vector" => assert_eq!(tuple_type, 0.0),
            _ => panic!("Unknown tuple type: {}", expected_tuple_type),
        },
        "is not" => match expected_tuple_type.as_str() {
            "point" => assert_ne!(tuple_type, 1.0),
            "vector" => assert_ne!(tuple_type, 0.0),
            _ => panic!("Unknown tuple type: {}", expected_tuple_type),
        },
        _ => panic!("Unknown operator: {}", compare_operator),
    }
}

#[given(regex = r"^\w\d? ← (point|vector)\((-?\d+.?\d*), (-?\d+.?\d*), (-?\d+.?\d*)\)$")]
fn create_multiple_tuples_shortcut(
    world: &mut TupleWorld,
    tuple_type: String,
    x: f64,
    y: f64,
    z: f64,
) {
    let tuple = match tuple_type.as_str() {
        "point" => TupleType::PointTuple(point(x, y, z)),
        "vector" => TupleType::VectorTuple(vector(x, y, z)),
        str => panic!("Unexpected tuple type: {}", str),
    };

    match world.input1 {
        None => world.input1 = Some(tuple),
        _ => world.input2 = Some(tuple),
    }
}

#[then(regex = r"^\w = tuple\((-?\d+.?\d*), (-?\d+.?\d*), (-?\d+.?\d*), (-?\d+.?\d*)\)")]
fn assert_tuples_shortcut(
    world: &mut TupleWorld,
    expected_x: f64,
    expected_y: f64,
    expected_z: f64,
    expected_w: f64,
) {
    let tuple = world
        .input1
        .as_ref()
        .unwrap_or_else(|| panic!("Failed to construct tuple from input"));

    let (x, y, z, w) = unwrap_tuple(tuple);

    assert_eq!(x, &expected_x);
    assert_eq!(y, &expected_y);
    assert_eq!(z, &expected_z);
    assert_eq!(w, expected_w);
}

#[then(
    regex = r"^\w\d ([+-]) \w\d = tuple\((-?\d+.?\d*), (-?\d+.?\d*), (-?\d+.?\d*), (-?\d+.?\d*)\)$"
)]
fn assert_tuple_to_tuple_operations(
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
        "+" => tuple_operation_matrix(
            tuple1,
            tuple2,
            |_, _| panic!("Unexpected add operation. Cannot add point to point"),
            |p, v| TupleType::PointTuple(p + v),
            |v, p| TupleType::PointTuple(v + p),
            |v1, v2| TupleType::VectorTuple(v1 + v2),
        ),
        "-" => tuple_operation_matrix(
            tuple1,
            tuple2,
            |p1, p2| TupleType::VectorTuple(p1 - p2),
            |p, v| TupleType::PointTuple(p - v),
            |_, _| panic!("Unexpected sub operation. Cannot sub point from vector"),
            |v1, v2| TupleType::VectorTuple(v1 - v2),
        ),
        _ => panic!("Unexpected operator: {}", operator),
    };

    let (x, y, z, w) = unwrap_tuple(&result);

    assert_eq!(x, &expected_x);
    assert_eq!(y, &expected_y);
    assert_eq!(z, &expected_z);
    assert_eq!(w, expected_w);
}

#[then(regex = r"^\-a = tuple\((-?\d+.?\d*), (-?\d+.?\d*), (-?\d+.?\d*), (-?\d+.?\d*)\)$")]
fn assert_tuple_negation(
    world: &mut TupleWorld,
    expected_x: f64,
    expected_y: f64,
    expected_z: f64,
    expected_w: f64,
) {
    let tuple = world
        .input1
        .as_ref()
        .unwrap_or_else(|| panic!("Failed to construct tuple from input"));

    let result = match tuple {
        TupleType::PointTuple(p) => TupleType::PointTuple(-(*p)),
        TupleType::VectorTuple(v) => TupleType::VectorTuple(-(*v)),
    };

    let (x, y, z, w) = unwrap_tuple(&result);

    assert_eq!(x, &expected_x);
    assert_eq!(y, &expected_y);
    assert_eq!(z, &expected_z);
    assert_eq!(w, expected_w);
}

#[then(
    regex = r"^\w ([*/]) (-?\d+.?\d*) = tuple\((-?\d+.?\d*), (-?\d+.?\d*), (-?\d+.?\d*), (-?\d+.?\d*)\)$"
)]
fn assert_tuple_to_scalar_operations(
    world: &mut TupleWorld,
    operator: String,
    scalar_value: f64,
    expected_x: f64,
    expected_y: f64,
    expected_z: f64,
    expected_w: f64,
) {
    let tuple = world
        .input1
        .as_ref()
        .unwrap_or_else(|| panic!("Failed to construct tuple from input"));

    let result = match operator.as_str() {
        "*" => match tuple {
            TupleType::PointTuple(p) => TupleType::PointTuple(p * Float::from(scalar_value)),
            TupleType::VectorTuple(v) => TupleType::VectorTuple(v * Float::from(scalar_value)),
        },
        "/" => match tuple {
            TupleType::PointTuple(p) => TupleType::PointTuple(p / Float::from(scalar_value)),
            TupleType::VectorTuple(v) => TupleType::VectorTuple(v / Float::from(scalar_value)),
        },
        _ => panic!("Unexpected scalar operator: {}", operator),
    };

    let (x, y, z, w) = unwrap_tuple(&result);

    assert_eq!(x, &expected_x);
    assert_eq!(y, &expected_y);
    assert_eq!(z, &expected_z);
    assert_eq!(w, expected_w);
}

#[then(regex = r"^magnitude\(v\) = (\d+)$")]
fn assert_magnitude_unit_vector(world: &mut TupleWorld, expected_value: f64) {
    let tuple = world
        .input1
        .as_ref()
        .unwrap_or_else(|| panic!("Failed to construct tuple from input"));

    let result = match tuple {
        TupleType::VectorTuple(v) => magnitude(v),
        _ => panic!("Only vector tuples allowed"),
    };

    assert_eq!(result, expected_value);
}

#[then(regex = r"^magnitude\(v\) = √(\d+)$")]
fn assert_magnitude_values(world: &mut TupleWorld, expected_squared_value: f64) {
    let tuple = world
        .input1
        .as_ref()
        .unwrap_or_else(|| panic!("Failed to construct tuple from input"));

    let result = match tuple {
        TupleType::VectorTuple(v) => magnitude(v),
        _ => panic!("Only vector tuples allowed"),
    };

    let expected_value = expected_squared_value.sqrt();

    assert_eq!(result, expected_value);
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
