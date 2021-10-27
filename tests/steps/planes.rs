use std::sync::Arc;

use crate::MyWorld;
use cucumber::Steps;
use lab_raytracing_rs::{intersections::Intersection, objects::default_plane};

use super::tuples::parse_point;

pub fn steps() -> Steps<MyWorld> {
    let mut steps: Steps<MyWorld> = Steps::new();

    steps.given("p ← plane()", |mut world, _ctx| {
        let p = default_plane();
        world.objects.insert("p".to_string(), Arc::new(p));
        world
    });

    steps.when_regex(
        r#"xs ← local_intersect\((p|c|cyl|shape), r\)"#,
        |mut world, ctx| {
            let obj = world.objects.get(&ctx.matches[1]).unwrap();
            world.xs = obj
                .intersect(&world.r)
                .iter()
                .map(|&i| Intersection {
                    t: i,
                    object: obj.clone(),
                })
                .collect();
            world
        },
    );

    steps.then("xs is empty", |world, _ctx| {
        assert_eq!(world.xs.len(), 0);
        world
    });

    steps.then_regex(r#"^xs\[([-0-9.]+)\].object = (p|s2|s1)$"#, |world, ctx| {
        let desired = world.objects.get(&ctx.matches[2]).unwrap();
        let index = ctx.matches[1].parse::<usize>().unwrap();
        let lookup = &world.xs.get(index).unwrap().object;
        assert_eq!(lookup, desired);
        world
    });

    steps.when_regex(
        r#"^(n|n1|n2|n3) ← local_normal_at\((p|cyl|shape), point\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)\)$"#,
        |mut world, ctx| {
            let point = &parse_point(&ctx.matches[3..=5]);
            let obj = world.objects.get(&ctx.matches[2]).unwrap();
            let normal = obj.shape.normal_at(point);
            world.tuples.insert(ctx.matches[1].clone(), normal);
            world
        },
    );

    steps.when_regex(
        r#"^(normal) ← local_normal_at\((c), (p)\)$"#,
        |mut world, ctx| {
            let point = world.tuples.get(&ctx.matches[3]).unwrap();
            let obj = world.objects.get(&ctx.matches[2]).unwrap();
            let normal = obj.shape.normal_at(point);
            world.tuples.insert(ctx.matches[1].clone(), normal);
            world
        },
    );

    steps
}
