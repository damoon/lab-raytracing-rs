use approx::assert_abs_diff_eq;
use cucumber_rust::Steps;
use lab_raytracing_rs::tuples::{cross, dot, point, vector, Tuple};

use crate::MyWorld;

pub fn steps() -> Steps<MyWorld> {
    let mut steps: Steps<MyWorld> = Steps::new();

    steps.given_regex(
        r#"([a-z0-9]+) ← tuple\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |mut world, ctx| {
            let name = ctx.matches[1].clone();
            let x = ctx.matches[2].parse::<f64>().unwrap();
            let y = ctx.matches[3].parse::<f64>().unwrap();
            let z = ctx.matches[4].parse::<f64>().unwrap();
            let w = ctx.matches[5].parse::<f64>().unwrap();
            let tuple = Tuple::new(x, y, z, w);
            world.tuples.insert(name, tuple);
            world
        },
    );

    steps.then_regex(r#"^([a-z]+).(x|y|z|w) = ([-0-9.]+)$"#, |world, ctx| {
        let name = ctx.matches[1].clone();
        let attr = ctx.matches[2].clone();
        let desired = ctx.matches[3].parse::<f64>().unwrap();
        let tuple = world.tuples.get(&name).unwrap();
        let value = match attr.as_str() {
            "x" => tuple.x,
            "y" => tuple.y,
            "z" => tuple.z,
            "w" => tuple.w,
            _ => panic!("Invalid attribute checked"),
        };
        assert_abs_diff_eq!(desired, value);

        world
    });

    steps.then_regex(r#"^([a-z]+) is a point$"#, |world, ctx| {
        let name = ctx.matches[1].clone();
        let tuple = world.tuples.get(&name).unwrap();
        assert!(tuple.is_point());

        world
    });

    steps.then_regex(r#"^([a-z]+) is not a point$"#, |world, ctx| {
        let name = ctx.matches[1].clone();
        let tuple = world.tuples.get(&name).unwrap();
        assert!(!tuple.is_point());

        world
    });

    steps.then_regex(r#"^([a-z]+) is a vector$"#, |world, ctx| {
        let name = ctx.matches[1].clone();
        let tuple = world.tuples.get(&name).unwrap();
        assert!(tuple.is_vector());

        world
    });

    steps.then_regex(r#"^([a-z]+) is not a vector$"#, |world, ctx| {
        let name = ctx.matches[1].clone();
        let tuple = world.tuples.get(&name).unwrap();
        assert!(!tuple.is_vector());

        world
    });

    steps.given_regex(
        r#"^([a-z0-9]+) ← vector\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |mut world, ctx| {
            let name = ctx.matches[1].clone();
            let x = ctx.matches[2].parse::<f64>().unwrap();
            let y = ctx.matches[3].parse::<f64>().unwrap();
            let z = ctx.matches[4].parse::<f64>().unwrap();
            let t = vector(x, y, z);
            world.tuples.insert(name, t);

            world
        },
    );

    steps.given_regex(
        r#"^([a-z0-9]+) ← point\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |mut world, ctx| {
            let name = ctx.matches[1].clone();
            let x = ctx.matches[2].parse::<f64>().unwrap();
            let y = ctx.matches[3].parse::<f64>().unwrap();
            let z = ctx.matches[4].parse::<f64>().unwrap();
            let t = point(x, y, z);
            world.tuples.insert(name, t);

            world
        },
    );

    steps.given_regex(
        r#"^([a-z]+) ← vector\(√2/2, √2/2, 0\)$"#,
        |mut world, ctx| {
            let name = ctx.matches[1].clone();
            let t = vector(2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0);
            world.tuples.insert(name, t);

            world
        },
    );

    steps.then_regex(
        r#"^([a-z]+) = tuple\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |world, ctx| {
            let name = ctx.matches[1].clone();
            let x = ctx.matches[2].parse::<f64>().unwrap();
            let y = ctx.matches[3].parse::<f64>().unwrap();
            let z = ctx.matches[4].parse::<f64>().unwrap();
            let w = ctx.matches[5].parse::<f64>().unwrap();
            let desired_tuple = Tuple::new(x, y, z, w);
            let tuple = world.tuples.get(&name).unwrap();
            assert_eq!(&desired_tuple, tuple);

            world
        },
    );

    steps.then_regex(
        r#"^([a-z0-9]+) = point\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |world, ctx| {
            let name = ctx.matches[1].clone();
            let x = ctx.matches[2].parse::<f64>().unwrap();
            let y = ctx.matches[3].parse::<f64>().unwrap();
            let z = ctx.matches[4].parse::<f64>().unwrap();
            let desired = point(x, y, z);
            let tuple = world.tuples.get(&name).unwrap();
            assert_eq!(&desired, tuple);

            world
        },
    );

    steps.then_regex(
        r#"^([a-z0-9]+) \+ ([a-z0-9]+) = tuple\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |world, ctx| {
            let name1 = ctx.matches[1].clone();
            let name2 = ctx.matches[2].clone();
            let tuple1 = world.tuples.get(&name1).unwrap();
            let tuple2 = world.tuples.get(&name2).unwrap();
            let added_tuple = tuple1 + tuple2;
            let x = ctx.matches[3].parse::<f64>().unwrap();
            let y = ctx.matches[4].parse::<f64>().unwrap();
            let z = ctx.matches[5].parse::<f64>().unwrap();
            let w = ctx.matches[6].parse::<f64>().unwrap();
            let desired = Tuple::new(x, y, z, w);
            assert_eq!(added_tuple, desired);

            world
        },
    );

    steps.then_regex(
        r#"^([a-z0-9]+) \- ([a-z0-9]+) = vector\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |world, ctx| {
            let name1 = ctx.matches[1].clone();
            let name2 = ctx.matches[2].clone();
            let tuple1 = world.tuples.get(&name1).unwrap();
            let tuple2 = world.tuples.get(&name2).unwrap();
            let added_tuple = tuple1 - tuple2;
            let x = ctx.matches[3].parse::<f64>().unwrap();
            let y = ctx.matches[4].parse::<f64>().unwrap();
            let z = ctx.matches[5].parse::<f64>().unwrap();
            let desired = vector(x, y, z);
            assert_eq!(added_tuple, desired);

            world
        },
    );

    steps.then_regex(
        r#"^([a-z0-9]+) \- ([a-z0-9]+) = point\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |world, ctx| {
            let name1 = ctx.matches[1].clone();
            let name2 = ctx.matches[2].clone();
            let tuple1 = world.tuples.get(&name1).unwrap();
            let tuple2 = world.tuples.get(&name2).unwrap();
            let added_tuple = tuple1 - tuple2;
            let x = ctx.matches[3].parse::<f64>().unwrap();
            let y = ctx.matches[4].parse::<f64>().unwrap();
            let z = ctx.matches[5].parse::<f64>().unwrap();
            let desired = point(x, y, z);
            assert_eq!(added_tuple, desired);

            world
        },
    );

    steps.then_regex(
        r#"^-([a-z]+) = tuple\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |world, ctx| {
            let name = ctx.matches[1].clone();
            let tuple = world.tuples.get(&name).unwrap();
            let x = ctx.matches[2].parse::<f64>().unwrap();
            let y = ctx.matches[3].parse::<f64>().unwrap();
            let z = ctx.matches[4].parse::<f64>().unwrap();
            let w = ctx.matches[5].parse::<f64>().unwrap();
            let desired = Tuple::new(x, y, z, w);
            assert_eq!(-tuple, desired);

            world
        },
    );

    steps.then_regex(
        r#"^([a-z]+) \* ([-0-9.]+) = tuple\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |world, ctx| {
            let name = ctx.matches[1].clone();
            let tuple = world.tuples.get(&name).unwrap();
            let multiplicator = ctx.matches[2].parse::<f64>().unwrap();
            let calculated = tuple * multiplicator;
            let x = ctx.matches[3].parse::<f64>().unwrap();
            let y = ctx.matches[4].parse::<f64>().unwrap();
            let z = ctx.matches[5].parse::<f64>().unwrap();
            let w = ctx.matches[6].parse::<f64>().unwrap();
            let desired = Tuple::new(x, y, z, w);
            assert_eq!(calculated, desired);

            world
        },
    );

    steps.then_regex(
        r#"^([a-z]+) / ([-0-9.]+) = tuple\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |world, ctx| {
            let name = ctx.matches[1].clone();
            let tuple = world.tuples.get(&name).unwrap();
            let divisor = ctx.matches[2].parse::<f64>().unwrap();
            let x = ctx.matches[3].parse::<f64>().unwrap();
            let y = ctx.matches[4].parse::<f64>().unwrap();
            let z = ctx.matches[5].parse::<f64>().unwrap();
            let w = ctx.matches[6].parse::<f64>().unwrap();
            let desired = Tuple::new(x, y, z, w);
            assert_eq!(tuple / divisor, desired);

            world
        },
    );

    steps.then_regex(r#"^magnitude\(([a-z]+)\) = ([-0-9.]+)$"#, |world, ctx| {
        let name = ctx.matches[1].clone();
        let tuple = world.tuples.get(&name).unwrap();
        let desired = ctx.matches[2].parse::<f64>().unwrap();
        let magnitude = tuple.magnitude();

        assert_abs_diff_eq!(desired, magnitude);

        world
    });

    steps.then_regex(
        r#"^magnitude\(([a-z]+)\) = √([-0-9.]+)$"#,
        |world, ctx| {
            let name = ctx.matches[1].clone();
            let tuple = world.tuples.get(&name).unwrap();
            let desired = ctx.matches[2].parse::<f64>().unwrap().sqrt();
            let magnitude = tuple.magnitude();

            assert_abs_diff_eq!(desired, magnitude);

            world
        },
    );

    steps.then_regex(
        r#"^normalize\(([a-z]+)\) = vector\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |world, ctx| {
            let name = ctx.matches[1].clone();
            let tuple = world.tuples.get(&name).unwrap();
            let x = ctx.matches[2].parse::<f64>().unwrap();
            let y = ctx.matches[3].parse::<f64>().unwrap();
            let z = ctx.matches[4].parse::<f64>().unwrap();
            let v = vector(x, y, z);
            assert_eq!(tuple.normalize(), v);

            world
        },
    );

    steps.then_regex(
        r#"^normalize\(([a-z]+)\) = approximately vector\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |world, ctx| {
            let name = ctx.matches[1].clone();
            let tuple = world.tuples.get(&name).unwrap();
            let x = ctx.matches[2].parse::<f64>().unwrap();
            let y = ctx.matches[3].parse::<f64>().unwrap();
            let z = ctx.matches[4].parse::<f64>().unwrap();
            let desired = vector(x, y, z);
            let calculated = tuple.normalize();
            assert_eq!(calculated, desired);

            world
        },
    );

    steps.when_regex(
        r#"^([a-z]+) ← normalize\(([a-z]+)\)$"#,
        |mut world, ctx| {
            let name1 = ctx.matches[1].clone();
            let name2 = ctx.matches[2].clone();
            let tuple = world.tuples.get(&name2).unwrap().normalize();
            world.tuples.insert(name1, tuple);

            world
        },
    );

    steps.then_regex(
        r#"^dot\(([a-z]+), ([a-z]+)\) = ([-0-9.]+)$"#,
        |world, ctx| {
            let name1 = ctx.matches[1].clone();
            let name2 = ctx.matches[2].clone();
            let tuple1 = world.tuples.get(&name1).unwrap().clone();
            let tuple2 = world.tuples.get(&name2).unwrap().clone();
            let dot = dot(tuple1, tuple2);
            let desired = ctx.matches[3].parse::<f64>().unwrap();

            assert_abs_diff_eq!(dot, desired);

            world
        },
    );

    steps.then_regex(
        r#"^cross\(([a-z]+), ([a-z]+)\) = vector\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |world, ctx| {
            let name1 = ctx.matches[1].clone();
            let name2 = ctx.matches[2].clone();
            let tuple1 = world.tuples.get(&name1).unwrap().clone();
            let tuple2 = world.tuples.get(&name2).unwrap().clone();
            let cross = cross(tuple1, tuple2);
            let x = ctx.matches[3].parse::<f64>().unwrap();
            let y = ctx.matches[4].parse::<f64>().unwrap();
            let z = ctx.matches[5].parse::<f64>().unwrap();
            let v = vector(x, y, z);
            assert_eq!(cross, v);

            world
        },
    );

    steps
}
