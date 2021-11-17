use lab_raytracing_rs::{
    camera::Camera,
    groups::Group,
    lights::Pointlight,
    obj_file::Parser,
    objects::default_plane,
    patterns::solid_pattern,
    transformations::{rotation_x, rotation_z, translation, view_transform},
    tuples::{color, point, vector},
    world::World,
};
use std::{f64::consts::PI, fs, io};

fn main() -> io::Result<()> {
    let mut world = World::default();

    let mut floor = default_plane();
    floor.material.pattern = Some(Box::new(solid_pattern(color(
        212.0 / 250.0,
        24.0 / 250.0,
        29.0 / 250.0,
    ))));
    floor.material.specular = 0.0;
    world.add_object(floor);

    let mut wall = default_plane();
    wall.material.pattern = Some(Box::new(solid_pattern(color(
        33.0 / 250.0,
        66.0 / 250.0,
        171.0 / 250.0,
    ))));
    wall.material.specular = 0.0;
    wall.set_transform(translation(0.0, 0.0, 10.0) * rotation_x(PI / 2.0));
    world.add_object(wall);

    let mut wall = default_plane();
    wall.material.pattern = Some(Box::new(solid_pattern(color(
        27.0 / 250.0,
        121.0 / 250.0,
        49.0 / 250.0,
    ))));
    wall.material.specular = 0.0;
    wall.set_transform(translation(9.0, 0.0, 0.0) * rotation_z(PI / 2.0));
    world.add_object(wall);

    let mut teapot = load_obj_file("examples/teapot.obj");
    teapot.set_color(&color(255.0 / 250.0, 215.0 / 250.0, 0.0 / 250.0));
    world.add_group(teapot);

    world.light = Some(Pointlight::new(
        point(-11.0, 3.0, -10.0),
        color(1.0, 1.0, 1.0),
    ));

    let mut camera = Camera::new(1600, 900, PI / 3.0);
    camera.set_transform(view_transform(
        &point(-5.0, 5.0, -10.0),
        &point(0.0, 1.0, 0.0),
        &vector(0.0, 1.0, 0.0),
    ));

    let canvas = camera.render(&world);

    let file = &mut io::stdout();
    let writer = &mut io::BufWriter::with_capacity(1024 * 128, file);
    canvas.ppm(writer)?;

    Ok(())
}

fn load_obj_file(path: &str) -> Group {
    let content = fs::read_to_string(path).expect("could not read file");
    Parser::parse_obj_file(content.as_str()).to_group()
}
