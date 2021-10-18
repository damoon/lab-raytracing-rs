use lab_raytracing_rs::camera::Camera;
use lab_raytracing_rs::lights::Pointlight;
use lab_raytracing_rs::objects::default_cone;
use lab_raytracing_rs::objects::default_cylinder;
use lab_raytracing_rs::objects::Shape;
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

    let red = color(1.0, 0.0, 0.0);
    let green = color(0.0, 1.0, 0.0);
    let white = color(1.0, 1.0, 1.0);

    let mut world = World::default();
    world.light = Some(Pointlight::new(point(12.0, 20.0, 12.0), white));

    let mut head = default_cone();
    head.shape = Shape::Cone(-1.2, 0.0, false);
    head.material.color = green;
    head.set_transform(translation(0.0, 10.0, 0.0));
    world.add_object(head);

    let mut body = default_cylinder();
    body.shape = Shape::Cylinder(0.0, 9.0, false);
    body.material.color = red;
    world.add_object(body);

    let mut camera = Camera::new(1600, 900, PI / 3.0);
    camera.set_transform(view_transform(
        &point(10.0, 15.0, 10.0),
        &point(0.0, 5.0, 0.0),
        &vector(0.0, 1.0, 0.0),
    ));

    let canvas = camera.render(&world);

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
