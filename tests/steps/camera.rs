use std::f64::consts::PI;

use crate::MyWorld;
use approx::assert_abs_diff_eq;
use cucumber_rust::Steps;
use lab_raytracing_rs::{
    camera::{render, Camera},
    transformations::{rotation_y, translation, view_transform},
};

pub fn steps() -> Steps<MyWorld<'static>> {
    let mut steps: Steps<MyWorld> = Steps::new();

    steps.given_regex(r#"^(hsize|vsize) ← ([-0-9.]+)$"#, |mut world, ctx| {
        let value = ctx.matches[2].parse::<usize>().unwrap();
        world.usizes.insert(ctx.matches[1].clone(), value);
        world
    });

    steps.given("field_of_view ← π/2", |mut world, _ctx| {
        let value = PI / 2.0;
        world.floats.insert("field_of_view".to_string(), value);
        world
    });

    steps.when(
        "c ← camera(hsize, vsize, field_of_view)",
        |mut world, _ctx| {
            let hsize = *world.usizes.get("hsize").unwrap();
            let vsize = *world.usizes.get("vsize").unwrap();
            let field_of_view = *world.floats.get("field_of_view").unwrap();
            world.camera = Camera::new(hsize, vsize, field_of_view);
            world
        },
    );

    steps.then_regex(r#"^c.(hsize|vsize) = ([-0-9.]+)$"#, |world, ctx| {
        let value = ctx.matches[2].parse::<usize>().unwrap();
        match ctx.matches[1].as_str() {
            "hsize" => assert_eq!(world.camera.hsize, value),
            "vsize" => assert_eq!(world.camera.vsize, value),
            _ => panic!("camera property not covered"),
        }
        world
    });

    steps.then_regex(r#"^c.(pixel_size) = ([-0-9.]+)$"#, |world, ctx| {
        let value = ctx.matches[2].parse::<f64>().unwrap();
        match ctx.matches[1].as_str() {
            "pixel_size" => assert_abs_diff_eq!(world.camera.pixel_size, value),
            _ => panic!("camera property not covered"),
        }
        world
    });

    steps.then("c.field_of_view = π/2", |world, _ctx| {
        let value = PI / 2.0;
        assert_abs_diff_eq!(world.camera.field_of_view, value);
        world
    });

    steps.then_regex(r#"^c.transform = (identity_matrix)$"#, |world, _ctx| {
        let matrix = world.get4x4("identity_matrix");
        assert_eq!(&world.camera.transform(), &matrix);
        world
    });

    steps.given_regex(
        r#"^c ← camera\(([-0-9.]+), ([-0-9.]+), π/2\)$"#,
        |mut world, ctx| {
            let hsize = ctx.matches[1].parse::<usize>().unwrap();
            let vsize = ctx.matches[2].parse::<usize>().unwrap();
            let field_of_view = PI / 2.0;
            world.camera = Camera::new(hsize, vsize, field_of_view);
            world
        },
    );

    steps.when_regex(
        r#"^r ← ray_for_pixel\(c, ([-0-9.]+), ([-0-9.]+)\)$"#,
        |mut world, ctx| {
            let px = ctx.matches[1].parse::<usize>().unwrap();
            let py = ctx.matches[2].parse::<usize>().unwrap();
            world.r = world.camera.ray_for_pixel(px, py);
            world
        },
    );

    steps.when(
        "c.transform ← rotation_y(π/4) * translation(0, -2, 5)",
        |mut world, _ctx| {
            let transform = rotation_y(PI / 4.0) * translation(0.0, -2.0, 5.0);
            world.camera.set_transform(transform);
            world
        },
    );

    steps.given(
        "c.transform ← view_transform(from, to, up)",
        |mut world, _ctx| {
            let from = world.tuples.get("from").unwrap();
            let to = world.tuples.get("to").unwrap();
            let up = world.tuples.get("up").unwrap();
            let transform = view_transform(from, to, up);
            world.camera.set_transform(transform);
            world
        },
    );

    steps.when("image ← render(c, w)", |mut world, _ctx| {
        world.image = render(&world.camera, &world.w);
        world
    });

    steps
}
