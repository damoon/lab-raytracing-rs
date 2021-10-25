use lab_raytracing_rs::camera::Camera;
use lab_raytracing_rs::groups::Group;
use lab_raytracing_rs::lights::Pointlight;
use lab_raytracing_rs::objects::default_plane;
use lab_raytracing_rs::objects::triangle;
use lab_raytracing_rs::patterns::checkers_pattern;
use lab_raytracing_rs::patterns::solid_pattern;
use lab_raytracing_rs::transformations::rotation_x;
use lab_raytracing_rs::transformations::rotation_y;
use lab_raytracing_rs::transformations::rotation_z;
use lab_raytracing_rs::transformations::translation;
use lab_raytracing_rs::transformations::view_transform;
use lab_raytracing_rs::tuples::color;
use lab_raytracing_rs::tuples::point;
use lab_raytracing_rs::tuples::vector;
use lab_raytracing_rs::world::World;
use std::f64::consts::PI;
use std::io;

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

    let mut tet = tetrahedron();
    tet.set_transform(translation(-5.0, 0.0, 0.55) * rotation_y(-PI / 10.0) * tet.transform());
    world.add_group(tet);

    let mut pyr = pyramid();
    pyr.set_transform(translation(-2.0, 0.0, -3.0) * rotation_y(PI / 7.0));
    world.add_group(pyr);

    let mut doc = dodecahedron();
    doc.set_transform(translation(1.0, 0.0, 1.0));
    world.add_group(doc);

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

fn tetrahedron() -> Group {
    let mut tet = Group::default();

    let k = 1.0 / 2.0_f64.sqrt();
    let p1 = point(1.0, 0.0, -k);
    let p2 = point(-1.0, 0.0, -k);
    let p3 = point(0.0, 1.0, k);
    let p4 = point(0.0, -1.0, k);

    let mut f1 = triangle(p1.clone(), p2.clone(), p3.clone());
    let mut f2 = triangle(p1.clone(), p2.clone(), p4.clone());
    let mut f3 = triangle(p1, p4.clone(), p3.clone());
    let mut f4 = triangle(p4, p2, p3);

    f1.material.color = color(0.1, 0.1, 1.0);
    f2.material.color = color(0.1, 0.1, 1.0);
    f3.material.color = color(0.1, 0.1, 1.0);
    f4.material.color = color(0.1, 0.1, 1.0);

    tet.add_object(f1);
    tet.add_object(f2);
    tet.add_object(f3);
    tet.add_object(f4);

    let edge_length = 2.0;
    let inner_radius = 6.0_f64.sqrt() * edge_length / 12.0;

    tet.set_transform(
        translation(0.0, inner_radius, 0.0) * rotation_x(-(1.0_f64 / 3.0).acos() / 2.0),
    );

    tet
}

fn pyramid() -> Group {
    let mut pyr = Group::default();

    let edge_length = 2.0;
    let h = 2.0_f64.sqrt() * edge_length / 2.0;

    let p1 = point(0.0, h, 0.0);
    let p2 = point(1.0, 0.0, 1.0);
    let p3 = point(-1.0, 0.0, 1.0);
    let p4 = point(-1.0, 0.0, -1.0);
    let p5 = point(1.0, 0.0, -1.0);

    let mut f1 = triangle(p1.clone(), p2.clone(), p3.clone());
    let mut f2 = triangle(p1.clone(), p3.clone(), p4.clone());
    let mut f3 = triangle(p1.clone(), p4.clone(), p5.clone());
    let mut f4 = triangle(p1, p5.clone(), p2.clone());
    let mut f5 = triangle(p2, p3.clone(), p4.clone());
    let mut f6 = triangle(p3, p4, p5);

    f1.material.color = color(1.0, 1.0, 0.1);
    f2.material.color = color(1.0, 1.0, 0.1);
    f3.material.color = color(1.0, 1.0, 0.1);
    f4.material.color = color(1.0, 1.0, 0.1);
    f5.material.color = color(1.0, 1.0, 0.1);
    f6.material.color = color(1.0, 1.0, 0.1);

    pyr.add_object(f1);
    pyr.add_object(f2);
    pyr.add_object(f3);
    pyr.add_object(f4);
    pyr.add_object(f5);
    pyr.add_object(f6);

    pyr
}

fn dodecahedron() -> Group {
    let mut doc = Group::default();

    let lower = dodecahedron_lower_half();
    let mut upper = lower.clone();

    let a = 2.0;
    let inner_radius = (a / 20.0) * (250.0 + 110.0 * (5.0_f64).sqrt()).sqrt();
    let height = 2.0 * inner_radius;

    upper.set_transform(translation(0.0, height, 0.0) * rotation_y(PI / 5.0) * rotation_z(PI));

    doc.add_group(lower);
    doc.add_group(upper);
    doc
}

fn dodecahedron_lower_half() -> Group {
    let mut doc = Group::default();

    let a = 2.0;
    let h = (a / 10.0) * (25.0 + 10.0 * 5.0_f64.sqrt()).sqrt();

    let face = pentagon();

    // angle between faces
    let beta = (-1.0 / (5.0_f64).sqrt()).acos();

    for n in 0..5 {
        let mut side = face.clone();
        side.set_transform(
            rotation_y(n as f64 * PI / 2.5)
                * translation(0.0, 0.0, h)
                * rotation_x(beta)
                * translation(0.0, 0.0, -h),
        );
        doc.add_group(side);
    }

    doc.add_group(face);
    doc
}

fn pentagon() -> Group {
    let mut doc = Group::default();

    let a = 2.0;
    let h = (a / 10.0) * (25.0 + 10.0 * 5.0_f64.sqrt()).sqrt();
    let p1 = point(0.0, 0.0, 0.0);
    let p2 = point(1.0, 0.0, h);
    let p3 = point(-1.0, 0.0, h);

    for n in 0..5 {
        let mut f = triangle(p1.clone(), p2.clone(), p3.clone());
        f.set_transform(rotation_y(n as f64 * 2.0 * PI / 5.0));
        f.material.color = color(1.0, 0.1, 0.1);
        doc.add_object(f);
    }

    doc
}
