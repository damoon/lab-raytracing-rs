use lab_raytracing_rs::camera::render;
use lab_raytracing_rs::camera::Camera;
use lab_raytracing_rs::lights::Pointlight;
use lab_raytracing_rs::materials::Material;
use lab_raytracing_rs::planes::default_plane;
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
use std::f64::consts::PI;
use std::io;

fn main() -> io::Result<()> {
    let mut world = World::default();
    world.light = Some(Pointlight::new(
        point(-2.0, 8.0, -2.0),
        color(1.0, 1.0, 1.0),
    ));

    let mut camera = Camera::new(800, 800, PI / 4.0);
    camera.set_transform(view_transform(
        &point(2.0, 8.0, 2.0),
        &point(0.0, 0.0, 0.0),
        &vector(0.0, 1.0, 0.0),
    ));

    let mut middle = default_sphere();
    middle.set_transform(translation(-0.5, 1.0, 0.5));
    middle.material = Material::default();
    middle.material.color = color(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;
    world.objects.push(middle);

    let mut right = default_sphere();
    right.set_transform(translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5));
    right.material = Material::default();
    right.material.color = color(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;
    world.objects.push(right);

    let mut left = default_sphere();
    left.set_transform(translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33));
    left.material = Material::default();
    left.material.color = color(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;
    world.objects.push(left);

    let mut floor = default_plane();
    floor.material = Material::default();
    floor.material.color = color(1.0, 0.9, 0.9);
    floor.material.specular = 0.0;
    world.objects.push(floor);

    let mut ceiling = default_plane();
    ceiling.set_transform(translation(0.0, 10.0, 0.0));
    ceiling.material = Material::default();
    ceiling.material.color = color(1.0, 0.9, 0.9);
    ceiling.material.specular = 0.0;
    world.objects.push(ceiling);

    for i in 0..6 {
        let f = i as f64;
        let mut wall = default_plane();
        wall.set_transform(
            rotation_y(f * PI / 3.0) * translation(0.0, 0.0, 5.0) * rotation_x(PI / 2.0),
        );
        wall.material = Material::default();
        wall.material.color = color(0.9 - f / 10.0, 0.9 - f / 10.0, 0.9 - f / 10.0);
        wall.material.specular = 0.0;
        world.objects.push(wall);
    }

    let canvas = render(&camera, &world);

    let file = &mut io::stdout();
    let writer = &mut io::BufWriter::with_capacity(1024 * 128, file);
    canvas.ppm(writer)?;

    Ok(())
}
