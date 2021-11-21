use lab_raytracing_rs::camera::Camera;
use lab_raytracing_rs::csg::CSG;
use lab_raytracing_rs::groups::GroupMember;
use lab_raytracing_rs::lights::Pointlight;
use lab_raytracing_rs::objects::default_cube;
use lab_raytracing_rs::objects::default_plane;
use lab_raytracing_rs::objects::default_sphere;
use lab_raytracing_rs::patterns::checkers_pattern;
use lab_raytracing_rs::patterns::solid_pattern;
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
use std::f64::consts::PI;
use std::io;
use std::sync::Arc;

fn main() -> io::Result<()> {
    let grey = Box::new(solid_pattern(color(0.9, 0.9, 0.9)));
    let grey_dark = Box::new(solid_pattern(color(0.8, 0.8, 0.8)));
    let white_color = color(1.0, 1.0, 1.0);

    let mut world = World::default();

    let mut floor = default_plane();
    floor.material.pattern = Some(Box::new(checkers_pattern(grey.clone(), grey_dark.clone())));
    floor.material.specular = 0.0;
    world.add_object(floor);

    let mut wall = default_plane();
    wall.material.pattern = Some(Box::new(checkers_pattern(grey.clone(), grey_dark.clone())));
    wall.material.specular = 0.0;
    wall.set_transform(translation(0.0, 0.0, 10.0) * rotation_x(PI / 2.0));
    world.add_object(wall);

    let mut wall = default_plane();
    wall.material.pattern = Some(Box::new(checkers_pattern(grey, grey_dark)));
    wall.material.specular = 0.0;
    wall.set_transform(translation(9.0, 0.0, 0.0) * rotation_z(PI / 2.0));
    world.add_object(wall);

    let mut cube = default_cube();
    cube.set_transform(translation(0.0, 1.0, 0.0));
    cube.material.color = color(1.0, 0.0, 0.0);

    let mut sphere = default_sphere();
    sphere.set_transform(translation(1.0, 2.0, -1.0) * scaling(1.5, 1.5, 1.5));
    sphere.material.color = color(0.0, 1.0, 0.0);

    // world.add_object(cube);
    // world.add_object(sphere);

    let mut csg = CSG::Difference(
        GroupMember::Object(Arc::new(cube)),
        GroupMember::Object(Arc::new(sphere)),
    );
    csg = csg.update_transform(&rotation_y(PI / 12.0));
    world.add_csg(csg);

    world.light = Some(Pointlight::new(point(-11.0, 3.0, -10.0), white_color));

    let mut camera = Camera::new(1600, 900, PI / 3.0);
    camera.set_transform(view_transform(
        &point(-10.0, 10.0, -10.0),
        &point(0.0, 1.0, 0.0),
        &vector(0.0, 1.0, 0.0),
    ));

    let canvas = camera.render(&world);

    let file = &mut io::stdout();
    let writer = &mut io::BufWriter::with_capacity(1024 * 128, file);
    canvas.ppm(writer)?;

    Ok(())
}
