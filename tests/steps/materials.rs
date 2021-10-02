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
        r#"^m.(ambient|diffuse|specular|shininess) = ([-0-9.]+)$"#,
        |world, ctx| {
            let desired = ctx.matches[2].parse::<f64>().unwrap();
            let value = match ctx.matches[1].as_str() {
                "ambient" => world.m.ambient,
                "diffuse" => world.m.diffuse,
                "specular" => world.m.specular,
                "shininess" => world.m.shininess,
                _ => panic!("material attribute not covered"),
            };
            assert_abs_diff_eq!(value, desired);
            world
        },
    );

    steps.given_regex(r#"^m.(ambient) ← ([-0-9.]+)$"#, |mut world, ctx| {
        let value = ctx.matches[2].parse::<f64>().unwrap();
        match ctx.matches[1].as_str() {
            "ambient" => world.m.ambient = value,
            _ => panic!("material attribute not covered"),
        };
        world
    });

    steps.given_regex(
        r#"^(outer|inner).material.ambient ← ([-0-9.]+)$"#,
        |mut world, ctx| {
            let value = ctx.matches[2].parse::<f64>().unwrap();
            match ctx.matches[1].as_str() {
                "outer" => {
                    // workaround for expected object pointer behaviour in cucumber tests
                    if let Some(idx) = world.w.objects.iter().position(|r| r == &world.outer) {
                        world.w.objects[idx].material.ambient = value;
                    }
                    world.outer.material.ambient = value
                }
                "inner" => {
                    // workaround for expected object pointer behaviour in cucumber tests
                    if let Some(idx) = world.w.objects.iter().position(|r| r == &world.inner) {
                        world.w.objects[idx].material.ambient = value;
                    }
                    world.inner.material.ambient = value
                }
                _ => panic!("material attribute not covered"),
            };
            world
        },
    );

    steps
}
