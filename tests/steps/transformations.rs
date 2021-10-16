use crate::steps::tuples::{parse_point, parse_vector};
use cucumber_rust::Steps;
use lab_raytracing_rs::matrices::Matrix4x4;
use lab_raytracing_rs::transformations::{
    rotation_x, rotation_y, rotation_z, scaling, shearing, translation, view_transform,
};
use std::f64::consts::PI;

use crate::MyWorld;

pub fn parse_translation(ss: &[String]) -> Matrix4x4 {
    let x = ss[0].parse::<f64>().unwrap();
    let y = ss[1].parse::<f64>().unwrap();
    let z = ss[2].parse::<f64>().unwrap();
    translation(x, y, z)
}

pub fn parse_scaling(ss: &[String]) -> Matrix4x4 {
    let x = ss[0].parse::<f64>().unwrap();
    let y = ss[1].parse::<f64>().unwrap();
    let z = ss[2].parse::<f64>().unwrap();
    scaling(x, y, z)
}

pub fn parse_shearing(ss: &[String]) -> Matrix4x4 {
    let xy = ss[0].parse::<f64>().unwrap();
    let xz = ss[1].parse::<f64>().unwrap();
    let yx = ss[2].parse::<f64>().unwrap();
    let yz = ss[3].parse::<f64>().unwrap();
    let zx = ss[4].parse::<f64>().unwrap();
    let zy = ss[5].parse::<f64>().unwrap();
    shearing(xy, xz, yx, yz, zx, zy)
}

pub fn steps() -> Steps<MyWorld> {
    let mut steps: Steps<MyWorld> = Steps::new();

    steps.given_regex(
        r#"^(transform|B|C|m) ← (translation|scaling)\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |mut world, ctx| {
            let transformation = match ctx.matches[2].as_str() {
                "translation" => parse_translation(&ctx.matches[3..=5]),
                "scaling" => parse_scaling(&ctx.matches[3..=5]),
                _ => panic!("transformation not covered"),
            };
            world.insert4x4(ctx.matches[1].clone(), transformation);
            world
        },
    );

    steps.given_regex(
        r#"^(m) ← scaling\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\) \* rotation_z\(π/5\)$"#,
        |mut world, ctx| {
            let scaling = parse_scaling(&ctx.matches[2..=4]);
            let rotation_z = rotation_z(PI / 5.0);
            let transformation = scaling * rotation_z;
            world.insert4x4(ctx.matches[1].clone(), transformation);
            world
        },
    );

    steps.then_regex(
        r#"^(transform|inv) \* (p|v) = (point|vector)\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |world, ctx| {
            let desired = match ctx.matches[3].as_str() {
                "point" => parse_point(&ctx.matches[4..=6]),
                "vector" => parse_vector(&ctx.matches[4..=6]),
                _ => panic!("action not defined"),
            };
            let transformation = world.get4x4(&ctx.matches[1]);
            let tuple = world.tuples.get(&ctx.matches[2]).unwrap().clone();
            let calculated = transformation * tuple;
            assert_eq!(calculated, desired);

            world
        },
    );

    steps.given_regex(
        r#"^(inv) ← inverse\((transform|half_quarter)\)$"#,
        |mut world, ctx| {
            let inverse = world.get4x4(&ctx.matches[2]).inverse().unwrap();
            world.insert4x4(ctx.matches[1].clone(), inverse);
            world
        },
    );

    steps.then_regex(r#"^(transform) \* (v) = (v)$"#, |world, ctx| {
        let transformation = world.get4x4(&ctx.matches[1]);
        let tuple = world.tuples.get(&ctx.matches[2]).unwrap();
        let desired = world.tuples.get(&ctx.matches[3]).unwrap();
        let calculated = transformation * tuple;
        assert_eq!(&calculated, desired);
        world
    });

    steps.given_regex(
        r#"^(half_quarter|full_quarter|A) ← rotation_(x|y|z)\(π / ([-0-9.]+)\)$"#,
        |mut world, ctx| {
            let divisor = ctx.matches[3].parse::<f64>().unwrap();
            let rotation = match ctx.matches[2].as_str() {
                "x" => rotation_x(PI / divisor),
                "y" => rotation_y(PI / divisor),
                "z" => rotation_z(PI / divisor),
                _ => panic!("axis unknown"),
            };
            world.insert4x4(ctx.matches[1].clone(), rotation);
            world
        },
    );

    steps.then_regex(
        r#"^(half_quarter|full_quarter|inv) \* (p) = (point)\(([-0-9.]+|-?√2/2), ([-0-9.]+|-?√2/2), ([-0-9.]+|-?√2/2)\)$"#,
        |world, ctx| {
            let transformation = world.get4x4(&ctx.matches[1]);
            let tuple = world.tuples.get(&ctx.matches[2]).unwrap().clone();
            let desired = match ctx.matches[3].as_str() {
                "point" => parse_point(&ctx.matches[4..=6]),
                _ => panic!("action not defined"),
            };
            let calculated = transformation * tuple;
            assert_eq!(calculated, desired);
            world
        },
    );

    steps.given_regex(
        r#"^(transform) ← shearing\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+), ([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |mut world, ctx| {
            let shearing = parse_shearing(&ctx.matches[2..=7]);
            world.insert4x4(ctx.matches[1].clone(), shearing);
            world
        },
    );

    steps.when_regex(
        r#"^(p2|p3|p4) ← (A|B|C) \* (p|p2|p3)$"#,
        |mut world, ctx| {
            let matrix = world.get4x4(&ctx.matches[2]);
            let tuple = world.tuples.get(&ctx.matches[3]).unwrap();
            let computed = matrix * tuple;
            world.tuples.insert(ctx.matches[1].clone(), computed);
            world
        },
    );

    steps.when("t ← view_transform(from, to, up)", |mut world, _ctx| {
        let from = world.tuples.get("from").unwrap();
        let to = world.tuples.get("to").unwrap();
        let up = world.tuples.get("up").unwrap();
        let view_transformation = view_transform(from, to, up);
        world.insert4x4("t".to_string(), view_transformation);
        world
    });

    steps.then_regex(
        r#"^(t|s.transform|pattern.transform) = (scaling|translation)\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |world, ctx| {
            let desired = match ctx.matches[2].as_str() {
                "scaling" => parse_scaling(&ctx.matches[3..=5]),
                "translation" => parse_translation(&ctx.matches[3..=5]),
                _ => panic!("desired function not covered"),
            };
            let lookup = match ctx.matches[1].as_str() {
                "s.transform" => world.shapes.get("s").unwrap().transform(),
                "pattern.transform" => world.pattern.transform(),
                _ => world.get4x4(&ctx.matches[1]),
            };
            assert_eq!(lookup, &desired);
            world
        },
    );

    steps
}
