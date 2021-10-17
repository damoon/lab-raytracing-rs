use lab_raytracing_rs::camera::Camera;
use lab_raytracing_rs::lights::Pointlight;
use lab_raytracing_rs::materials::Material;
use lab_raytracing_rs::shapes::default_plane;
use lab_raytracing_rs::shapes::default_sphere;
use lab_raytracing_rs::transformations::rotation_x;
use lab_raytracing_rs::transformations::rotation_y;
use lab_raytracing_rs::transformations::scaling;
use lab_raytracing_rs::transformations::translation;
use lab_raytracing_rs::transformations::view_transform;
use lab_raytracing_rs::tuples::color;
use lab_raytracing_rs::tuples::point;
use lab_raytracing_rs::tuples::vector;
use lab_raytracing_rs::world::World;
use std::env;
use std::f64::consts::PI;
use std::io;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut world = World::default();
    world.light = Some(Pointlight::new(
        point(-10.0, 10.0, -10.0),
        color(1.0, 1.0, 1.0),
    ));

    let mut background_material = Material::default();
    background_material.color = color(1.0, 0.9, 0.9);
    background_material.specular = 0.0;

    let mut floor = default_plane();
    floor.material = background_material.clone();
    world.add_object(floor);

    let mut left_wall = default_plane();
    left_wall.material = background_material.clone();
    left_wall
        .set_transform(translation(0.0, 0.0, 5.0) * rotation_y(-PI / 4.0) * rotation_x(PI / 2.0));
    world.add_object(left_wall);

    let mut right_wall = default_plane();
    right_wall.material = background_material;
    right_wall
        .set_transform(translation(0.0, 0.0, 5.0) * rotation_y(PI / 4.0) * rotation_x(PI / 2.0));
    world.add_object(right_wall);

    let mut middle = default_sphere();
    middle.set_transform(translation(-0.5, 1.0, 0.5));
    middle.material = Material::default();
    middle.material.color = color(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;
    world.add_object(middle);

    let mut right = default_sphere();
    right.set_transform(translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5));
    right.material = Material::default();
    right.material.color = color(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;
    world.add_object(right);

    let mut left = default_sphere();
    left.set_transform(translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33));
    left.material = Material::default();
    left.material.color = color(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;
    world.add_object(left);

    let mut camera = Camera::new(800, 800, PI / 3.0);
    camera.set_transform(match args[1].as_str() {
        "1" => view_transform(
            &point(0.0, 1.5, -5.0),
            &point(0.0, 1.0, 0.0),
            &vector(0.0, 1.0, 0.0),
        ),
        "2" => view_transform(
            &point(-3.0, 4.0, -3.0),
            &point(-0.0, 1.0, 0.0),
            &vector(0.0, 1.0, 0.0),
        ),
        _ => view_transform(
            &point(2.0, 1.0, -4.0),
            &point(0.0, 1.0, 0.0),
            &vector(-1.0, 1.0, 0.0),
        ),
    });

    let canvas = camera.render(&world);

    let file = &mut io::stdout();
    let writer = &mut io::BufWriter::with_capacity(1024 * 128, file);
    canvas.ppm(writer)?;

    Ok(())
}
