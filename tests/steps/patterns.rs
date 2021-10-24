use crate::{
    steps::{
        transformations::{parse_scaling, parse_translation},
        tuples::{parse_color, parse_point},
    },
    MyWorld,
};
use cucumber::{given, then, when};
use lab_raytracing_rs::{
    patterns::{
        checkers_pattern, gradient_pattern, pattern_at_shape, ring_pattern, solid_pattern,
        stripe_pattern, test_pattern, Renderer,
    },
    tuples::point,
};

#[given(
    regex = r"^pattern ← (stripe_pattern|gradient_pattern|ring_pattern|checkers_pattern)\(white, black\)$"
)]
async fn assign_pattern(world: &mut MyWorld, pattern: String) {
    let white = Box::new(solid_pattern(world.tuples.get("white").unwrap().clone()));
    let black = Box::new(solid_pattern(world.tuples.get("black").unwrap().clone()));
    world.pattern = match pattern.as_str() {
        "stripe_pattern" => stripe_pattern(white, black),
        "gradient_pattern" => gradient_pattern(white, black),
        "ring_pattern" => ring_pattern(white, black),
        "checkers_pattern" => checkers_pattern(white, black),
        _ => panic!("pattern not covered"),
    };
}

#[then(regex = r"^pattern.(a|b) = (white|black)$")]
async fn pattern_recursion(world: &mut MyWorld, attribute: String, color: String) {
    let desired = world.tuples.get(&color).unwrap();
    let pattern = match (&world.pattern.renderer, attribute.as_str()) {
        (Renderer::Stripes(a, _), "a") => a,
        (Renderer::Stripes(_, b), "b") => b,
        _ => panic!("pattern attribute not covered"),
    };
    let lookup = pattern.color_at(&point(0.0, 0.0, 0.0));
    assert_eq!(&lookup, desired);
}

#[then(
    regex = r"^(stripe_at|pattern_at)\(pattern, point\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)\) = (white|black)$"
)]
async fn color_at_pattern(
    world: &mut MyWorld,
    _attribute: String,
    x: String,
    y: String,
    z: String,
    color: String,
) {
    let color = world.tuples.get(&color).unwrap();
    let point = parse_point(&[x, y, z]);
    let computed = world.pattern.color_at(&point);
    assert_eq!(&computed, color);
}

#[allow(clippy::too_many_arguments)]
#[then(
    regex = r"^(stripe_at|pattern_at)\(pattern, point\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)\) = color\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"
)]
async fn color_at_pattern_is(
    world: &mut MyWorld,
    _attribute: String,
    point_x: String,
    point_y: String,
    point_z: String,
    color_r: String,
    color_g: String,
    color_b: String,
) {
    let point = parse_point(&[point_x, point_y, point_z]);
    let color = parse_color(&[color_r, color_g, color_b]);
    let computed = world.pattern.color_at(&point);
    assert_eq!(computed, color);
}

#[given(
    regex = r"^m.pattern ← stripe_pattern\(color\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\), color\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)\)$"
)]
async fn assign_stripe_pattern(
    world: &mut MyWorld,
    r1: String,
    g1: String,
    b1: String,
    r2: String,
    g2: String,
    b2: String,
) {
    let color_a = Box::new(solid_pattern(parse_color(&[r1, g1, b1])));
    let color_b = Box::new(solid_pattern(parse_color(&[r2, g2, b2])));
    let pattern = Box::new(stripe_pattern(color_a, color_b));
    world.m.pattern = Some(pattern);
}

#[when(
    regex = r"^c ← (stripe_at_object|pattern_at_shape)\(pattern, (object|shape), point\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)\)$"
)]
async fn assign_color_from_stripe_pattern(
    world: &mut MyWorld,
    _attribute: String,
    object_name: String,
    x: String,
    y: String,
    z: String,
) {
    let point = parse_point(&[x, y, z]);
    let object = world.objects.get(&object_name).unwrap();
    let color = pattern_at_shape(&world.pattern, object, &point);
    world.tuples.insert("c".to_string(), color);
}

#[given(
    regex = r"^set_pattern_transform\(pattern, (scaling|translation)\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)\)$"
)]
#[when(
    regex = r"^set_pattern_transform\(pattern, (scaling|translation)\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)\)$"
)]
async fn set_pattern_transform(
    world: &mut MyWorld,
    transformation: String,
    x: String,
    y: String,
    z: String,
) {
    let transformation = match transformation.as_str() {
        "scaling" => parse_scaling(&[x, y, z]),
        "translation" => parse_translation(&[x, y, z]),
        _ => panic!("transformation not covered"),
    };
    world.pattern.set_transform(transformation);
}

#[given("pattern ← test_pattern()")]
async fn assign_test_pattern(world: &mut MyWorld) {
    world.pattern = test_pattern();
}

#[then(regex = r"^pattern.transform = (identity_matrix)$")]
async fn compare_transform(world: &mut MyWorld, desired_matrix: String) {
    let desired = world.get4x4(&desired_matrix);
    let lookup = world.pattern.transform();
    assert_eq!(lookup, desired);
}
