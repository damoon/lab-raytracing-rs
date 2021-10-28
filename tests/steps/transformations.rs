use crate::steps::tuples::{parse_point, parse_vector};
use cucumber::{given, then, when};
use lab_raytracing_rs::matrices::Matrix4x4;
use lab_raytracing_rs::transformations::{
    rotation_x, rotation_y, rotation_z, scaling, shearing, translation, view_transform,
};
use std::f64::consts::PI;

use crate::MyWorld;

pub fn parse_translation(ss: &[String]) -> Matrix4x4 {
    let x = ss[0].parse::<f64>().unwrap();
    let y = ss[1].parse::<f64>().unwrap();
    let z = ss[2].parse::<f64>().unwrap();
    translation(x, y, z)
}

pub fn parse_scaling(ss: &[String]) -> Matrix4x4 {
    let x = ss[0].parse::<f64>().unwrap();
    let y = ss[1].parse::<f64>().unwrap();
    let z = ss[2].parse::<f64>().unwrap();
    scaling(x, y, z)
}

pub fn parse_shearing(ss: &[String]) -> Matrix4x4 {
    let xy = ss[0].parse::<f64>().unwrap();
    let xz = ss[1].parse::<f64>().unwrap();
    let yx = ss[2].parse::<f64>().unwrap();
    let yz = ss[3].parse::<f64>().unwrap();
    let zx = ss[4].parse::<f64>().unwrap();
    let zy = ss[5].parse::<f64>().unwrap();
    shearing(xy, xz, yx, yz, zx, zy)
}

#[given(
    regex = r"^(transform|B|C|m|t) ← (translation|scaling)\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"
)]
async fn create_transformation(
    world: &mut MyWorld,
    target: String,
    kind: String,
    x: String,
    y: String,
    z: String,
) {
    let transformation = match kind.as_str() {
        "translation" => parse_translation(&[x, y, z]),
        "scaling" => parse_scaling(&[x, y, z]),
        _ => panic!("transformation not covered"),
    };
    world.insert4x4(target, transformation);
}

#[given(regex = r"^(m) ← scaling\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\) \* rotation_z\(π/5\)$")]
async fn combine_transformations(
    world: &mut MyWorld,
    target: String,
    x: String,
    y: String,
    z: String,
) {
    let scaling = parse_scaling(&[x, y, z]);
    let rotation_z = rotation_z(PI / 5.0);
    let transformation = scaling * rotation_z;
    world.insert4x4(target, transformation);
}

#[then(
    regex = r"^(half_quarter|full_quarter|transform|inv) \* (p|v) = (point|vector)\(([-0-9.]+|-?√2/2), ([-0-9.]+|-?√2/2), ([-0-9.]+|-?√2/2)\)$"
)]
async fn transform_tuple_desired(
    world: &mut MyWorld,
    transformation: String,
    tuple: String,
    kind: String,
    x: String,
    y: String,
    z: String,
) {
    let desired = match kind.as_str() {
        "point" => parse_point(&[x, y, z]),
        "vector" => parse_vector(&[x, y, z]),
        _ => panic!("action not defined"),
    };
    let transformation = world.get4x4(&transformation);
    let tuple = world.tuples.get(&tuple).unwrap().clone();
    let calculated = transformation * tuple;
    assert_eq!(calculated, desired);
}

#[given(regex = r"^(inv) ← inverse\((transform|half_quarter)\)$")]
async fn inverse_transformations(world: &mut MyWorld, target: String, transformation: String) {
    let inverse = world.get4x4(&transformation).inverse().unwrap();
    world.insert4x4(target, inverse);
}

#[then(regex = r"^(transform) \* (v) = (v)$")]
async fn transform_tuple_compare(
    world: &mut MyWorld,
    transformation: String,
    tuple: String,
    desired: String,
) {
    let transformation = world.get4x4(&transformation);
    let tuple = world.tuples.get(&tuple).unwrap();
    let desired = world.tuples.get(&desired).unwrap();
    let calculated = transformation * tuple;
    assert_eq!(&calculated, desired);
}

#[given(regex = r"^(half_quarter|full_quarter|A) ← rotation_(x|y|z)\(π / ([-0-9.]+)\)$")]
async fn prepare_rotation(world: &mut MyWorld, transformation: String, axis: String, divisor: f64) {
    let rotation = match axis.as_str() {
        "x" => rotation_x(PI / divisor),
        "y" => rotation_y(PI / divisor),
        "z" => rotation_z(PI / divisor),
        _ => panic!("axis unknown"),
    };
    world.insert4x4(transformation, rotation);
}

#[given(
    regex = r"^transform ← shearing\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+), ([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"
)]
async fn prepare_shearing(
    world: &mut MyWorld,
    x1: String,
    y1: String,
    z1: String,
    x2: String,
    y2: String,
    z2: String,
) {
    let shearing = parse_shearing(&[x1, y1, z1, x2, y2, z2]);
    world.insert4x4("transform".to_string(), shearing);
}

#[when(regex = r"^(p2|p3|p4) ← (A|B|C) \* (p|p2|p3)$")]
async fn transform_tuple(
    world: &mut MyWorld,
    target: String,
    transformation: String,
    tuple: String,
) {
    let matrix = world.get4x4(&transformation);
    let tuple = world.tuples.get(&tuple).unwrap();
    let computed = matrix * tuple;
    world.tuples.insert(target, computed);
}

#[when(regex = r"^t ← view_transform\(from, to, up\)$")]
async fn create_view(world: &mut MyWorld) {
    let from = world.tuples.get("from").unwrap();
    let to = world.tuples.get("to").unwrap();
    let up = world.tuples.get("up").unwrap();
    let view_transformation = view_transform(from, to, up);
    world.insert4x4("t".to_string(), view_transformation);
}

#[then(
    regex = r"^(t|s.transform|pattern.transform) = (scaling|translation)\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"
)]
async fn compare_transformation(
    world: &mut MyWorld,
    target: String,
    kind: String,
    x: String,
    y: String,
    z: String,
) {
    let desired = match kind.as_str() {
        "scaling" => parse_scaling(&[x, y, z]),
        "translation" => parse_translation(&[x, y, z]),
        _ => panic!("desired function not covered"),
    };
    let lookup = match target.as_str() {
        "s.transform" => world.objects.get("s").unwrap().transform(),
        "pattern.transform" => world.pattern.transform(),
        a => world.get4x4(a),
    };
    assert_eq!(lookup, &desired);
}
