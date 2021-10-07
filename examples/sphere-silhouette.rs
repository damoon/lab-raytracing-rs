use lab_raytracing_rs::canvas::Canvas;
use lab_raytracing_rs::intersections::hit;
use lab_raytracing_rs::rays::Ray;
use lab_raytracing_rs::spheres::default_sphere;
use lab_raytracing_rs::tuples::color;
use lab_raytracing_rs::tuples::point;
use std::io;

fn main() -> io::Result<()> {
    let black = color(0.1, 0.1, 0.1);
    let red = color(1.0, 0.0, 0.0);

    let ray_origin = point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 500;
    let pixel_size = wall_size / canvas_pixels as f64;
    let half_wall_size = wall_size / 2.0;
    let half_pixel_size = pixel_size / 2.0;
    let shape = default_sphere();

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    canvas.fill(black);

    for y in 0..canvas_pixels {
        let world_y = half_wall_size - (pixel_size * y as f64) - half_pixel_size;
        for x in 0..canvas_pixels {
            let world_x = -half_wall_size + (pixel_size * x as f64) + half_pixel_size;

            let position = point(world_x, world_y, wall_z);
            let r = Ray::new(ray_origin, (position - ray_origin).normalize());
            let xs = shape.intersect(&r);
            let hit = hit(&xs);
            if hit.is_some() {
                canvas.set(x, y, red);
            };
        }
    }

    let file = &mut io::stdout();
    let writer = &mut io::BufWriter::with_capacity(1024 * 128, file);
    canvas.ppm(writer)?;

    Ok(())
}
