use lab_raytracing_rs::camera::Camera;
use lab_raytracing_rs::lights::Pointlight;
use lab_raytracing_rs::matrices::identity_matrix;
use lab_raytracing_rs::patterns::checkers_pattern;
use lab_raytracing_rs::patterns::ring_pattern;
use lab_raytracing_rs::patterns::solid_pattern;
use lab_raytracing_rs::patterns::stripe_pattern;
use lab_raytracing_rs::patterns::Pattern;
use lab_raytracing_rs::patterns::Renderer;
use lab_raytracing_rs::shapes::default_plane;
use lab_raytracing_rs::shapes::default_sphere;
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
    let black = Box::new(solid_pattern(color(0.0, 0.0, 0.0)));
    let red = Box::new(solid_pattern(color(1.0, 0.0, 0.0)));
    let green = Box::new(solid_pattern(color(0.0, 1.0, 0.0)));
    let blue = Box::new(solid_pattern(color(0.0, 0.0, 1.0)));
    let blue_color = color(0.0, 0.0, 1.0);
    let grey = Box::new(solid_pattern(color(0.8, 0.8, 0.8)));
    let white = Box::new(solid_pattern(color(1.0, 1.0, 1.0)));
    let white_color = color(1.0, 1.0, 1.0);

    let mut world = World::default();
    world.light = Some(Pointlight::new(point(-10.0, 10.0, -10.0), white_color));

    let mut stripes1 = stripe_pattern(black.clone(), green);
    stripes1.set_transform(rotation_y(PI / 3.0) * scaling(0.2, 0.2, 0.2));
    let stripes1 = Box::new(stripes1);
    let mut stripes2 = stripe_pattern(white.clone(), blue);
    stripes2.set_transform(rotation_y(-PI / 3.0) * scaling(0.2, 0.2, 0.2));
    let stripes2 = Box::new(stripes2);
    let merged_stripes = Box::new(Pattern::new(
        identity_matrix(),
        Renderer::Checkers(stripes1, stripes2),
    ));

    let mut floor = default_plane();
    // floor.material.pattern = Some(ring_pattern(red, grey));
    floor.material.pattern = Some(merged_stripes);
    world.add_object(floor);

    let wall_pattern = Box::new(ring_pattern(red.clone(), grey.clone()));
    let mut wall = default_plane();
    wall.set_transform(translation(0.0, 0.0, 4.0) * rotation_x(PI / 2.0));
    wall.material.pattern = Some(wall_pattern);
    world.add_object(wall);

    let mut pattern = Box::new(checkers_pattern(black, white));
    pattern.set_transform(scaling(0.25, 0.25, 0.25));
    let mut middle = default_sphere();
    middle.set_transform(
        translation(-0.5, 1.0, 0.5)
            * rotation_x(PI / 4.0)
            * rotation_y(PI / 4.0)
            * rotation_z(PI / 4.0),
    );
    middle.material.pattern = Some(pattern);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;
    world.add_object(middle);

    let right_pattern = Box::new(stripe_pattern(red, grey));
    let mut right = default_sphere();
    right.set_transform(translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5));
    let px = Box::new(Perlin::new());
    px.set_seed(1);
    let py = Box::new(Perlin::new());
    py.set_seed(1);
    let pz = Box::new(Perlin::new());
    pz.set_seed(1);
    let perlin_pattern = Box::new(Pattern::new(
        identity_matrix(),
        Renderer::Perturbed(0.5, px, py, pz, right_pattern),
    ));
    right.material.pattern = Some(perlin_pattern);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;
    world.add_object(right);

    let mut left = default_sphere();
    left.set_transform(translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33));
    left.material.color = blue_color;
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;
    world.add_object(left);

    let mut camera = Camera::new(800, 800, PI / 3.0);
    camera.set_transform(view_transform(
        &point(0.0, 1.5, -5.0),
        &point(0.0, 1.0, 0.0),
        &vector(0.0, 1.0, 0.0),
    ));

    let canvas = camera.render(&world);

    let file = &mut io::stdout();
    let writer = &mut io::BufWriter::with_capacity(1024 * 128, file);
    canvas.ppm(writer)?;

    Ok(())
}
