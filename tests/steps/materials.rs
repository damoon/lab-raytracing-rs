use std::{ops::Deref, sync::Arc};

use approx::assert_abs_diff_eq;
use cucumber_rust::Steps;
use lab_raytracing_rs::materials::Material;

use crate::{steps::tuples::parse_color, MyWorld};

pub fn steps() -> Steps<MyWorld> {
    let mut steps: Steps<MyWorld> = Steps::new();

    steps.given("m ← material()", |mut world, _ctx| {
        world.m = Material::default();
        world
    });

    steps.then("m = material()", |world, _ctx| {
        assert_eq!(world.m, Material::default());
        world
    });

    steps.then_regex(
        r#"^m.color = color\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |world, ctx| {
            let color = parse_color(&ctx.matches[1..=3]);
            assert_eq!(world.m.color, color);
            world
        },
    );

    steps.then_regex(
        r#"^(m|s.material).(ambient|diffuse|specular|shininess|reflective|transparency|refractive_index) = ([-0-9.]+)$"#,
        |world, ctx| {
            let desired = ctx.matches[3].parse::<f64>().unwrap();
            let material = match ctx.matches[1].as_str() {
                "m" => world.m.clone(),
                "s.material" => world.shapes.get("s").unwrap().material.clone(),
                _ => panic!("material origin not covered"),
            };
            let value = match ctx.matches[2].as_str() {
                "ambient" => material.ambient,
                "diffuse" => material.diffuse,
                "specular" => material.specular,
                "shininess" => material.shininess,
                "reflective" => material.reflective,
                "transparency" => material.transparency,
                "refractive_index" => material.refractive_index,
                _ => panic!("material attribute not covered"),
            };
            assert_abs_diff_eq!(value, desired);
            world
        },
    );

    steps.given_regex(
        r#"^m.(ambient|diffuse|specular) ← ([-0-9.]+)$"#,
        |mut world, ctx| {
            let value = ctx.matches[2].parse::<f64>().unwrap();
            match ctx.matches[1].as_str() {
                "ambient" => world.m.ambient = value,
                "diffuse" => world.m.diffuse = value,
                "specular" => world.m.specular = value,
                _ => panic!("material attribute not covered"),
            };
            world
        },
    );

    steps.given_regex(
        r#"^(outer|inner|shape).material.ambient ← ([-0-9.]+)$"#,
        |mut world, ctx| {
            let value = ctx.matches[2].parse::<f64>().unwrap();
            let mut object = world.shapes.get(&ctx.matches[1]).unwrap().deref().clone();
            object.material.ambient = value;
            world
                .shapes
                .insert(ctx.matches[1].clone(), Arc::new(object));
            world
        },
    );

    steps
}
