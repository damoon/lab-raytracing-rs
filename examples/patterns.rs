use lab_raytracing_rs::camera::render;
use lab_raytracing_rs::camera::Camera;
use lab_raytracing_rs::lights::Pointlight;
use lab_raytracing_rs::matrices::identity_matrix;
use lab_raytracing_rs::patterns::checkers_pattern;
use lab_raytracing_rs::patterns::ring_pattern;
use lab_raytracing_rs::patterns::stripe_pattern;
use lab_raytracing_rs::patterns::Pattern;
use lab_raytracing_rs::patterns::Renderer;
use lab_raytracing_rs::planes::Plane;
use lab_raytracing_rs::spheres::Sphere;
use lab_raytracing_rs::transformations::rotation_x;
use lab_raytracing_rs::transformations::rotation_y;
use lab_raytracing_rs::transformations::rotation_z;
use lab_raytracing_rs::transformations::scaling;
use lab_raytracing_rs::transformations::translation;
use lab_raytracing_rs::transformations::view_transform;
use lab_raytracing_rs::tuples::color;
use lab_raytracing_rs::tuples::point;
use lab_raytracing_rs::tuples::vector;
use lab_raytracing_rs::world::World;
use noise::{Perlin, Seedable};
use std::f64::consts::PI;
use std::io;

fn main() -> io::Result<()> {
    let black = color(0.0, 0.0, 0.0);
    let red = color(1.0, 0.0, 0.0);
    let green = color(0.0, 1.0, 0.0);
    let blue = color(0.0, 0.0, 1.0);
    let grey = color(0.8, 0.8, 0.8);
    let white = color(1.0, 1.0, 1.0);

    let mut stripes1 = stripe_pattern(white, green);
    stripes1.transform = rotation_y(PI / 3.0) * scaling(0.2, 0.2, 0.2);
    let mut stripes2 = stripe_pattern(white, blue);
    stripes2.transform = rotation_y(-PI / 3.0) * scaling(0.2, 0.2, 0.2);
    let merged_stripes = Pattern {
        transform: identity_matrix(),
        renderer: Renderer::Checkers(Box::new(stripes1), Box::new(stripes2)),
    };

    let mut floor = Plane::default();
    // floor.material.pattern = Some(ring_pattern(red, grey));
    floor.material.pattern = Some(merged_stripes);

    let mut wall = Plane::default();
    wall.transform = translation(0.0, 0.0, 4.0) * rotation_x(PI / 2.0);
    wall.material.pattern = Some(ring_pattern(red, grey));

    let mut pattern = checkers_pattern(black, white);
    pattern.transform = scaling(0.25, 0.25, 0.25);
    let mut middle = Sphere::default();
    middle.transform = translation(-0.5, 1.0, 0.5)
        * rotation_x(PI / 4.0)
        * rotation_y(PI / 4.0)
        * rotation_z(PI / 4.0);
    middle.material.pattern = Some(pattern);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    let mut right = Sphere::default();
    right.transform = translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5);
    let px = Perlin::new();
    px.set_seed(1);
    let py = Perlin::new();
    py.set_seed(1);
    let pz = Perlin::new();
    pz.set_seed(1);
    let perlin_pattern = Pattern {
        transform: identity_matrix(),
        renderer: Renderer::Perturbed(
            0.5,
            Box::new(px),
            Box::new(py),
            Box::new(pz),
            Box::new(stripe_pattern(red, grey)),
        ),
    };
    right.material.pattern = Some(perlin_pattern);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    let mut left = Sphere::default();
    left.transform = translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33);
    left.material.color = blue;
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    let mut world = World::default();
    world.light = Some(Pointlight::new(point(-10.0, 10.0, -10.0), white));
    world.objects = vec![floor, wall, middle, right, left];

    let mut camera = Camera::new(800, 800, PI / 3.0);
    camera.transform = view_transform(
        &point(0.0, 1.5, -5.0),
        &point(0.0, 1.0, 0.0),
        &vector(0.0, 1.0, 0.0),
    );

    let canvas = render(&camera, &world);

    let file = &mut io::stdout();
    let writer = &mut io::BufWriter::with_capacity(1024 * 128, file);
    canvas.ppm(writer)?;

    Ok(())
}