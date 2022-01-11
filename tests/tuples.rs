use std::convert::Infallible;

use async_trait::async_trait;
use cucumber::{given, then, World, WorldInit};

use ray_tracer::tuples::Tuple;

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
            "point" => assert_eq!(
                tuple,
                &Tuple::Point {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                    w: 0.0
                }
            ),
            "vector" => assert_eq!(
                tuple,
                &Tuple::Vector {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                    w: 0.0
                }
            ),
            _ => panic!("Unknown tuple type: {}", expected_tuple_type),
        },
        "is not" => match expected_tuple_type.as_str() {
            "point" => assert_ne!(
                tuple,
                &Tuple::Point {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                    w: 0.0
                }
            ),
            "vector" => assert_ne!(
                tuple,
                &Tuple::Vector {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                    w: 0.0
                }
            ),
            _ => panic!("Unknown tuple type: {}", expected_tuple_type),
        },
        _ => panic!("Unknown operator: {}", compare_operator),
    }
}

fn main() {
    futures::executor::block_on(TupleWorld::run("./tests/features/tuples.feature"));
}
