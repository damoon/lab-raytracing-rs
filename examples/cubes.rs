use lab_raytracing_rs::camera::render;
use lab_raytracing_rs::camera::Camera;
use lab_raytracing_rs::lights::Pointlight;
use lab_raytracing_rs::planes::default_cube;
use lab_raytracing_rs::transformations::rotation_y;
use lab_raytracing_rs::transformations::scaling;
use lab_raytracing_rs::transformations::translation;
use lab_raytracing_rs::transformations::view_transform;
use lab_raytracing_rs::tuples::color;
use lab_raytracing_rs::tuples::point;
use lab_raytracing_rs::tuples::vector;
use lab_raytracing_rs::world::World;
use pprof::protos::Message;
use std::env;
use std::f64::consts::PI;
use std::fs::File;
use std::io;
use std::io::Write;

fn main() -> io::Result<()> {
    // start cpu profiler
    let guard = match env::var("PROFILE_CPU") {
        Err(_) => None,
        Ok(_) => Some(pprof::ProfilerGuard::new(100).unwrap()),
    };

    let yellow = color(1.0, 1.0, 0.0);
    let red = color(1.0, 0.0, 0.0);
    let green = color(0.0, 1.0, 0.0);
    let blue = color(0.0, 0.0, 1.0);
    let grey = color(0.8, 0.8, 0.8);
    let white = color(1.0, 1.0, 1.0);

    let mut world = World::default();
    world.light = Some(Pointlight::new(point(0.0, 2.9, -3.0), white.clone()));

    let mut light_cube = default_cube();
    light_cube.set_transform(translation(0.0, 2.9, -3.0) * scaling(0.1, 0.1, 0.1));
    light_cube.material.color = yellow;
    light_cube.material.ambient = 1.0;
    light_cube.throws_shaddow = false;
    world.add_object(light_cube);

    let mut cube = default_cube();
    cube.set_transform(translation(0.0, 0.3, 0.0) * scaling(0.2, 0.2, 0.2) * rotation_y(PI / 8.0));
    world.add_object(cube);

    let mut walls = default_cube();
    walls.set_transform(scaling(10.0, 11.0, 10.0));
    walls.material.color = blue;
    world.add_object(walls);

    let mut floor = default_cube();
    floor.set_transform(scaling(11.0, 3.0, 11.0));
    floor.material.color = red;
    world.add_object(floor);

    let mut tabletop = default_cube();
    tabletop.set_transform(scaling(3.0, 0.1, 2.0));
    tabletop.material.color = green;
    world.add_object(tabletop);

    let mut table_leg_1 = default_cube();
    table_leg_1.set_transform(translation(-2.8, -1.5, -1.8) * scaling(0.1, 1.5, 0.1));
    table_leg_1.material.color = grey.clone();
    world.add_object(table_leg_1);

    let mut table_leg_2 = default_cube();
    table_leg_2.set_transform(translation(2.8, -1.5, -1.8) * scaling(0.1, 1.5, 0.1));
    table_leg_2.material.color = grey.clone();
    world.add_object(table_leg_2);

    let mut table_leg_3 = default_cube();
    table_leg_3.set_transform(translation(2.8, -1.5, 1.8) * scaling(0.1, 1.5, 0.1));
    table_leg_3.material.color = grey.clone();
    world.add_object(table_leg_3);

    let mut table_leg_4 = default_cube();
    table_leg_4.set_transform(translation(-2.8, -1.5, 1.8) * scaling(0.1, 1.5, 0.1));
    table_leg_4.material.color = grey;
    world.add_object(table_leg_4);

    let mut camera = Camera::new(1600, 900, PI / 3.0);
    camera.set_transform(view_transform(
        &point(-9.0, 2.9, -5.0),
        &point(0.0, 0.0, 0.0),
        &vector(0.0, 1.0, 0.0),
    ));

    let canvas = render(&camera, &world);

    let file = &mut io::stdout();
    let writer = &mut io::BufWriter::with_capacity(1024 * 128, file);
    canvas.ppm(writer)?;

    if let Some(guard) = guard {
        // write cpu profile
        let report = guard.report().build().unwrap();
        let mut file = File::create("profile.pb").unwrap();
        let profile = report.pprof().unwrap();
        let mut content = Vec::new();
        profile.encode(&mut content).unwrap();
        file.write_all(&content).unwrap();

        // write flamegraph
        let file = File::create("flamegraph.svg").unwrap();
        report.flamegraph(file).unwrap();
    }

    Ok(())
}
