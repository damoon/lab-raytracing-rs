use std::rc::Rc;

use crate::MyWorld;
use cucumber_rust::Steps;
use lab_raytracing_rs::{intersections::Intersection, planes::default_plane};

use super::tuples::parse_point;

pub fn steps() -> Steps<MyWorld> {
    let mut steps: Steps<MyWorld> = Steps::new();

    steps.given("p ← plane()", |mut world, _ctx| {
        let p = default_plane();
        world.shapes.insert("p".to_string(), Rc::new(p));
        world
    });

    steps.when("xs ← local_intersect(p, r)", |mut world, _ctx| {
        let obj = world.shapes.get("p").unwrap();
        world.xs = obj
            .shape
            .intersect(&world.r)
            .iter()
            .map(|&i| Intersection {
                t: i,
                object: obj.clone(),
            })
            .collect();
        world
    });

    steps.then("xs is empty", |world, _ctx| {
        assert_eq!(world.xs.len(), 0);
        world
    });

    steps.then_regex(r#"^xs\[([-0-9.]+)\].object = p$"#, |world, ctx| {
        let desired = world.shapes.get("p").unwrap();
        let index = ctx.matches[1].parse::<usize>().unwrap();
        let lookup = &world.xs.get(index).unwrap().object;
        assert_eq!(lookup, desired);
        world
    });

    steps.when_regex(
        r#"^(n1|n2|n3) ← local_normal_at\((p), point\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)\)$"#,
        |mut world, ctx| {
            let point = &parse_point(&ctx.matches[3..=5]);
            let obj = world.shapes.get(&ctx.matches[2]).unwrap();
            let normal = obj.shape.normal_at(point);
            world.tuples.insert(ctx.matches[1].to_string(), normal);
            world
        },
    );

    steps
}
