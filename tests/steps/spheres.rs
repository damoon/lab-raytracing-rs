use crate::MyWorld;
use approx::assert_abs_diff_eq;
use cucumber_rust::Steps;
use lab_raytracing_rs::{
    matrices::Matrix4x4,
    spheres::Sphere,
    transformations::{scaling, translation},
    tuples::{color, point, Tuple},
};
use regex::Regex;

use super::transformations::{parse_scaling, parse_translation};

pub fn steps() -> Steps<MyWorld> {
    let mut steps: Steps<MyWorld> = Steps::new();

    steps.given_regex(
        r#"^(s|shape|s1|object) ← sphere\(\)$"#,
        |mut world, ctx| {
            let shape = Sphere::default();
            world.shapes.insert(ctx.matches[1].clone(), shape);
            world
        },
    );

    steps.given_regex(
        r#"^(s1|s2|shape) ← sphere\(\) with:$"#,
        |mut world, ctx| {
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
            world.shapes.insert(ctx.matches[1].to_string(), s);
            world
        },
    );

    steps.when_regex(r#"^m ← s.material$"#, |mut world, _ctx| {
        world.m = world.shapes.get("s").unwrap().material.clone();
        world
    });

    steps.when("s.material ← m", |mut world, _ctx| {
        world.shapes.get_mut("s").unwrap().material = world.m.clone();
        world
    });

    steps.then("s.material = m", |world, _ctx| {
        let s = world.shapes.get("s").unwrap();
        assert_eq!(s.material, world.m);
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
        world.shapes.get_mut("s").unwrap().transform = transformation;
        world
    });

    steps.when_regex(r#"^set_transform\(s, (t|m)\)$"#, |mut world, ctx| {
        let transformation = world.get4x4(&ctx.matches[1]);
        world.shapes.get_mut("s").unwrap().transform = transformation;
        world
    });

    steps.given_regex(
        r#"^set_transform\((s|object|shape), (scaling|translation)\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)\)$"#,
        |mut world, ctx| {
            let transformation = match ctx.matches[2].as_str() {
                "scaling" => parse_scaling(&ctx.matches[3..=5]),
                "translation" => parse_translation(&ctx.matches[3..=5]),
                _ => panic!("transformation not covered"),
            };
            world.shapes.get_mut(&ctx.matches[1]).unwrap().transform = transformation;
            world
        },
    );

    steps.when_regex(
        r#"^set_transform\(s, (scaling|translation)\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)\)$"#,
        |mut world, ctx| {
            let transformation = match ctx.matches[1].as_str() {
                "scaling" => parse_scaling(&ctx.matches[2..=4]),
                "translation" => parse_translation(&ctx.matches[2..=4]),
                _ => panic!("transformation not covered"),
            };
            world.shapes.get_mut("s").unwrap().transform = transformation;
            world
        },
    );

    steps.then_regex(r#"^s.transform = (identity_matrix|t)$"#, |world, ctx| {
        let lookup = world.shapes.get("s").unwrap().transform.clone();
        let desired = &world.get4x4(ctx.matches[1].as_str());
        assert_eq!(&lookup, desired);
        world
    });

    steps.when_regex(r#"^xs ← intersect\(s, r\)$"#, |mut world, _ctx| {
        let s = world.shapes.get("s").unwrap();
        world.xs = s.intersect(&world.r);
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
        let desired = world.shapes.get("s").unwrap();
        let lookup = world.xs.get(index).unwrap().object.clone();
        assert_eq!(&lookup, desired);
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
            let normal = world.shapes.get("s").unwrap().normal_at(&point);
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
    let re = Regex::new(r#"(scaling|translation)\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)"#).unwrap();
    let captures = re.captures(s).expect("transform not covered");
    let x = captures.get(2).unwrap().as_str().parse::<f64>().unwrap();
    let y = captures.get(3).unwrap().as_str().parse::<f64>().unwrap();
    let z = captures.get(4).unwrap().as_str().parse::<f64>().unwrap();
    match captures.get(1).unwrap().as_str() {
        "scaling" => scaling(x, y, z),
        "translation" => translation(x, y, z),
        _ => panic!("transformation not covered"),
    }
}
