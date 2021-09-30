use crate::MyWorld;
use approx::assert_abs_diff_eq;
use cucumber_rust::Steps;
use lab_raytracing_rs::spheres::Sphere;

use super::transformations::{parse_scaling, parse_translation};

pub fn steps() -> Steps<MyWorld> {
    let mut steps: Steps<MyWorld> = Steps::new();

    steps.given_regex(r#"^s ← sphere\(\)$"#, |mut world, _ctx| {
        world.s = Sphere::default();
        world
    });

    steps.given_regex(
        r#"^(t) ← translation\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |mut world, ctx| {
            let translation = parse_translation(&ctx.matches[2..=4]);
            world.insert4x4(ctx.matches[1].clone(), translation);
            world
        },
    );

    steps.when_regex(r#"^set_transform\(s, t\)$"#, |mut world, _ctx| {
        let transformation = world.get4x4("t");
        world.s.transform = transformation;
        world
    });

    steps.when_regex(
        r#"^set_transform\(s, (scaling|translation)\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)\)$"#,
        |mut world, ctx| {
            world.s.transform = match ctx.matches[1].as_str() {
                "scaling" => parse_scaling(&ctx.matches[2..=4]),
                "translation" => parse_translation(&ctx.matches[2..=4]),
                _ => panic!("transformation not covered"),
            };
            world
        },
    );

    steps.then_regex(r#"^s.transform = (identity_matrix|t)$"#, |world, ctx| {
        let transform = &world.s.transform;
        let matrix = &world.get4x4(ctx.matches[1].as_str());
        assert_eq!(transform, matrix);
        world
    });

    steps.when_regex(r#"^xs ← intersect\(s, r\)$"#, |mut world, _ctx| {
        world.xs = world.s.intersect(&world.r);
        world
    });

    steps.then_regex(r#"^xs.count = ([-0-9.]+)$"#, |world, ctx| {
        let c = ctx.matches[1].parse::<usize>().unwrap();
        assert_eq!(world.xs.len(), c);
        world
    });

    steps.then_regex(r#"^xs\[([-0-9.]+)\] = ([-0-9.]+)$"#, |world, ctx| {
        let index = ctx.matches[1].parse::<usize>().unwrap();
        let desired = ctx.matches[2].parse::<f64>().unwrap();
        let value = world.xs.get(index).unwrap().t;
        assert_abs_diff_eq!(value, desired);
        world
    });

    steps.then_regex(r#"^xs\[([-0-9.]+)\].object = s$"#, |world, ctx| {
        let index = ctx.matches[1].parse::<usize>().unwrap();
        assert_eq!(world.xs.get(index).unwrap().object, world.s);
        world
    });

    steps
}
