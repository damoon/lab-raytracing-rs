use super::tuples::parse_color;
use crate::MyWorld;
use cucumber_rust::Steps;
use lab_raytracing_rs::canvas::Canvas;

pub fn steps() -> Steps<MyWorld> {
    let mut steps: Steps<MyWorld> = Steps::new();

    steps.given_regex(
        r#"^c ← canvas\(([0-9]+), ([0-9]+)\)$"#,
        |mut world, ctx| {
            let w = ctx.matches[1].parse::<usize>().unwrap();
            let h = ctx.matches[2].parse::<usize>().unwrap();
            world.canvas = Canvas::new(w, h);
            world
        },
    );

    steps.then_regex(r#"^c.(width|height) = ([0-9]+)$"#, |world, ctx| {
        let desired = ctx.matches[2].parse::<usize>().unwrap();
        let value = match ctx.matches[1].as_str() {
            "width" => world.canvas.width,
            "height" => world.canvas.height,
            _ => panic!("Invalid attribute checked"),
        };
        assert_eq!(value, desired);

        world
    });

    steps.then_regex(
        r#"^every pixel of c is color\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |world, ctx| {
            let color = parse_color(&ctx.matches[1..=3]);
            for w in 0..world.canvas.width {
                for h in 0..world.canvas.height {
                    assert_eq!(color, world.canvas.at(w, h));
                }
            }
            world
        },
    );

    steps.when_regex(
        r#"^write_pixel\(c, ([0-9]+), ([0-9]+), (\w+)\)$"#,
        |mut world, ctx| {
            let color = world.tuples.get(&ctx.matches[3]).unwrap();
            let w = ctx.matches[1].parse::<usize>().unwrap();
            let h = ctx.matches[2].parse::<usize>().unwrap();
            world.canvas.set(w, h, *color);
            world
        },
    );

    steps.then_regex(
        r#"^pixel_at\(c, ([-0-9.]+), ([-0-9.]+)\) = (red)$"#,
        |world, ctx| {
            let w = ctx.matches[1].parse::<usize>().unwrap();
            let h = ctx.matches[2].parse::<usize>().unwrap();
            let color = world.canvas.at(w, h);
            let desired = world.tuples.get(&ctx.matches[3]).unwrap();
            assert_eq!(&color, desired);
            world
        },
    );

    steps.then_regex(
        r#"^pixel_at\(image, ([-0-9.]+), ([-0-9.]+)\) = color\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |world, ctx| {
            let w = ctx.matches[1].parse::<usize>().unwrap();
            let h = ctx.matches[2].parse::<usize>().unwrap();
            let desired = parse_color(&ctx.matches[3..=5]);
            let color = world.image.at(w, h);
            assert_eq!(color, desired);
            world
        },
    );

    steps.when_regex(r#"^ppm ← canvas_to_ppm\(c\)$"#, |mut world, _ctx| {
        let mut writer = std::io::BufWriter::new(Vec::new());
        world.canvas.ppm(&mut writer).expect("failed to write ppm");
        let bytes = writer.into_inner().expect("access written ppm buffer");
        world.ppm = String::from_utf8(bytes).expect("convert ppm bytes to ut8 string");
        world
    });

    steps.then_regex(
        r#"^lines ([-0-9.]+)-([-0-9.]+) of ppm are$"#,
        |world, ctx| {
            let beginning = ctx.matches[1].parse::<usize>().unwrap() - 1;
            let end = ctx.matches[2].parse::<usize>().unwrap() - 1;
            let mut desired_lines = ctx.step.docstring.as_ref().unwrap().lines();
            let mut ppm_lines = world.ppm.lines();
            desired_lines.next(); // skip first because of leading line break
            for _ in 0..beginning {
                ppm_lines.next();
            }
            for _ in 0..(end - beginning) {
                let desired_line = desired_lines.next();
                let ppm_line = ppm_lines.next();
                assert_eq!(desired_line, ppm_line);
            }
            world
        },
    );

    steps.then_regex(r#"^ppm ends with a newline character$"#, |world, _ctx| {
        assert_eq!('\n', world.ppm.chars().last().unwrap());
        world
    });

    steps.when_regex(
        r#"^every pixel of c is set to color\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |mut world, ctx| {
            let color = parse_color(&ctx.matches[1..=3]);
            world.canvas.fill(color);
            world
        },
    );

    steps
}
