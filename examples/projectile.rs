use lab_raytracing_rs::tuples::{point, vector, Tuple};
use std::fmt;

fn main() {
    let mut p = Projectile {
        position: point(0.0, 1.0, 0.0),
        velocity: vector(1.0, 1.0, 0.0).normalize(),
    };

    let env = Environment {
        gravity: vector(0.0, -0.1, 0.0),
        wind: vector(-0.01, 0.0, 0.0),
    };

    println!("{}", p);

    while p.position.y > 0.0 {
        p.tick(&env);
        println!("{}", p);
    }
}

struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

impl fmt::Display for Projectile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "position: {}, velocity: {}",
            self.position, self.velocity
        )
    }
}

struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

impl Projectile {
    fn tick(&mut self, env: &Environment) {
        self.position = self.position + self.velocity;
        self.velocity = self.velocity + env.gravity + env.wind;
    }
}
