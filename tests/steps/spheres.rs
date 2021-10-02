use crate::MyWorld;
use approx::assert_abs_diff_eq;
use cucumber_rust::Steps;
use lab_raytracing_rs::{
    matrices::Matrix4x4,
    spheres::Sphere,
    transformations::scaling,
    tuples::{color, point, Tuple},
};
use regex::Regex;

use super::transformations::{parse_scaling, parse_translation};

pub fn steps() -> Steps<MyWorld> {
    let mut steps: Steps<MyWorld> = Steps::new();

    steps.given_regex(r#"^(s|shape) ← sphere\(\)$"#, |mut world, ctx| {
        match ctx.matches[1].as_str() {
            "s" => world.s = Sphere::default(),
            "shape" => world.shape = Sphere::default(),
            _ => panic!("object name not covered"),
        };

        world
    });

    steps.given_regex(r#"^(s1|s2) ← sphere\(\) with:$"#, |mut world, ctx| {
        let mut s = Sphere::default();

        for row in &ctx.step.table.as_ref().unwrap().rows {
            let key = row.get(0).unwrap().clone();
            let value = row.get(1).unwrap();
            match key.as_str() {
                "material.color" => s.material.color = color_from_string(value),
                "material.diffuse" => s.material.diffuse = value.parse::<f64>().unwrap(),
                "material.specular" => s.material.specular = value.parse::<f64>().unwrap(),
                "transform" => s.transform = transform_from_string(value),
                _ => panic!("sphere property not covered"),
            }
        }

        match ctx.matches[1].as_str() {
            "s1" => world.s1 = s,
            "s2" => world.s2 = s,
            _ => panic!("object name not covered"),
        };
        world
    });

    steps.when_regex(r#"^m ← s.material$"#, |mut world, _ctx| {
        world.m = world.s.material.clone();
        world
    });

    steps.when("s.material ← m", |mut world, _ctx| {
        world.s.material = world.m.clone();
        world
    });

    steps.then("s.material = m", |world, _ctx| {
        assert_eq!(world.s.material, world.m);
        world
    });

    steps.given_regex(
        r#"^(t) ← translation\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |mut world, ctx| {
            let translation = parse_translation(&ctx.matches[2..=4]);
            world.insert4x4(ctx.matches[1].clone(), translation);
            world
        },
    );

    steps.given_regex(r#"^set_transform\(s, (m)\)$"#, |mut world, ctx| {
        let transformation = world.get4x4(&ctx.matches[1]);
        world.s.transform = transformation;
        world
    });

    steps.when_regex(r#"^set_transform\(s, (t)\)$"#, |mut world, ctx| {
        let transformation = world.get4x4(&ctx.matches[1]);
        world.s.transform = transformation;
        world
    });

    steps.given_regex(
        r#"^set_transform\(s, (scaling|translation)\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)\)$"#,
        |mut world, ctx| {
            world.s.transform = match ctx.matches[1].as_str() {
                "scaling" => parse_scaling(&ctx.matches[2..=4]),
                "translation" => parse_translation(&ctx.matches[2..=4]),
                _ => panic!("transformation not covered"),
            };
            world
        },
    );

    steps.when_regex(
        r#"^set_transform\(s, (scaling|translation)\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)\)$"#,
        |mut world, ctx| {
            world.s.transform = match ctx.matches[1].as_str() {
                "scaling" => parse_scaling(&ctx.matches[2..=4]),
                "translation" => parse_translation(&ctx.matches[2..=4]),
                _ => panic!("transformation not covered"),
            };
            world
        },
    );

    steps.then_regex(r#"^s.transform = (identity_matrix|t)$"#, |world, ctx| {
        let transform = &world.s.transform;
        let matrix = &world.get4x4(ctx.matches[1].as_str());
        assert_eq!(transform, matrix);
        world
    });

    steps.when_regex(r#"^xs ← intersect\(s, r\)$"#, |mut world, _ctx| {
        world.xs = world.s.intersect(&world.r);
        world
    });

    steps.then_regex(r#"^xs.count = ([-0-9.]+)$"#, |world, ctx| {
        let c = ctx.matches[1].parse::<usize>().unwrap();
        assert_eq!(world.xs.len(), c);
        world
    });

    steps.then_regex(r#"^xs\[([-0-9.]+)\] = ([-0-9.]+)$"#, |world, ctx| {
        let index = ctx.matches[1].parse::<usize>().unwrap();
        let desired = ctx.matches[2].parse::<f64>().unwrap();
        let value = world.xs.get(index).unwrap().t;
        assert_abs_diff_eq!(value, desired);
        world
    });

    steps.then_regex(r#"^xs\[([-0-9.]+)\].object = s$"#, |world, ctx| {
        let index = ctx.matches[1].parse::<usize>().unwrap();
        assert_eq!(world.xs.get(index).unwrap().object, world.s);
        world
    });

    steps.when_regex(
        r#"^(n) ← normal_at\(s, point\((√3/3|[-0-9.]+), (√3/3|√2/2|[-0-9.]+), (√3/3|-√2/2|[-0-9.]+)\)\)$"#,
        |mut world, ctx| {
            let x = match ctx.matches[2].as_str() {
                "√3/3" => 3.0_f64.sqrt()/3.0,
                s => s.parse::<f64>().unwrap()
            };            let y = match ctx.matches[3].as_str() {
                "√3/3" => 3.0_f64.sqrt()/3.0,
                "√2/2" => 2.0_f64.sqrt()/2.0,
                s => s.parse::<f64>().unwrap()
            };            let z = match ctx.matches[4].as_str() {
                "√3/3" => 3.0_f64.sqrt()/3.0,
                "-√2/2" => -(2.0_f64.sqrt()/2.0),
                s => s.parse::<f64>().unwrap()
            };
            let point = point(x, y, z);
            let normal = world.s.normal_at(point);
            world.tuples.insert(ctx.matches[1].clone(), normal);
            world
        },
    );

    steps
}

fn color_from_string(s: &str) -> Tuple {
    let re = Regex::new(r#"\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)"#).unwrap();
    let captures = re.captures(s).unwrap();
    let r = captures.get(1).unwrap().as_str().parse::<f64>().unwrap();
    let g = captures.get(2).unwrap().as_str().parse::<f64>().unwrap();
    let b = captures.get(3).unwrap().as_str().parse::<f64>().unwrap();
    color(r, g, b)
}

fn transform_from_string(s: &str) -> Matrix4x4 {
    let re = Regex::new(r#"(scaling)\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)"#).unwrap();
    let captures = re.captures(s).unwrap();
    let x = captures.get(2).unwrap().as_str().parse::<f64>().unwrap();
    let y = captures.get(3).unwrap().as_str().parse::<f64>().unwrap();
    let z = captures.get(4).unwrap().as_str().parse::<f64>().unwrap();
    match captures.get(1).unwrap().as_str() {
        "scaling" => scaling(x, y, z),
        _ => panic!("transformation not covered"),
    }
}
