use cucumber_rust::Steps;
use lab_raytracing_rs::{lights::{lighting, Pointlight}, shapes::default_sphere};

use crate::MyWorld;

use super::tuples::parse_point;

pub fn steps() -> Steps<MyWorld<'static>> {
    let mut steps: Steps<MyWorld> = Steps::new();

    steps.when(
        "light ← point_light(position, intensity)",
        |mut world, _ctx| {
            let position = world.tuples.get("position").unwrap();
            let intensity = world.tuples.get("intensity").unwrap();
            world.light = Pointlight::new(position.clone(), intensity.clone());
            world
        },
    );

    steps.then("light.position = position", |world, _ctx| {
        let position = world.tuples.get("position").unwrap();
        assert_eq!(world.light.position, *position);
        world
    });

    steps.then("light.intensity = intensity", |world, _ctx| {
        let intensity = world.tuples.get("intensity").unwrap();
        assert_eq!(world.light.intensity, *intensity);
        world
    });

    steps.given_regex(
        r#"^(w\.)?light ← point_light\(point\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\), color\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)\)$"#,
        |mut world, ctx| {
            let position = parse_point(&ctx.matches[2..=4]);
            let intensity = parse_point(&ctx.matches[5..=7]);
            let light = Pointlight::new(position, intensity);
            match ctx.matches[1].as_str() {
                "w." => world.w.light = Some(light),
                _ => world.light = light,
            };
            world
        },
    );

    steps.when(
        "result ← lighting(m, light, position, eyev, normalv)",
        |mut world, _ctx| {
            let material = &world.m;
            let object = default_sphere();
            let light = &world.light;
            let position = world.tuples.get("position").unwrap();
            let eyev = world.tuples.get("eyev").unwrap();
            let normalv = world.tuples.get("normalv").unwrap();
            let result = lighting(material, &object, light, position, eyev, normalv, false);
            world.tuples.insert("result".to_string(), result);
            world
        },
    );

    steps.when(
        "result ← lighting(m, light, position, eyev, normalv, in_shadow)",
        |mut world, _ctx| {
            let material = &world.m;
            let object = default_sphere();
            let light = &world.light;
            let position = world.tuples.get("position").unwrap();
            let eyev = world.tuples.get("eyev").unwrap();
            let normalv = world.tuples.get("normalv").unwrap();
            let in_shadow = world.in_shadow;
            let result = lighting(material, &object, light, position, eyev, normalv, in_shadow);
            world.tuples.insert("result".to_string(), result);
            world
        },
    );

    steps.when_regex(
        r#"^(c1|c2) ← lighting\(m, light, point\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\), eyev, normalv, false\)$"#,
        |mut world, ctx| {
            let material = &world.m;
            let object = default_sphere();
            let light = &world.light;
            let position = parse_point(&ctx.matches[2..=4]);
            let eyev = world.tuples.get("eyev").unwrap();
            let normalv = world.tuples.get("normalv").unwrap();
            let in_shadow = false;
            let result = lighting(material, &object, light, &position, eyev, normalv, in_shadow);
            world.tuples.insert(ctx.matches[1].clone(), result);
            world
        },
    );

    steps
}
