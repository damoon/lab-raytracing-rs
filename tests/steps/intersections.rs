use crate::MyWorld;
use approx::assert_abs_diff_eq;
use cucumber_rust::Steps;
use lab_raytracing_rs::intersections::{hit, Intersection};

pub fn steps() -> Steps<MyWorld> {
    let mut steps: Steps<MyWorld> = Steps::new();

    steps.when_regex(
        r#"^(i) ← intersection\(([-0-9.]+), s\)$"#,
        |mut world, ctx| {
            let name = ctx.matches[1].clone();
            let t = ctx.matches[2].parse::<f64>().unwrap();
            let object = world.s.clone();
            let intersection = Intersection { t, object };
            world.intersections.insert(name, intersection);
            world
        },
    );

    steps.given_regex(
        r#"^(i1|i2|i3|i4) ← intersection\(([-0-9.]+), s\)$"#,
        |mut world, ctx| {
            let name = ctx.matches[1].clone();
            let t = ctx.matches[2].parse::<f64>().unwrap();
            let object = world.s.clone();
            let intersection = Intersection { t, object };
            world.intersections.insert(name, intersection);
            world
        },
    );

    steps.then_regex(r#"^(i).object = s$"#, |world, ctx| {
        let intersection = world.intersections.get(&ctx.matches[1]).unwrap();
        assert_eq!(world.s, intersection.object);
        world
    });

    steps.then_regex(r#"^(i).t = ([-0-9.]+)$"#, |world, ctx| {
        let intersection = world.intersections.get(&ctx.matches[1]).unwrap();
        let desired = ctx.matches[2].parse::<f64>().unwrap();
        assert_abs_diff_eq!(desired, intersection.t);
        world
    });

    steps.when_regex(
        r#"^xs ← intersections\((i1), (i2)\)$"#,
        |mut world, ctx| {
            let i1 = world.intersections.get(&ctx.matches[1]).unwrap().clone();
            let i2 = world.intersections.get(&ctx.matches[2]).unwrap().clone();
            world.xs = vec![i1, i2];
            world
        },
    );

    steps.given_regex(
        r#"^xs ← intersections\((i2), (i1)\)$"#,
        |mut world, ctx| {
            let i1 = world.intersections.get(&ctx.matches[1]).unwrap().clone();
            let i2 = world.intersections.get(&ctx.matches[2]).unwrap().clone();
            world.xs = vec![i1, i2];
            world
        },
    );

    steps.given_regex(
        r#"^xs ← intersections\((i1), (i2), (i3), (i4)\)$"#,
        |mut world, ctx| {
            let i1 = world.intersections.get(&ctx.matches[1]).unwrap().clone();
            let i2 = world.intersections.get(&ctx.matches[2]).unwrap().clone();
            let i3 = world.intersections.get(&ctx.matches[3]).unwrap().clone();
            let i4 = world.intersections.get(&ctx.matches[4]).unwrap().clone();
            world.xs = vec![i1, i2, i3, i4];
            world
        },
    );

    steps.then_regex(r#"^xs\[([-0-9.]+)\].t = ([-0-9.]+)$"#, |world, ctx| {
        let index = ctx.matches[1].parse::<usize>().unwrap();
        let desired = ctx.matches[2].parse::<f64>().unwrap();
        assert_abs_diff_eq!(world.xs.get(index).unwrap().t, desired);
        world
    });

    steps.when_regex(r#"^(i) ← hit\(xs\)$"#, |mut world, ctx| {
        let name = ctx.matches[1].clone();
        let intersection = hit(world.xs.clone());
        match intersection {
            None => world.intersections.remove(&name),
            Some(i) => world.intersections.insert(name, i),
        };
        world
    });

    steps.then_regex(r#"^(i) = (i1|i2|i4)$"#, |world, ctx| {
        let intersection = world.intersections.get(&ctx.matches[1]).unwrap().clone();
        let desired = world.intersections.get(&ctx.matches[2]).unwrap().clone();
        assert_eq!(intersection, desired);
        world
    });

    steps.then_regex(r#"^(i) is nothing$"#, |world, ctx| {
        let intersection = world.intersections.get(&ctx.matches[1]);
        assert_eq!(intersection, None);
        world
    });

    steps
}
