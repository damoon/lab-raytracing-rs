use crate::{
    steps::tuples::{parse_float, parse_point, parse_vector},
    MyWorld,
};
use approx::assert_abs_diff_eq;
use cucumber_rust::Steps;
use lab_raytracing_rs::intersections::{hit, prepare_computations, schlick, Intersection};

pub fn steps() -> Steps<MyWorld> {
    let mut steps: Steps<MyWorld> = Steps::new();

    steps.when_regex(
        r#"^(i) ← intersection\(([-0-9.]+), s\)$"#,
        |mut world, ctx| {
            let t = ctx.matches[2].parse::<f64>().unwrap();
            let object = world.shapes.get("s").unwrap().clone();
            let intersection = Intersection { t, object };
            world
                .intersections
                .insert(ctx.matches[1].clone(), intersection);
            world
        },
    );

    steps.given_regex(
        r#"^(i|i1|i2|i3|i4) ← intersection\((√2|[-0-9.]+), (s|s2|shape)\)$"#,
        |mut world, ctx| {
            let t = match ctx.matches[2].as_str() {
                "√2" => 2.0_f64.sqrt(),
                s => s.parse::<f64>().unwrap(),
            };
            let object = world.shapes.get(&ctx.matches[3]).unwrap().clone();
            let intersection = Intersection { t, object };
            world
                .intersections
                .insert(ctx.matches[1].clone(), intersection);
            world
        },
    );

    steps.then_regex(r#"^(i).object = s$"#, |world, ctx| {
        let shape = &world.intersections.get(&ctx.matches[1]).unwrap().object;
        let desired = world.shapes.get("s").unwrap();
        assert_eq!(shape, desired);
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

    steps.given_regex(r#"^xs ← intersections\((i)\)$"#, |mut world, ctx| {
        let i1 = world.intersections.get(&ctx.matches[1]).unwrap().clone();
        world.xs = vec![i1];
        world
    });

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
        assert!((world.xs.get(index).unwrap().t - desired).abs() < 0.0001);
        world
    });

    steps.when_regex(r#"^(i) ← hit\(xs\)$"#, |mut world, ctx| {
        let intersection = hit(&world.xs);
        match intersection {
            None => world.intersections.remove(&ctx.matches[1]),
            Some(i) => world.intersections.insert(ctx.matches[1].clone(), i),
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
        world.comps = prepare_computations(intersection.clone(), &world.r, &world.xs);
        world
    });

    steps.when(
        "comps ← prepare_computations(i, r, xs)",
        |mut world, _ctx| {
            let intersection = world.intersections.get("i").unwrap();
            world.comps = prepare_computations(intersection.clone(), &world.r, &world.xs);
            world
        },
    );

    steps.when_regex(
        r#"comps ← prepare_computations\(xs\[([0-9]+)\], r, xs\)"#,
        |mut world, ctx| {
            let index = ctx.matches[1].parse::<usize>().unwrap();
            let intersection = &world.xs[index];
            world.comps = prepare_computations(intersection.clone(), &world.r, &world.xs);
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

    steps.then("comps.over_point.z < -EPSILON/2", |world, _ctx| {
        let maximum = -0.0001 / 2.0;
        assert!(world.comps.over_point.z < maximum);
        world
    });

    steps.then("comps.under_point.z > EPSILON/2", |world, _ctx| {
        let minimum = 0.0001 / 2.0;
        assert!(world.comps.under_point.z > minimum);
        world
    });

    steps.then("comps.point.z > comps.over_point.z", |world, _ctx| {
        assert!(world.comps.point.z > world.comps.over_point.z);
        world
    });

    steps.then("comps.point.z < comps.under_point.z", |world, _ctx| {
        assert!(world.comps.point.z < world.comps.under_point.z);
        world
    });

    steps.given(
        "xs ← intersections(2:A, 2.75:B, 3.25:C, 4.75:B, 5.25:C, 6:A)",
        |mut world, _ctx| {
            world.xs = vec![
                Intersection {
                    t: 2.0,
                    object: world.shapes.get("A").unwrap().clone(),
                },
                Intersection {
                    t: 2.75,
                    object: world.shapes.get("B").unwrap().clone(),
                },
                Intersection {
                    t: 3.25,
                    object: world.shapes.get("C").unwrap().clone(),
                },
                Intersection {
                    t: 4.75,
                    object: world.shapes.get("B").unwrap().clone(),
                },
                Intersection {
                    t: 5.25,
                    object: world.shapes.get("C").unwrap().clone(),
                },
                Intersection {
                    t: 6.0,
                    object: world.shapes.get("A").unwrap().clone(),
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
                    object: world.shapes.get(&ctx.matches[2]).unwrap().clone(),
                },
                Intersection {
                    t: parse_float(&ctx.matches[3]),
                    object: world.shapes.get(&ctx.matches[4]).unwrap().clone(),
                },
                Intersection {
                    t: parse_float(&ctx.matches[5]),
                    object: world.shapes.get(&ctx.matches[6]).unwrap().clone(),
                },
                Intersection {
                    t: parse_float(&ctx.matches[7]),
                    object: world.shapes.get(&ctx.matches[8]).unwrap().clone(),
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
                    object: world.shapes.get(&ctx.matches[2]).unwrap().clone(),
                },
                Intersection {
                    t: parse_float(&ctx.matches[3]),
                    object: world.shapes.get(&ctx.matches[4]).unwrap().clone(),
                },
            ];
            world
        },
    );

    steps.given_regex(
        r#"xs ← intersections\(([-√/0-9\.]+):(floor|shape)\)"#,
        |mut world, ctx| {
            world.xs = vec![Intersection {
                t: parse_float(&ctx.matches[1]),
                object: world.shapes.get(&ctx.matches[2]).unwrap().clone(),
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
        assert!((desired - lookup).abs() < 0.0001);
        world
    });

    steps
}
