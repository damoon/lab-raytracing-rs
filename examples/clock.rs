use lab_raytracing_rs::transformations::rotation_z;
use lab_raytracing_rs::{colors::color, transformations::translation};
use lab_raytracing_rs::canvas::Canvas;
use lab_raytracing_rs::tuples::point;
use std::io::{self, Write};
use std::f64::consts::PI;

fn main() -> io::Result<()> {
    let black = color(0.1, 0.1, 0.1);
    let white = color(1.0, 1.0, 1.0);

    let mut c = Canvas::new(100, 100);
    c.fill(black);

    for i in 0..12 {
        let point = point(0.0, 0.0, 0.0)
            * translation(0.0,-0.4 * c.width as f64, 0.0)
            * rotation_z(i as f64 * PI/6.0)
            * translation(0.5 * c.width as f64, 0.5 * c.width as f64, 0.0);
        c.set(point.x as usize, point.y as usize, white);
    }

    io::stdout().write_all(c.ppm().as_bytes())?;

    Ok(())
}
