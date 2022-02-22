use std::fs;
use std::io::{Error, Write};

use ray_tracer::geometry::{normalize, point, vector, Point, Vector};
use ray_tracer::graphics::{canvas, canvas_to_ppm, color, write_pixel, Canvas};
use ray_tracer::Float;

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

/// draw projectile simulation on a canvas
fn render_projectile_simulation() -> Canvas {
    let mut projectile = Projectile {
        position: point(0.0, 1.0, 0.0),
        velocity: normalize(vector(1.0, 1.8, 0.0)) * Float::from(11.25),
    };

    let env = Environment {
        gravity: vector(0.0, -0.1, 0.0),
        wind: vector(-0.01, 0.0, 0.0),
    };

    let color = color(255.0, 0.0, 0.0);
    let mut canvas = canvas(900, 550);
    let ground = canvas.height;
    write_pixel(
        &mut canvas,
        projectile.position.x.to_number() as usize,
        ground - projectile.position.y.to_number() as usize,
        color,
    );

    while projectile.position.y > 0.0 {
        projectile = tick(env, projectile);
        write_pixel(
            &mut canvas,
            projectile.position.x.to_number() as usize,
            ground - projectile.position.y.to_number() as usize,
            color,
        );
    }

    canvas
}

fn main() -> Result<(), Error> {
    fs::create_dir(dbg!(format!("{}/docs", env!("CARGO_MANIFEST_DIR")))).unwrap_or(());

    let mut file = fs::File::create(dbg!(format!(
        "{}/docs/projectile_render.ppm",
        env!("CARGO_MANIFEST_DIR")
    )))
    .unwrap();

    let result = render_projectile_simulation();

    write!(file, "{}", canvas_to_ppm(&result))
}
