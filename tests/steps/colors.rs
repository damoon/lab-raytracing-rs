use approx::assert_abs_diff_eq;
use cucumber_rust::Steps;
use lab_raytracing_rs::colors::Color;

use crate::MyWorld;

pub fn steps() -> Steps<MyWorld> {
    let mut steps: Steps<MyWorld> = Steps::new();

    steps.given_regex(
        r#"([a-z0-9]+) ← color\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |mut world, ctx| {
            let name = ctx.matches[1].clone();
            let r = ctx.matches[2].parse::<f32>().unwrap();
            let g = ctx.matches[3].parse::<f32>().unwrap();
            let b = ctx.matches[4].parse::<f32>().unwrap();
            let c = Color { r, g, b };
            world.colors.insert(name, c);

            world
        },
    );

    steps.then_regex(
        r#"^([a-z]+).(red|green|blue) = ([-0-9.]+)$"#,
        |world, ctx| {
            let name = ctx.matches[1].clone();
            let attr = ctx.matches[2].clone();
            let desired = ctx.matches[3].parse::<f32>().unwrap();
            let color = world.colors.get(&name).unwrap();
            let value = match attr.as_str() {
                "red" => color.r,
                "green" => color.g,
                "blue" => color.b,
                _ => panic!("Invalid attribute checked"),
            };

            assert_abs_diff_eq!(value, desired);

            world
        },
    );

    steps.then_regex(
        r#"^([a-z0-9]+) \+ ([a-z0-9]+) = color\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |world, ctx| {
            let name1 = ctx.matches[1].clone();
            let name2 = ctx.matches[2].clone();
            let color1 = world.colors.get(&name1).unwrap();
            let color2 = world.colors.get(&name2).unwrap();
            let new_color = color1 + color2;
            let r = ctx.matches[3].parse::<f32>().unwrap();
            let g = ctx.matches[4].parse::<f32>().unwrap();
            let b = ctx.matches[5].parse::<f32>().unwrap();
            let desired = Color { r, g, b };
            assert!(new_color.approximately(desired));

            world
        },
    );

    steps.then_regex(
        r#"^([a-z0-9]+) \- ([a-z0-9]+) = color\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |world, ctx| {
            let name1 = ctx.matches[1].clone();
            let name2 = ctx.matches[2].clone();
            let color1 = world.colors.get(&name1).unwrap();
            let color2 = world.colors.get(&name2).unwrap();
            let new_color = color1 - color2;
            let r = ctx.matches[3].parse::<f32>().unwrap();
            let g = ctx.matches[4].parse::<f32>().unwrap();
            let b = ctx.matches[5].parse::<f32>().unwrap();
            let desired = Color { r, g, b };
            assert!(new_color.approximately(desired));

            world
        },
    );

    steps.then_regex(
        r#"^([a-z0-9]+) \* ([-0-9.]+) = color\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |world, ctx| {
            let name = ctx.matches[1].clone();
            let factor = ctx.matches[2].parse::<f32>().unwrap();
            let color = world.colors.get(&name).unwrap();
            let new_color = color * factor;
            let r = ctx.matches[3].parse::<f32>().unwrap();
            let g = ctx.matches[4].parse::<f32>().unwrap();
            let b = ctx.matches[5].parse::<f32>().unwrap();
            let desired = Color { r, g, b };
            assert!(new_color.approximately(desired));

            world
        },
    );

    steps.then_regex(
        r#"^([a-z0-9]+) \* ([a-z0-9]+) = color\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |world, ctx| {
            let name1 = ctx.matches[1].clone();
            let name2 = ctx.matches[2].clone();
            let color1 = world.colors.get(&name1).unwrap();
            let color2 = world.colors.get(&name2).unwrap();
            let new_color = color1 * color2;
            let r = ctx.matches[3].parse::<f32>().unwrap();
            let g = ctx.matches[4].parse::<f32>().unwrap();
            let b = ctx.matches[5].parse::<f32>().unwrap();
            let desired = Color { r, g, b };
            assert!(new_color.approximately(desired));

            world
        },
    );

    steps
}
