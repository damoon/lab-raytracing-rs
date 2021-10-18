use crate::{steps::transformations::parse_scaling, MyWorld};
use cucumber_rust::Steps;
use lab_raytracing_rs::{groups::Group, transformations::rotation_y};
use std::{f64::consts::PI, ops::Deref};

pub fn steps() -> Steps<MyWorld> {
    let mut steps: Steps<MyWorld> = Steps::new();

    steps.given_regex(r#"^(g|g1|g2) ← group\(\)$"#, |mut world, ctx| {
        let grp = Group::default();
        match ctx.matches[1].as_str() {
            "g" => world.g = grp,
            "g1" => world.g1 = grp,
            "g2" => world.g2 = grp,
            _ => panic!("group not covered"),
        }
        world
    });

    steps.then_regex(r#"^g.transform = (identity_matrix)$"#, |world, _ctx| {
        let matrix = world.get4x4("identity_matrix");
        assert_eq!(&world.g.transform(), &matrix);
        world
    });

    steps.then_regex(r#"^g is empty$"#, |world, _ctx| {
        assert!(world.g.is_empty());
        world
    });

    steps.then_regex(r#"^g is not empty$"#, |world, _ctx| {
        assert!(!world.g.is_empty());
        world
    });

    // steps.then_regex(r#"^g includes s$"#, |world, _ctx| {
    //     let object = world.objects.get("s").unwrap().clone();
    //     assert!(world.g.contains_object(object));
    //     world
    // });

    steps.given_regex(
        r#"^set_transform\((g1), rotation_y\(π/2\)\)$"#,
        |mut world, ctx| {
            let transform = rotation_y(PI / 2.0);
            match ctx.matches[1].as_str() {
                "g1" => world.g1.set_transform(transform),
                _ => panic!("group not covered"),
            }
            world
        },
    );

    steps.given_regex(
        r#"^set_transform\((g|g2), scaling\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)\)$"#,
        |mut world, ctx| {
            let transform = parse_scaling(&ctx.matches[2..=4]);
            match ctx.matches[1].as_str() {
                "g" => world.g.set_transform(transform),
                "g2" => world.g2.set_transform(transform),
                _ => panic!("group not covered"),
            }
            world
        },
    );

    steps.given_regex(
        r#"^add_child\((g|g1|g2), (s|s1|s2|s3)\)$"#,
        |mut world, ctx| {
            let obj = world.objects.get(&ctx.matches[2]).unwrap().deref().clone();
            match ctx.matches[1].as_str() {
                "g" => world.g.add_object(obj),
                "g1" => world.g1.add_object(obj),
                "g2" => world.g1.add_object(obj),
                _ => panic!("group not covered"),
            }
            world
        },
    );

    steps.given_regex(r#"^add_child\((g1), (g2)\)$"#, |mut world, ctx| {
        match ctx.matches[1].as_str() {
            "g1" => world.g1.add_group(world.g2.clone()),
            _ => panic!("group not covered"),
        }
        world
    });

    steps.when_regex(r#"^add_child\((g), (s)\)$"#, |mut world, ctx| {
        let obj = world.objects.get(&ctx.matches[2]).unwrap().deref().clone();
        match ctx.matches[1].as_str() {
            "g" => world.g.add_object(obj),
            _ => panic!("group not covered"),
        }
        world
    });

    steps.when_regex(r#"^xs ← local_intersect\(g, r\)$"#, |mut world, _ctx| {
        world.xs = world.g.local_intersect(&world.r);
        world
    });

    steps.when_regex(r#"^xs ← intersect\(g, r\)$"#, |mut world, _ctx| {
        world.xs = world.g.intersect(&world.r);
        world
    });

    steps
}
