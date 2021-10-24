use lab_raytracing_rs::camera::Camera;
use lab_raytracing_rs::groups::Group;
use lab_raytracing_rs::lights::Pointlight;
use lab_raytracing_rs::objects::cylinder;
use lab_raytracing_rs::objects::default_sphere;
use lab_raytracing_rs::objects::Object;
use lab_raytracing_rs::transformations::rotation_y;
use lab_raytracing_rs::transformations::rotation_z;
use lab_raytracing_rs::transformations::scaling;
use lab_raytracing_rs::transformations::translation;
use lab_raytracing_rs::transformations::view_transform;
use lab_raytracing_rs::tuples::color;
use lab_raytracing_rs::tuples::point;
use lab_raytracing_rs::tuples::vector;
use lab_raytracing_rs::world::World;
use std::f64::consts::PI;
use std::io;

fn main() -> io::Result<()> {
    let white_color = color(1.0, 1.0, 1.0);

    let mut world = World::default();

    let hexagon = hexagon();
    world.add_group(hexagon);

    let mut camera = Camera::new(1600, 900, PI / 3.0);
    camera.set_transform(view_transform(
        &point(-3.0, 3.0, -3.0),
        &point(0.0, 0.0, 0.0),
        &vector(0.0, 1.0, 0.0),
    ));
    world.light = Some(Pointlight::new(point(20.0, 20.0, -20.0), white_color));

    let canvas = camera.render(&world);

    let file = &mut io::stdout();
    let writer = &mut io::BufWriter::with_capacity(1024 * 128, file);
    canvas.ppm(writer)?;

    Ok(())
}

fn hexagon() -> Group {
    let mut hex = Group::default();
    for n in 0..=5 {
        let mut side = hexagon_side();
        side.set_transform(rotation_y(n as f64 * std::f64::consts::PI / 3.0));
        hex.add_group(side);
    }
    hex
}

fn hexagon_side() -> Group {
    let mut side = Group::default();
    side.add_object(hexagon_corner());
    side.add_object(hexagon_edge());
    side
}

fn hexagon_corner() -> Object {
    let mut corner = default_sphere();
    corner.set_transform(translation(0.0, 0.0, -1.0) * scaling(0.25, 0.25, 0.25));
    corner
}

fn hexagon_edge() -> Object {
    let mut edge = cylinder(0.0, 1.0);
    edge.set_transform(
        translation(0.0, 0.0, -1.0)
            * rotation_y(-std::f64::consts::PI / 6.0)
            * rotation_z(-std::f64::consts::PI / 2.0)
            * scaling(0.25, 1.0, 0.25),
    );
    edge
}
