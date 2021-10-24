use super::tuples::{eq_tuples_similar, parse_color};
use crate::MyWorld;
use cucumber::{gherkin::Step, given, then, when};
use lab_raytracing_rs::{canvas::Canvas, tuples::color};

#[given(regex = r"^c ← canvas\(([0-9]+), ([0-9]+)\)$")]
async fn create_canvas(world: &mut MyWorld, w: usize, h: usize) {
    world.canvas = Canvas::new(w, h);
}

#[then(regex = r"^c.(width|height) = ([0-9]+)$")]
async fn compare_canvas(world: &mut MyWorld, attribute: String, desired: usize) {
    let value = match attribute.as_str() {
        "width" => world.canvas.width,
        "height" => world.canvas.height,
        _ => panic!("Invalid attribute checked"),
    };
    assert_eq!(value, desired);
}

#[then(regex = r"^every pixel of c is color\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$")]
async fn every_pixel_is_colored(world: &mut MyWorld, x: String, y: String, z: String) {
    let color = parse_color(&[x, y, z]);
    for w in 0..world.canvas.width {
        for h in 0..world.canvas.height {
            assert_eq!(&color, world.canvas.at(w, h));
        }
    }
}

#[when(regex = r"^every pixel of c is set to color\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$")]
async fn color_every_pixel(world: &mut MyWorld, x: String, y: String, z: String) {
    let color = parse_color(&[x, y, z]);
    world.canvas.fill(color);
}

#[when(regex = r"^write_pixel\(c, ([0-9]+), ([0-9]+), (\w+)\)$")]
async fn write_pixel(world: &mut MyWorld, w: usize, h: usize, color: String) {
    let color = world.tuples.get(&color).unwrap().clone();
    world.canvas.set(w, h, color);
}

#[then(regex = r"^pixel_at\(c, ([-0-9.]+), ([-0-9.]+)\) = (red)$")]
async fn compare_pixel(world: &mut MyWorld, w: usize, h: usize, desired: String) {
    let color = world.canvas.at(w, h);
    let desired = world.tuples.get(&desired).unwrap();
    eq_tuples_similar(color, desired);
}

#[then(
    regex = r"^pixel_at\(image, ([-0-9.]+), ([-0-9.]+)\) = color\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"
)]
async fn compare_pixel_color(
    world: &mut MyWorld,
    w: usize,
    h: usize,
    red: f64,
    green: f64,
    blue: f64,
) {
    let desired = color(red, green, blue);
    let color = world.image.at(w, h);
    eq_tuples_similar(color, &desired);
}

#[when("ppm ← canvas_to_ppm(c)")]
async fn canvas_to_ppm(world: &mut MyWorld) {
    let mut writer = std::io::BufWriter::new(Vec::new());
    world.canvas.ppm(&mut writer).expect("failed to write ppm");
    let bytes = writer.into_inner().expect("access written ppm buffer");
    world.ppm = String::from_utf8(bytes).expect("convert ppm bytes to ut8 string");
}

#[then(regex = r"^lines ([-0-9.]+)-([-0-9.]+) of ppm are$")]
async fn compare_ppm_lines(world: &mut MyWorld, beginning: usize, end: usize, step: &Step) {
    let mut desired_lines = step.docstring.as_ref().unwrap().lines();
    let mut ppm_lines = world.ppm.lines();
    desired_lines.next(); // skip first because of leading line break
    for _ in 0..beginning - 1 {
        ppm_lines.next();
    }
    for _ in 0..((end - 1) - beginning) {
        let desired_line = desired_lines.next();
        let ppm_line = ppm_lines.next();
        assert_eq!(desired_line, ppm_line);
    }
}

#[then("ppm ends with a newline character")]
async fn compare_ppm_end(world: &mut MyWorld) {
    assert_eq!('\n', world.ppm.chars().last().unwrap());
}
