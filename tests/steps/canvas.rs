use cucumber_rust::Steps;
use lab_raytracing_rs::canvas::{Canvas};
use lab_raytracing_rs::colors::{Color};

use crate::MyWorld;

pub fn steps() -> Steps<MyWorld> {
    let mut steps: Steps<MyWorld> = Steps::new();

    steps.given_regex(
        r#"(\w+) ← canvas\(([0-9]+), ([0-9]+)\)$"#,
        |mut world, ctx| {
            let name = ctx.matches[1].clone();
            let w = ctx.matches[2].parse::<usize>().unwrap();
            let h = ctx.matches[3].parse::<usize>().unwrap();
            let canvas = Canvas::new(w, h);
            world.canvases.insert(name, canvas);
            world
        },
    );

    steps.when_regex(
        r#"write_pixel\((\w+), ([0-9]+), ([0-9]+), (\w+)\)$"#,
        |mut world, ctx| {
            let canvas_name = ctx.matches[1].clone();
            let color_name = ctx.matches[4].clone();
            let canvas = world.canvases.get_mut(&canvas_name).unwrap();
            let color = world.colors.get(&color_name).unwrap();
            let w = ctx.matches[2].parse::<usize>().unwrap();
            let h = ctx.matches[3].parse::<usize>().unwrap();
            canvas.set(w, h, color.clone());
            world
        },
    );

    steps.then_regex(r#"^(\w+).(width|height) = ([0-9]+)$"#, |world, ctx| {
        let name = ctx.matches[1].clone();
        let attr = ctx.matches[2].clone();
        let desired = ctx.matches[3].parse::<usize>().unwrap();
        let canvas = world.canvases.get(&name).unwrap();
        let value = match attr.as_str() {
            "width" => canvas.width,
            "height" => canvas.height,
            _ => panic!("Invalid attribute checked"),
        };
        assert_eq!(value, desired);

        world
    });

    steps.then_regex(r#"^every pixel of (\w+) is color\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#, |world, ctx| {
        let name = ctx.matches[1].clone();
        let r = ctx.matches[2].parse::<f32>().unwrap();
        let g = ctx.matches[3].parse::<f32>().unwrap();
        let b = ctx.matches[4].parse::<f32>().unwrap();
        let color = Color { r, g, b };
        let canvas = world.canvases.get(&name).unwrap();
        
        for w in 0..canvas.width {
            for h in 0..canvas.height {
                assert_eq!(color, canvas.at(w, h));
            }
        }       

        world
    });

    steps.then_regex(r#"^pixel_at\((\w+), ([-0-9.]+), ([-0-9.]+)\) = (\w+)$"#, |world, ctx| {
        let canvas_name = ctx.matches[1].clone();
        let color_name = ctx.matches[4].clone();
        let canvas = world.canvases.get(&canvas_name).unwrap();
        let desired = world.colors.get(&color_name).unwrap();
        let w = ctx.matches[2].parse::<usize>().unwrap();
        let h = ctx.matches[3].parse::<usize>().unwrap();
        let color = canvas.at(w, h).clone();
        
        assert_eq!(&color, desired);

        world
    });

    steps
}
