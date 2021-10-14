use std::rc::Rc;

use crate::{
    steps::tuples::{parse_point, parse_vector},
    MyWorld,
};
use cucumber_rust::Steps;
use lab_raytracing_rs::shapes::default_testshape;

pub fn steps() -> Steps<MyWorld<'static>> {
    let mut steps: Steps<MyWorld> = Steps::new();

    steps.given_regex(r#"^(s) ← test_shape\(\)$"#, |mut world, ctx| {
        world
            .shapes
            .insert(ctx.matches[1].clone(), default_testshape());
        world
    });

    steps.then_regex(
        r#"^s.saved_ray.(origin|direction) = (point|vector)\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |world, ctx| {
            let desired = match ctx.matches[2].as_str() {
                "point" => parse_point(&ctx.matches[3..=5]),
                "vector" => parse_vector(&ctx.matches[3..=5]),
                _ => panic!("desired kind not covered"),
            };
            let ray = lab_raytracing_rs::shapes::SAVED_RAY.with(|c| c.read().unwrap().clone());
            let lookup = match ctx.matches[1].as_str() {
                "origin" => ray.origin.clone(),
                "direction" => ray.direction.clone(),
                _ => panic!("lookup attribute not covered"),
            };
            assert_eq!(lookup, desired);
            world
        },
    );

    steps
}
