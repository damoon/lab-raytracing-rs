use super::{
    transformations::{parse_scaling, parse_translation},
    tuples::parse_point,
};
use crate::MyWorld;
use approx::assert_abs_diff_eq;
use cucumber_rust::Steps;
use lab_raytracing_rs::{
    matrices::Matrix4x4,
    patterns::test_pattern,
    planes::{default_cube, default_plane, glass_sphere},
    shapes::intersect, spheres::default_sphere,
    transformations::{scaling, translation},
    tuples::{color, Tuple}
};
use regex::Regex;
use std::{ops::Deref, rc::Rc};

pub fn steps() -> Steps<MyWorld> {
    let mut steps: Steps<MyWorld> = Steps::new();

    steps.given_regex(
        r#"^(s|shape|s1|object|c) ← (sphere|plane|glass_sphere|cube)\(\)$"#,
        |mut world, ctx| {
            let s = match ctx.matches[2].as_str() {
                "sphere" => default_sphere(),
                "plane" => default_plane(),
                "glass_sphere" => glass_sphere(),
                "cube" => default_cube(),
                _ => panic!("object kind not covered"),
            };
            world.shapes.insert(ctx.matches[1].clone(), Rc::new(s));
            world
        },
    );

    steps.given_regex(
        r#"^(s1|s2|shape|lower|upper|A|B|C|floor|ball) ← (sphere|plane|glass_sphere)\(\) with:$"#,
        |mut world, ctx| {
            let mut s = match ctx.matches[2].as_str() {
                "sphere" => default_sphere(),
                "plane" => default_plane(),
                "glass_sphere" => glass_sphere(),
                _ => panic!("object kind not covered"),
            };
            for row in &ctx.step.table.as_ref().unwrap().rows {
                let key = row.get(0).unwrap();
                let value = row.get(1).unwrap();
                match key.as_str() {
                    "material.color" => s.material.color = color_from_string(value),
                    "material.ambient" => s.material.ambient = value.parse::<f64>().unwrap(),
                    "material.diffuse" => s.material.diffuse = value.parse::<f64>().unwrap(),
                    "material.specular" => s.material.specular = value.parse::<f64>().unwrap(),
                    "material.reflective" => s.material.reflective = value.parse::<f64>().unwrap(),
                    "material.transparency" => {
                        s.material.transparency = value.parse::<f64>().unwrap()
                    }
                    "material.refractive_index" => {
                        s.material.refractive_index = value.parse::<f64>().unwrap()
                    }
                    "transform" => s.set_transform(transform_from_string(value)),
                    _ => panic!("object property not covered"),
                }
            }
            world.shapes.insert(ctx.matches[1].to_string(), Rc::new(s));
            world
        },
    );

    steps.given_regex(r#"^(shape|A|B) has:$"#, |mut world, ctx| {
        let mut s = world.shapes.get(&ctx.matches[1]).unwrap().deref().clone();
        for row in &ctx.step.table.as_ref().unwrap().rows {
            let key = row.get(0).unwrap();
            let value = row.get(1).unwrap();
            match (key.as_str(), value.as_str()) {
                ("material.ambient", value) => s.material.ambient = value.parse::<f64>().unwrap(),
                ("material.pattern", "test_pattern()") => s.material.pattern = Some(test_pattern()),
                ("material.transparency", value) => {
                    s.material.transparency = value.parse::<f64>().unwrap()
                }
                ("material.refractive_index", value) => {
                    s.material.refractive_index = value.parse::<f64>().unwrap()
                }
                _ => panic!("object property not covered"),
            }
        }
        world.shapes.insert(ctx.matches[1].to_string(), Rc::new(s));
        world
    });

    steps.when_regex(r#"^m ← s.material$"#, |mut world, _ctx| {
        world.m = world.shapes.get("s").unwrap().material.clone();
        world
    });

    steps.when("s.material ← m", |mut world, _ctx| {
        let mut obj = world.shapes.get_mut("s").unwrap().deref().deref().clone();
        obj.material = world.m.clone();
        world.shapes.insert("s".to_string(), Rc::new(obj));
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
        let transformation = world.get4x4(&ctx.matches[1]).clone();
        let mut obj = world.shapes.get_mut("s").unwrap().deref().deref().clone();
        obj.set_transform(transformation);
        world.shapes.insert("s".to_string(), Rc::new(obj));
        world
    });

    steps.when_regex(r#"^set_transform\(s, (t|m)\)$"#, |mut world, ctx| {
        let transformation = world.get4x4(&ctx.matches[1]).clone();
        let mut obj = world.shapes.get_mut("s").unwrap().deref().deref().clone();
        obj.set_transform(transformation);
        world.shapes.insert("s".to_string(), Rc::new(obj));
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
            let mut obj = world.shapes.get_mut(&ctx.matches[1]).unwrap().deref().deref().clone();
            obj.set_transform(transformation);
            world.shapes.insert(ctx.matches[1].to_string(), Rc::new(obj));
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
            let mut obj = world.shapes.get_mut("s").unwrap().deref().deref().clone();
            obj.set_transform(transformation);
            world.shapes.insert("s".to_string(), Rc::new(obj));
            world
        },
    );

    steps.then_regex(r#"^s.transform = (identity_matrix|t)$"#, |world, ctx| {
        let lookup = world.shapes.get("s").unwrap().transform();
        let desired = world.get4x4(ctx.matches[1].as_str());
        assert_eq!(lookup, desired);
        world
    });

    steps.when_regex(r#"^xs ← intersect\(s, r\)$"#, |mut world, _ctx| {
        let s = world.shapes.get("s").unwrap();
        world.xs = intersect(s, &world.r);
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
            let point = parse_point(&ctx.matches[2..=4]);
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
