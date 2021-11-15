use crate::MyWorld;
use cucumber::{given, then, when};
use lab_raytracing_rs::{groups::Group, transformations::{rotation_y, scaling}};
use std::{f64::consts::PI, ops::Deref};

#[given(regex = r"^(g|g1|g2) ← group\(\)$")]
async fn create_group(world: &mut MyWorld, name: String) {
    let grp = Group::default();
    match name.as_str() {
        "g" => world.g = grp,
        "g1" => world.g1 = grp,
        "g2" => world.g2 = grp,
        _ => panic!("group not covered"),
    }
}

#[given("g ← parser.default_group")]
async fn group_from_parser(world: &mut MyWorld) {
    world.g = world.parser.default_group.clone();
}

#[then(regex = r"^g.transform = (identity_matrix)$")]
async fn compare_group_transform(world: &mut MyWorld, name: String) {
    let matrix = world.get4x4(&name);
    assert_eq!(&world.g.transform(), &matrix);
}

#[then("g is empty")]
async fn group_is_empty(world: &mut MyWorld) {
    assert!(world.g.is_empty());
}

#[then("g is not empty")]
async fn group_is_not_empty(world: &mut MyWorld) {
    assert!(!world.g.is_empty());
}

#[given(regex = r"set_transform\((g|g2), scaling\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)\)")]
async fn scale_group(world: &mut MyWorld, group: String, x: f64, y: f64, z: f64) {
    let transform = scaling(x, y, z);
    match group.as_str() {
        "g" => world.g.set_transform(transform),
        "g2" => world.g2.set_transform(transform),
        _ => panic!("group not covered"),
    }
}

#[given(regex = r"set_transform\((g1), rotation_y\(π/2\)\)")]
async fn rotate_group(world: &mut MyWorld, group: String) {
    let transform = rotation_y(PI / 2.0);
    match group.as_str() {
        "g1" => world.g1.set_transform(transform),
        _ => panic!("group not covered"),
    }
}

#[given(regex = r"^add_child\((g1), (g2)\)$")]
async fn add_child_group(world: &mut MyWorld, group: String, child: String) {
    let g = match child.as_str() {
        "g2" => world.g2.clone(),
        _ => panic!("group not covered"),
    };
    match group.as_str() {
        "g1" => world.g1.add_group(g),
        _ => panic!("group not covered"),
    }
}

#[given(regex = r"^add_child\((g|g1|g2), (s|s1|s2|s3)\)$")]
#[when(regex = r"^add_child\((g|g1|g2), (s|s1|s2|s3)\)$")]
async fn add_child_object(world: &mut MyWorld, group: String, child: String) {
    let obj = world.objects.get(&child).unwrap().deref().clone();
    match group.as_str() {
        "g" => world.g.add_object(obj),
        "g1" => world.g1.add_object(obj),
        "g2" => world.g2.add_object(obj),
        _ => panic!("group not covered"),
    }
}

#[when(regex = r"^xs ← local_intersect\(g, r\)$")]
#[when(regex = r"^xs ← intersect\(g, r\)$")]
async fn intersect_group(world: &mut MyWorld) {
    world.xs = world.g.intersect(&world.r);
}

#[when(regex = r"^(t1|t2) ← (first|second) child of g$")]
async fn assign_object_from_group(world: &mut MyWorld, target: String, position: String) {
    let object = match position.as_str() {
        "first" => world.g.get_object(0),
        "second" => world.g.get_object(1),
        _ => panic!("position not covered"),
    };
    world.objects.insert(target, object);
}
