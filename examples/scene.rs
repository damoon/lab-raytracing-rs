use lab_raytracing_rs::camera::render;
use lab_raytracing_rs::camera::Camera;
use lab_raytracing_rs::lights::Pointlight;
use lab_raytracing_rs::materials::Material;
use lab_raytracing_rs::spheres::default_sphere;
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

    let mut floor = default_sphere();
    floor.set_transform(scaling(10.0, 0.01, 10.0));
    floor.material = Material::default();
    floor.material.color = color(1.0, 0.9, 0.9);
    floor.material.specular = 0.0;

    let mut left_wall = default_sphere();
    left_wall.set_transform(
        translation(0.0, 0.0, 5.0)
            * rotation_y(-PI / 4.0)
            * rotation_x(PI / 2.0)
            * scaling(10.0, 0.01, 10.0),
    );
    left_wall.material = floor.material.clone();

    let mut right_wall = default_sphere();
    right_wall.set_transform(
        translation(0.0, 0.0, 5.0)
            * rotation_y(PI / 4.0)
            * rotation_x(PI / 2.0)
            * scaling(10.0, 0.01, 10.0),
    );
    right_wall.material = floor.material.clone();

    let mut middle = default_sphere();
    middle.set_transform(translation(-0.5, 1.0, 0.5));
    middle.material = Material::default();
    middle.material.color = color(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    let mut right = default_sphere();
    right.set_transform(translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5));
    right.material = Material::default();
    right.material.color = color(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    let mut left = default_sphere();
    left.set_transform(translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33));
    left.material = Material::default();
    left.material.color = color(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    let mut world = World::default();
    world.light = Some(Pointlight::new(
        point(-10.0, 10.0, -10.0),
        color(1.0, 1.0, 1.0),
    ));
    world.objects = vec![floor, left_wall, right_wall, middle, right, left];

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

    let canvas = render(&camera, &world);

    let file = &mut io::stdout();
    let writer = &mut io::BufWriter::with_capacity(1024 * 128, file);
    canvas.ppm(writer)?;

    Ok(())
}
