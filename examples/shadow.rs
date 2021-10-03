use lab_raytracing_rs::camera::render;
use lab_raytracing_rs::camera::Camera;
use lab_raytracing_rs::lights::Pointlight;
use lab_raytracing_rs::spheres::Sphere;
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
    let mut base = Sphere::default();
    base.transform = scaling(0.8, 1.0, 0.8);

    let mut finger1 = Sphere::default();
    finger1.transform =
        translation(0.5, 0.8, 0.0) * rotation_z(-PI / 12.0) * scaling(0.3, 1.5, 0.3);

    let mut finger2 = Sphere::default();
    finger2.transform =
        translation(0.2, 1.2, 0.0) * rotation_z(-PI / 12.0) * scaling(0.3, 1.5, 0.3);

    let mut finger3 = Sphere::default();
    finger3.transform =
        translation(-0.2, 1.2, 0.0) * rotation_z(PI / 12.0) * scaling(0.3, 1.5, 0.3);

    let mut finger4 = Sphere::default();
    finger4.transform =
        translation(-0.5, 1.0, 0.0) * rotation_z(PI / 12.0) * scaling(0.3, 1.5, 0.3);

    let mut finger5 = Sphere::default();
    finger5.transform =
        translation(-0.8, 0.0, 0.0) * rotation_z(PI / 4.0) * scaling(0.25, 1.0, 0.25);

    let mut wall = Sphere::default();
    wall.transform = translation(-2.5, 0.0, -10.0) * scaling(10.0, 10.0, 0.01);

    let mut world = World::default();
    world.light = Some(Pointlight::new(point(5.0, 0.0, 10.0), color(1.0, 1.0, 1.0)));
    world.objects = vec![base, wall, finger1, finger2, finger3, finger4, finger5];

    let mut camera = Camera::new(800, 800, PI / 3.0);
    camera.transform = view_transform(
        &point(0.0, 0.0, 10.0),
        &point(-1.25, 0.0, 0.0),
        &vector(0.0, 1.0, 0.0),
    );

    let canvas = render(&camera, &world);

    let file = &mut io::stdout();
    let writer = &mut io::BufWriter::with_capacity(1024 * 128, file);
    canvas.ppm(writer)?;

    Ok(())
}
