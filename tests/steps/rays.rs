use super::tuples::{parse_point, parse_vector};
use crate::MyWorld;
use cucumber_rust::Steps;
use lab_raytracing_rs::rays::Ray;

pub fn steps() -> Steps<MyWorld> {
    let mut steps: Steps<MyWorld> = Steps::new();

    steps.when_regex(r#"^r ← ray\(origin, direction\)$"#, |mut world, _ctx| {
        let origin = *world.tuples.get("origin").unwrap();
        let direction = *world.tuples.get("direction").unwrap();
        world.r = Ray::new(origin, direction);
        world
    });

    steps.given_regex(r#"^r ← ray\(point\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\), vector\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)\)$"#, |mut world, ctx| {
        let origin = parse_point(&ctx.matches[1..=3]);
        let direction = parse_vector(&ctx.matches[4..=6]);
        world.r = Ray::new(origin, direction);
        world
    });

    steps.then_regex(
        r#"^position\(r, ([-0-9.]+)\) = point\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |world, ctx| {
            let t = ctx.matches[1].parse::<f64>().unwrap();
            let calculated = world.r.position(t);
            let desired = parse_point(&ctx.matches[2..=4]);
            assert_eq!(desired, calculated);
            world
        },
    );

    steps.then_regex(r#"^r.origin = origin$"#, |world, _ctx| {
        assert_eq!(&world.r.origin, world.tuples.get("origin").unwrap());
        world
    });

    steps.then_regex(r#"^r.direction = direction$"#, |world, _ctx| {
        assert_eq!(&world.r.direction, world.tuples.get("direction").unwrap());
        world
    });

    steps.when_regex(r#"^r2 ← transform\(r, m\)$"#, |mut world, _ctx| {
        let transformation = world.get4x4("m");
        world.r2 = world.r.transform(&transformation);
        world
    });

    steps.then_regex(
        r#"^r2.origin = point\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |world, ctx| {
            let point = parse_point(&ctx.matches[1..=3]);
            assert_eq!(world.r2.origin, point);
            world
        },
    );

    steps.then_regex(
        r#"^r2.direction = vector\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |world, ctx| {
            let vector = parse_vector(&ctx.matches[1..=3]);
            assert_eq!(world.r2.direction, vector);
            world
        },
    );

    steps
}
