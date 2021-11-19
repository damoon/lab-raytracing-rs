use std::sync::Arc;

use crate::{
    steps::tuples::{parse_point, parse_vector},
    MyWorld,
};
use cucumber::{given, then};
use lab_raytracing_rs::objects::default_testshape;

#[given(regex = r"^(s) ← test_shape\(\)$")]
async fn create_test_shape(world: &mut MyWorld, shape: String) {
    world.objects.insert(shape, Arc::new(default_testshape()));
}

#[then(
    regex = r"^s.saved_ray.(origin|direction) = (point|vector)\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"
)]
async fn compare_ray_properties(
    _world: &mut MyWorld,
    property: String,
    kind: String,
    x: String,
    y: String,
    z: String,
) {
    let desired = match kind.as_str() {
        "point" => parse_point(&[x, y, z]),
        "vector" => parse_vector(&[x, y, z]),
        _ => panic!("desired kind not covered"),
    };
    let ray = lab_raytracing_rs::objects::SAVED_RAY.with(|c| c.read().unwrap().clone());
    let lookup = match property.as_str() {
        "origin" => ray.origin.clone(),
        "direction" => ray.direction.clone(),
        _ => panic!("lookup attribute not covered"),
    };
    assert_eq!(lookup, desired);
}

#[then(regex = r"^(t1|t2) = (t1|t2)$")]
async fn compare_objects(world: &mut MyWorld, name: String, desired: String) {
    let tuple = world.objects.get(&name).unwrap().as_ref().clone();
    let desired = world.objects.get(&desired).unwrap().as_ref().clone();
    assert_eq!(tuple, desired);
}
