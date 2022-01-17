use rayon::prelude::*;

use ray_tracer::tuples::{normalize, point, vector, Float, Point, Vector};

#[derive(Copy, Clone, Debug)]
struct Projectile {
    position: Point,
    velocity: Vector,
}

#[derive(Copy, Clone, Debug)]
struct Environment {
    gravity: Vector,
    wind: Vector,
}

fn tick(env: Environment, projectile: Projectile) -> Projectile {
    let position = projectile.position + projectile.velocity;
    let velocity = projectile.velocity + env.gravity + env.wind;
    Projectile { position, velocity }
}

/// returns number of `ticks` required for projectile to hit ground
/// ground meaning `projectile.position.y <= 0`
fn simulate_projectile_launch(initial_velocity_multiplier: i32) -> (i32, Point) {
    let mut projectile = Projectile {
        position: point(0.0, 1.0, 0.0),
        velocity: normalize(vector(1.0, 1.0, 0.0))
            * Float::from(initial_velocity_multiplier as f64),
    };

    let env = Environment {
        gravity: vector(0.0, -0.1, 0.0),
        wind: vector(-0.01, 0.0, 0.0),
    };

    let mut num_iterations = 1;

    while projectile.position.y > 0.0 {
        projectile = tick(env, projectile);
        num_iterations += 1;
    }

    (num_iterations, projectile.position)
}

/// Run several simulations of a projectile launch to test tuple manipulation
///
/// Sample output
/// ```sh
/// Total number of iterations for projectile at initial velocity 1 to reach `y <= 0`: 18
/// Total number of iterations for projectile at initial velocity 5 to reach `y <= 0`: 73
/// Total number of iterations for projectile at initial velocity 6 to reach `y <= 0`: 88
/// Total number of iterations for projectile at initial velocity 3 to reach `y <= 0`: 45
/// Total number of iterations for projectile at initial velocity 4 to reach `y <= 0`: 59
/// Total number of iterations for projectile at initial velocity 2 to reach `y <= 0`: 31
/// Total number of iterations for projectile at initial velocity 8 to reach `y <= 0`: 116
/// Total number of iterations for projectile at initial velocity 7 to reach `y <= 0`: 102
/// Total number of iterations for projectile at initial velocity 9 to reach `y <= 0`: 130
/// ```
fn main() {
    (1..10).into_par_iter().for_each(|multiplier| {
        let (num_iterations, _final_position) = simulate_projectile_launch(multiplier);
        println!(
            "Total number of iterations for projectile at initial velocity {} to reach `y <= 0`: {}",
            multiplier,
            num_iterations,
        );

        // println!("Final position for initial velocity {} - {:#?}", multiplier, final_position);
    });
}
