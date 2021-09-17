use lab_raytracing_rs::canvas::Canvas;
use lab_raytracing_rs::colors::color;
use lab_raytracing_rs::tuples::{Tuple, point, vector};
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut p = Projectile{
        position: point(0.0,1.0,0.0),
        velocity: vector(1.0,1.8,0.0).normalize() * 11.25,
    };

    let env = Environment {
        gravity: vector(0.0, -0.1, 0.0),
        wind: vector(-0.01, 0.0, 0.0),
    };

    let black = color(0.1, 0.1, 0.1);
    let white = color(1.0, 1.0, 1.0);

    let mut c = Canvas::new(900, 500);
    c.fill(black);

    while p.position.y > 0.0 {
        p.tick(&env);
        let w = p.position.x as i32;
        let h = 500 - (p.position.y as i32);
        if (0..900).contains(&w) && (0..500).contains(&h) {
            c.set(w as usize, h as usize, white);
        }
    }

    io::stdout().write_all(c.ppm().as_bytes())?;

    Ok(())
}

struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

impl Projectile {
    fn tick(&mut self, env: &Environment) {
        self.position = &self.position + &self.velocity;
        self.velocity = &self.velocity + &env.gravity + &env.wind;
    }
}
