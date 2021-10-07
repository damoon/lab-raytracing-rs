use lab_raytracing_rs::canvas::Canvas;
use lab_raytracing_rs::intersections::hit;
use lab_raytracing_rs::lights::lighting;
use lab_raytracing_rs::lights::Pointlight;
use lab_raytracing_rs::rays::Ray;
use lab_raytracing_rs::spheres::default_sphere;
use lab_raytracing_rs::tuples::color;
use lab_raytracing_rs::tuples::point;
use std::io;

fn main() -> io::Result<()> {
    let black = color(0.1, 0.1, 0.1);

    let ray_origin = point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 500;
    let pixel_size = wall_size / canvas_pixels as f64;
    let half_wall_size = wall_size / 2.0;
    let half_pixel_size = pixel_size / 2.0;

    let mut shape = default_sphere();
    shape.material.color = color(1.0, 0.2, 1.0);

    let light_position = point(-10.0, 10.0, -10.0);
    let light_color = color(1.0, 1.0, 1.0);
    let light = Pointlight::new(light_position, light_color);

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    canvas.fill(black);

    for y in 0..canvas_pixels {
        let world_y = half_wall_size - (pixel_size * y as f64) - half_pixel_size;
        for x in 0..canvas_pixels {
            let world_x = -half_wall_size + (pixel_size * x as f64) + half_pixel_size;

            let position = point(world_x, world_y, wall_z);
            let ray = Ray::new(ray_origin, (position - ray_origin).normalize());
            let xs = shape.intersect(&ray);
            let hit = hit(&xs);
            if let Some(hit) = hit {
                let world_point = ray.position(hit.t);
                let normal = hit.object.normal_at(&world_point);
                let eye = -ray.direction;
                let color = lighting(
                    &hit.object.material,
                    &shape,
                    &light,
                    &world_point,
                    &eye,
                    &normal,
                    false,
                );
                canvas.set(x, y, color);
            };
        }
    }

    let file = &mut io::stdout();
    let writer = &mut io::BufWriter::with_capacity(1024 * 128, file);
    canvas.ppm(writer)?;

    Ok(())
}
