use cucumber_rust::Steps;
use lab_raytracing_rs::{
    patterns::{
        checkers_pattern, gradient_pattern, pattern_at_shape, ring_pattern, stripe_pattern,
        test_pattern, Renderer,
    },
    tuples::point,
};

use crate::{
    steps::{
        transformations::{parse_scaling, parse_translation},
        tuples::{parse_color, parse_point},
    },
    MyWorld,
};

pub fn steps() -> Steps<MyWorld> {
    let mut steps: Steps<MyWorld> = Steps::new();

    steps.given_regex(
        r#"^pattern ← (stripe_pattern|gradient_pattern|ring_pattern|checkers_pattern)\(white, black\)$"#,
        |mut world, ctx| {
            let white = world.tuples.get("white").unwrap();
            let black = world.tuples.get("black").unwrap();
            world.pattern = match ctx.matches[1].as_str() {
                "stripe_pattern" => stripe_pattern(*white, *black),
                "gradient_pattern" => gradient_pattern(*white, *black),
                "ring_pattern" => ring_pattern(*white, *black),
                "checkers_pattern" => checkers_pattern(*white, *black),
                _ => panic!("pattern not covered"),
            };
            world
        },
    );

    steps.then_regex(r#"^pattern.(a|b) = (white|black)$"#, |world, ctx| {
        let desired = world.tuples.get(&ctx.matches[2]).unwrap();
        let pattern = match (&world.pattern.renderer, ctx.matches[1].as_str()) {
            (Renderer::Stripes(a, _), "a") => a,
            (Renderer::Stripes(_, b), "b") => b,
            _ => panic!("pattern attribute not covered"),
        };
        let lookup = pattern.color_at(&point(0.0, 0.0, 0.0));
        assert_eq!(&lookup, desired);
        world
    });

    steps.then_regex(
        r#"^(stripe_at|pattern_at)\(pattern, point\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)\) = (white|black)$"#,
        |world, ctx| {
            let color = world.tuples.get(&ctx.matches[5]).unwrap();
            let point = parse_point(&ctx.matches[2..=4]);
            let computed = world.pattern.color_at(&point);
            assert_eq!(&computed, color);
            world
        },
    );

    steps.then_regex(
        r#"^(stripe_at|pattern_at)\(pattern, point\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)\) = color\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |world, ctx| {
            let color = parse_color(&ctx.matches[5..=7]);
            let point = parse_point(&ctx.matches[2..=4]);
            let computed = world.pattern.color_at(&point);
            assert_eq!(computed, color);
            world
        },
    );

    steps.given_regex(
        r#"^m.pattern ← stripe_pattern\(color\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\), color\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)\)$"#,
        |mut world, ctx| {
            let color_a = parse_point(&ctx.matches[1..=3]);
            let color_b = parse_point(&ctx.matches[4..=6]);
            world.m.pattern = Some(stripe_pattern(color_a, color_b));
            world
        },
    );

    steps.when_regex(
        r#"^c ← (stripe_at_object|pattern_at_shape)\(pattern, (object|shape), point\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)\)$"#,
        |mut world, ctx| {
            let point = parse_point(&ctx.matches[3..=5]);
            let object = world.shapes.get(&ctx.matches[2]).unwrap();
            let color = pattern_at_shape(&world.pattern, object, &point);
            world.tuples.insert("c".to_string(), color);
            world
        },
    );

    steps.given_regex(
        r#"^set_pattern_transform\(pattern, (scaling|translation)\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)\)$"#,
        |mut world, ctx| {
            let transformation = match ctx.matches[1].as_str() {
                "scaling" => parse_scaling(&ctx.matches[2..=4]),
                "translation" => parse_translation(&ctx.matches[2..=4]),
                _ => panic!("transformation not covered"),
            };
            world.pattern.set_transform(transformation);
            world
        },
    );

    steps.when_regex(
        r#"^set_pattern_transform\(pattern, (scaling|translation)\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)\)$"#,
        |mut world, ctx| {
            let transformation = match ctx.matches[1].as_str() {
                "scaling" => parse_scaling(&ctx.matches[2..=4]),
                "translation" => parse_translation(&ctx.matches[2..=4]),
                _ => panic!("transformation not covered"),
            };
            world.pattern.set_transform(transformation);
            world
        },
    );

    steps.given("pattern ← test_pattern()", |mut world, _ctx| {
        world.pattern = test_pattern();
        world
    });

    steps.then_regex("pattern.transform = (identity_matrix)", |world, ctx| {
        let desired = world.get4x4(&ctx.matches[1]);
        let lookup = world.pattern.transform();
        assert_eq!(lookup, desired);
        world
    });

    steps
}
