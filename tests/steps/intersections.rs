use crate::{
    steps::tuples::{parse_float, parse_point, parse_vector},
    MyWorld,
};
use approx::assert_abs_diff_eq;
use cucumber::{given, then, when};
use lab_raytracing_rs::intersections::{hit, prepare_computations, schlick, Intersection};
use std::sync::Arc;

#[when(regex = r"(i) ← intersection\(([-0-9.]+), (s)\)$")]
#[given(regex = r"(i|i1|i2|i3|i4) ← intersection\((√2|[-0-9.]+), (s|s2|shape)\)$")]
async fn assign_intersection(world: &mut MyWorld, target: String, t: String, shape: String) {
    let t = parse_float(t.as_str());
    let object = world.objects.get(&shape).unwrap().clone();
    let intersection = Intersection { t, object };
    world.intersections.insert(target, intersection);
}

#[then(regex = r"(i).object = s$")]
async fn compare_intersection_object(world: &mut MyWorld, intersection: String) {
    let shape = &world.intersections.get(&intersection).unwrap().object;
    let desired = world.objects.get("s").unwrap();
    assert!(Arc::ptr_eq(shape, desired));
}

#[then(regex = r"(i).t = ([-0-9.]+)$")]
async fn compare_intersection_distance(world: &mut MyWorld, intersection: String, desired: f64) {
    let intersection = world.intersections.get(&intersection).unwrap().clone();
    assert_abs_diff_eq!(desired, intersection.t);
}

#[when(regex = r"xs ← intersections\((i1), (i2)\)$")]
#[given(regex = r"xs ← intersections\((i2), (i1)\)$")]
async fn assign_intersections(world: &mut MyWorld, i1: String, i2: String) {
    let i1 = world.intersections.get(&i1).unwrap().clone();
    let i2 = world.intersections.get(&i2).unwrap().clone();
    world.xs = vec![i1, i2];
}

#[given(regex = r"xs ← intersections\((i)\)$")]
async fn assign_intersections_single(world: &mut MyWorld, i1: String) {
    let i1 = world.intersections.get(&i1).unwrap().clone();
    world.xs = vec![i1];
}

#[given(regex = r"xs ← intersections\((i1), (i2), (i3), (i4)\)$")]
async fn assign_intersections_quad(
    world: &mut MyWorld,
    i1: String,
    i2: String,
    i3: String,
    i4: String,
) {
    let i1 = world.intersections.get(&i1).unwrap().clone();
    let i2 = world.intersections.get(&i2).unwrap().clone();
    let i3 = world.intersections.get(&i3).unwrap().clone();
    let i4 = world.intersections.get(&i4).unwrap().clone();
    world.xs = vec![i1, i2, i3, i4];
}

/*
    steps.then_regex(r#"^xs\[([-0-9.]+)\].t = ([-0-9.]+)$"#, |world, ctx| {
        let index = ctx.matches[1].parse::<usize>().unwrap();
        let desired = ctx.matches[2].parse::<f64>().unwrap();
        assert!((world.xs.get(index).unwrap().t - desired).abs() < 0.0001);
        world
    });

    steps.when_regex(r#"^(i) ← hit\(xs\)$"#, |mut world, ctx| {
        let intersection = hit(&world.xs, None);
        match intersection {
            None => world.intersections.remove(&ctx.matches[1]),
            Some(i) => world
                .intersections
                .insert(ctx.matches[1].clone(), i.clone()),
        };
        world
    });

    steps.then_regex(r#"^(i) = (i1|i2|i4)$"#, |world, ctx| {
        let intersection = world.intersections.get(&ctx.matches[1]).unwrap();
        let desired = world.intersections.get(&ctx.matches[2]).unwrap();
        assert_eq!(intersection, desired);
        world
    });

    steps.then_regex(r#"^(i) is nothing$"#, |world, ctx| {
        let intersection = world.intersections.get(&ctx.matches[1]);
        assert_eq!(intersection, None);
        world
    });

    steps.when("comps ← prepare_computations(i, r)", |mut world, _ctx| {
        let intersection = world.intersections.get("i").unwrap();
        world.comps = prepare_computations(intersection, &world.r, &world.xs);
        world
    });

    steps.when(
        "comps ← prepare_computations(i, r, xs)",
        |mut world, _ctx| {
            let intersection = world.intersections.get("i").unwrap();
            world.comps = prepare_computations(intersection, &world.r, &world.xs);
            world
        },
    );

    steps.when_regex(
        r#"comps ← prepare_computations\(xs\[([0-9]+)\], r, xs\)"#,
        |mut world, ctx| {
            let index = ctx.matches[1].parse::<usize>().unwrap();
            let intersection = &world.xs[index];
            world.comps = prepare_computations(intersection, &world.r, &world.xs);
            world
        },
    );

    steps.then("comps.t = i.t", |world, _ctx| {
        let intersection = world.intersections.get("i").unwrap();
        assert_abs_diff_eq!(world.comps.t, intersection.t);
        world
    });

    steps.then("comps.object = i.object", |world, _ctx| {
        let lookup = &world.comps.object;
        let desired = &world.intersections.get("i").unwrap().object;
        assert_eq!(lookup, desired);
        world
    });

    steps.then_regex(
        r#"^comps\.(point|eyev|normalv|reflectv) = (point|vector)\((√2/2|[-0-9.]+), (√2/2|[-0-9.]+), (√2/2|[-0-9.]+)\)$"#,
        |world, ctx| {
            let tuple = match ctx.matches[2].as_str() {
                "point" => parse_point(&ctx.matches[3..=5]),
                "vector" => parse_vector(&ctx.matches[3..=5]),
                _ => panic!("type not covered"),
            };
            match ctx.matches[1].as_str() {
                "point" => assert_eq!(world.comps.point, tuple),
                "eyev" => assert_eq!(world.comps.eyev, tuple),
                "normalv" => assert_eq!(world.comps.normalv, tuple),
                "reflectv" => assert_eq!(world.comps.reflectv, tuple),
                _ => panic!("type not covered"),
            };
            world
        },
    );

    steps.then_regex(r#"^comps\.(n1|n2) = ([-0-9.]+)$"#, |world, ctx| {
        let desired = ctx.matches[2].parse::<f64>().unwrap();
        match ctx.matches[1].as_str() {
            "n1" => assert_abs_diff_eq!(world.comps.n1, desired),
            "n2" => assert_abs_diff_eq!(world.comps.n2, desired),
            _ => panic!("type not covered"),
        };
        world
    });

    steps.then_regex(r#"^comps\.inside = (true|false)$"#, |world, ctx| {
        let desired = match ctx.matches[1].as_str() {
            "true" => true,
            "false" => false,
            _ => panic!("only true and false are valid values"),
        };
        assert_eq!(world.comps.inside, desired);
        world
    });

    steps.then("comps.over_point.z < -EPSILON/2", |world, _ctx| world);

    steps.then("comps.under_point.z > EPSILON/2", |world, _ctx| world);

    steps.then("comps.point.z > comps.over_point.z", |world, _ctx| world);

    steps.then("comps.point.z < comps.under_point.z", |world, _ctx| world);

    steps.given(
        "xs ← intersections(2:A, 2.75:B, 3.25:C, 4.75:B, 5.25:C, 6:A)",
        |mut world, _ctx| {
            world.xs = vec![
                Intersection {
                    t: 2.0,
                    object: world.objects.get("A").unwrap().clone(),
                },
                Intersection {
                    t: 2.75,
                    object: world.objects.get("B").unwrap().clone(),
                },
                Intersection {
                    t: 3.25,
                    object: world.objects.get("C").unwrap().clone(),
                },
                Intersection {
                    t: 4.75,
                    object: world.objects.get("B").unwrap().clone(),
                },
                Intersection {
                    t: 5.25,
                    object: world.objects.get("C").unwrap().clone(),
                },
                Intersection {
                    t: 6.0,
                    object: world.objects.get("A").unwrap().clone(),
                },
            ];
            world
        },
    );

    steps.given_regex(
        r#"xs ← intersections\(([-√/0-9\.]+):(A|B), ([-√/0-9\.]+):(A|B), ([-√/0-9\.]+):(A|B), ([-√/0-9\.]+):(A|B)\)"#,
        |mut world, ctx| {
            world.xs = vec![
                Intersection {
                    t: parse_float(&ctx.matches[1]),
                    object: world.objects.get(&ctx.matches[2]).unwrap().clone(),
                },
                Intersection {
                    t: parse_float(&ctx.matches[3]),
                    object: world.objects.get(&ctx.matches[4]).unwrap().clone(),
                },
                Intersection {
                    t: parse_float(&ctx.matches[5]),
                    object: world.objects.get(&ctx.matches[6]).unwrap().clone(),
                },
                Intersection {
                    t: parse_float(&ctx.matches[7]),
                    object: world.objects.get(&ctx.matches[8]).unwrap().clone(),
                },
            ];
            world
        },
    );

    steps.given_regex(
        r#"xs ← intersections\(([-√/0-9\.]+):(shape), ([-√/0-9\.]+):(shape)\)"#,
        |mut world, ctx| {
            world.xs = vec![
                Intersection {
                    t: parse_float(&ctx.matches[1]),
                    object: world.objects.get(&ctx.matches[2]).unwrap().clone(),
                },
                Intersection {
                    t: parse_float(&ctx.matches[3]),
                    object: world.objects.get(&ctx.matches[4]).unwrap().clone(),
                },
            ];
            world
        },
    );

    steps.given_regex(
        r#"xs ← intersections\(([-√/0-9\.]+):(floor|shape)\)"#,
        |mut world: MyWorld, ctx| {
            world.xs = vec![Intersection {
                t: parse_float(&ctx.matches[1]),
                object: world.objects.get(&ctx.matches[2]).unwrap().clone(),
            }];
            world
        },
    );

    steps.when("reflectance ← schlick(comps)", |mut world, _ctx| {
        let reflectance = schlick(&world.comps);
        world.floats.insert("reflectance".to_string(), reflectance);
        world
    });

    steps.then_regex(r#"(reflectance) = ([-√/0-9\.]+)"#, |world, ctx| {
        let desired = parse_float(&ctx.matches[2]);
        let lookup = world.floats.get(&ctx.matches[1]).unwrap();
        assert_abs_diff_eq!(&desired, lookup);
        world
    });

    steps
}
*/
