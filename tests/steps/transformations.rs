use crate::Matrix;
use cucumber_rust::Steps;
use lab_raytracing_rs::transformations::{
    rotation_x, rotation_y, rotation_z, scaling, shearing, translation,
};
use lab_raytracing_rs::tuples::{point, vector};
use std::f64::consts::PI;

use crate::MyWorld;

pub fn steps() -> Steps<MyWorld> {
    let mut steps: Steps<MyWorld> = Steps::new();

    steps.given_regex(
        r#"^transform ← translation\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |mut world, ctx| {
            let x = ctx.matches[1].parse::<f64>().unwrap();
            let y = ctx.matches[2].parse::<f64>().unwrap();
            let z = ctx.matches[3].parse::<f64>().unwrap();

            world.transform = translation(x, y, z);

            world
        },
    );

    steps.then_regex(
        r#"^transform \* p = point\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |world, ctx| {
            let x = ctx.matches[1].parse::<f64>().unwrap();
            let y = ctx.matches[2].parse::<f64>().unwrap();
            let z = ctx.matches[3].parse::<f64>().unwrap();
            let desired = point(x, y, z);
            let point = world.tuples.get("p").unwrap();
            let calculated = &world.transform * point;

            assert_eq!(calculated, desired);

            world
        },
    );

    steps.given_regex(r#"^inv ← inverse\(transform\)$"#, |mut world, _ctx| {
        world.inv = world.transform.inverse().unwrap();

        world
    });

    steps.then_regex(
        r#"^inv \* p = point\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |world, ctx| {
            let x = ctx.matches[1].parse::<f64>().unwrap();
            let y = ctx.matches[2].parse::<f64>().unwrap();
            let z = ctx.matches[3].parse::<f64>().unwrap();
            let desired = point(x, y, z);
            let point = world.tuples.get("p").unwrap();
            let calculated = &world.inv * point;

            assert_eq!(calculated, desired);

            world
        },
    );

    steps.then_regex(r#"^transform \* v = v$"#, |world, _ctx| {
        let point = world.tuples.get("v").unwrap();
        let calculated = &world.transform * point;

        assert_eq!(&calculated, point);

        world
    });

    steps.given_regex(
        r#"^transform ← scaling\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |mut world, ctx| {
            let x = ctx.matches[1].parse::<f64>().unwrap();
            let y = ctx.matches[2].parse::<f64>().unwrap();
            let z = ctx.matches[3].parse::<f64>().unwrap();

            world.transform = scaling(x, y, z);

            world
        },
    );

    steps.then_regex(
        r#"^transform \* v = vector\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |world, ctx| {
            let x = ctx.matches[1].parse::<f64>().unwrap();
            let y = ctx.matches[2].parse::<f64>().unwrap();
            let z = ctx.matches[3].parse::<f64>().unwrap();
            let desired = vector(x, y, z);
            let vector = world.tuples.get("v").unwrap();
            let calculated = &world.transform * vector;

            assert_eq!(calculated, desired);

            world
        },
    );

    steps.then_regex(
        r#"^inv \* v = vector\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |world, ctx| {
            let x = ctx.matches[1].parse::<f64>().unwrap();
            let y = ctx.matches[2].parse::<f64>().unwrap();
            let z = ctx.matches[3].parse::<f64>().unwrap();
            let desired = vector(x, y, z);
            let point = world.tuples.get("v").unwrap();
            let calculated = &world.inv * point;

            assert_eq!(calculated, desired);

            world
        },
    );

    steps.given_regex(
        r#"^half_quarter ← rotation_x\(π / 4\)$"#,
        |mut world, _ctx| {
            world.half_quarter = rotation_x(PI / 4.0);
            world
        },
    );

    steps.given_regex(
        r#"^full_quarter ← rotation_x\(π / 2\)$"#,
        |mut world, _ctx| {
            world.full_quarter = rotation_x(PI / 2.0);
            world
        },
    );

    steps.given_regex(
        r#"^half_quarter ← rotation_y\(π / 4\)$"#,
        |mut world, _ctx| {
            world.half_quarter = rotation_y(PI / 4.0);
            world
        },
    );

    steps.given_regex(
        r#"^full_quarter ← rotation_y\(π / 2\)$"#,
        |mut world, _ctx| {
            world.full_quarter = rotation_y(PI / 2.0);
            world
        },
    );

    steps.given_regex(
        r#"^half_quarter ← rotation_z\(π / 4\)$"#,
        |mut world, _ctx| {
            world.half_quarter = rotation_z(PI / 4.0);
            world
        },
    );

    steps.given_regex(
        r#"^full_quarter ← rotation_z\(π / 2\)$"#,
        |mut world, _ctx| {
            world.full_quarter = rotation_z(PI / 2.0);
            world
        },
    );

    steps.given_regex(r#"^inv ← inverse\(half_quarter\)$"#, |mut world, _ctx| {
        world.inv = world.half_quarter.inverse().unwrap();
        world
    });

    steps.then_regex(
        r#"^half_quarter \* p = point\(0, √2/2, √2/2\)$"#,
        |world, _ctx| {
            let desired = point(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0);
            let point = world.tuples.get("p").unwrap();
            let calculated = &world.half_quarter * point;

            assert_eq!(calculated, desired);

            world
        },
    );

    steps.then_regex(
        r#"^half_quarter \* p = point\(√2/2, 0, √2/2\)$"#,
        |world, _ctx| {
            let desired = point(2.0_f64.sqrt() / 2.0, 0.0, 2.0_f64.sqrt() / 2.0);
            let point = world.tuples.get("p").unwrap();
            let calculated = &world.half_quarter * point;

            assert_eq!(calculated, desired);

            world
        },
    );

    steps.then_regex(
        r#"^half_quarter \* p = point\(-√2/2, √2/2, 0\)$"#,
        |world, _ctx| {
            let desired = point(-(2.0_f64.sqrt()) / 2.0, 2.0_f64.sqrt() / 2.0, 0.0);
            let point = world.tuples.get("p").unwrap();
            let calculated = &world.half_quarter * point;

            assert_eq!(calculated, desired);

            world
        },
    );

    steps.then_regex(
        r#"^inv \* p = point\(0, √2/2, -√2/2\)$"#,
        |world, _ctx| {
            let desired = point(0.0, 2.0_f64.sqrt() / 2.0, -(2.0_f64.sqrt()) / 2.0);
            let point = world.tuples.get("p").unwrap();
            let calculated = &world.inv * point;

            assert_eq!(calculated, desired);

            world
        },
    );

    steps.then_regex(
        r#"^full_quarter \* p = point\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |world, ctx| {
            let x = ctx.matches[1].parse::<f64>().unwrap();
            let y = ctx.matches[2].parse::<f64>().unwrap();
            let z = ctx.matches[3].parse::<f64>().unwrap();
            let desired = point(x, y, z);
            let point = world.tuples.get("p").unwrap();
            let calculated = &world.full_quarter * point;

            assert_eq!(calculated, desired);

            world
        },
    );

    steps.given_regex(
        r#"^transform ← shearing\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+), ([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |mut world, ctx| {
            let xy = ctx.matches[1].parse::<f64>().unwrap();
            let xz = ctx.matches[2].parse::<f64>().unwrap();
            let yx = ctx.matches[3].parse::<f64>().unwrap();
            let yz = ctx.matches[4].parse::<f64>().unwrap();
            let zx = ctx.matches[5].parse::<f64>().unwrap();
            let zy = ctx.matches[6].parse::<f64>().unwrap();
            world.transform = shearing(xy, xz, yx, yz, zx, zy);

            world
        },
    );

    steps.given_regex(r#"^A ← rotation_x\(π / 2\)$"#, |mut world, _ctx| {
        let matrix = rotation_x(PI / 2.0);
        world.matrices.insert("A".to_string(), Matrix::M4x4(matrix));

        world
    });

    steps.given_regex(r#"^B ← scaling\(5, 5, 5\)$"#, |mut world, _ctx| {
        let matrix = scaling(5.0, 5.0, 5.0);
        world.matrices.insert("B".to_string(), Matrix::M4x4(matrix));

        world
    });

    steps.given_regex(r#"^C ← translation\(10, 5, 7\)$"#, |mut world, _ctx| {
        let matrix = translation(10.0, 5.0, 7.0);
        world.matrices.insert("C".to_string(), Matrix::M4x4(matrix));

        world
    });

    steps.when_regex(
        r#"^([a-z0-9]+) ← ([A-Z]+) \* ([a-z0-9]+)$"#,
        |mut world, ctx| {
            let name1 = ctx.matches[1].clone();
            let name2 = ctx.matches[2].clone();
            let name3 = ctx.matches[3].clone();
            let matrix = match world.matrices.get(&name2).unwrap() {
                Matrix::M4x4(m) => m,
                _ => panic!("matrix needs to be in 4x4 form"),
            };
            let tuple = world.tuples.get(&name3).unwrap();

            let computed = matrix * tuple;

            world.tuples.insert(name1, computed);

            world
        },
    );

    steps
}
