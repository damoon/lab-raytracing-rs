use super::tuples::{parse_point, parse_vector};
use crate::steps::tuples::eq_tuples_similar;
use crate::MyWorld;
use cucumber::{given, then, when};
use lab_raytracing_rs::rays::Ray;
use lab_raytracing_rs::tuples::point;

#[given(regex = r"^r ← ray\(point\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\), direction\)$")]
async fn create_ray_from(world: &mut MyWorld, x: f64, y: f64, z: f64) {
    let origin = point(x, y, z);
    let direction = world.tuples.get("direction").unwrap().clone();
    world.r = Ray::new(origin, direction);
}

#[when("r ← ray(origin, direction)")]
async fn create_ray(world: &mut MyWorld) {
    let origin = world.tuples.get("origin").unwrap().clone();
    let direction = world.tuples.get("direction").unwrap().clone();
    world.r = Ray::new(origin, direction);
}

#[given(
    regex = r"^r ← ray\(point\(([-0-9.]+), ([-0-9.]+), (√2/2|[-0-9.]+)\), vector\(([-0-9.]+), (-√2/2|[-0-9.]+), (√2/2|[-0-9.]+)\)\)$"
)]
#[when(
    regex = r"^r ← ray\(point\(([-0-9.]+), ([-0-9.]+), (√2/2|[-0-9.]+)\), vector\(([-0-9.]+), (-√2/2|[-0-9.]+), (√2/2|[-0-9.]+)\)\)$"
)]
async fn create_ray_from_to(
    world: &mut MyWorld,
    x1: String,
    y1: String,
    z1: String,
    x2: String,
    y2: String,
    z2: String,
) {
    let origin = parse_point(&[x1, y1, z1]);
    let direction = parse_vector(&[x2, y2, z2]);
    world.r = Ray::new(origin, direction);
}

#[then(regex = r"^position\(r, ([-0-9.]+)\) = point\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$")]
async fn compare_position(world: &mut MyWorld, t: f64, x: f64, y: f64, z: f64) {
    let calculated = world.r.position(t);
    let desired = point(x, y, z);
    assert_eq!(desired, calculated);
}

#[then(regex = r"^(r|r2).(origin|direction) = (origin|direction)$")]
async fn compare_ray(world: &mut MyWorld, ray: String, attribute: String, desired: String) {
    let desired = world.tuples.get(&desired).unwrap();
    let ray = match ray.as_str() {
        "r" => &world.r,
        "r2" => &world.r2,
        _ => panic!("ray not covered",),
    };
    match attribute.as_str() {
        "origin" => assert_eq!(&ray.origin, desired),
        "direction" => assert_eq!(&ray.direction, desired),
        _ => panic!("attribute not covered",),
    };
}

#[then(
    regex = r"^(r|r2).(origin|direction) = (point|vector)\((-?√2/2|[-0-9.]+), (-?√2/2|[-0-9.]+), (-?√2/2|[-0-9.]+)\)$"
)]
async fn compare_ray_with_tuple(
    world: &mut MyWorld,
    ray: String,
    attribute: String,
    kind: String,
    x: String,
    y: String,
    z: String,
) {
    let ray = match ray.as_str() {
        "r" => &world.r,
        "r2" => &world.r2,
        _ => panic!("ray not covered",),
    };
    let desired = match kind.as_str() {
        "point" => parse_point(&[x, y, z]),
        "vector" => parse_point(&[x, y, z]),
        _ => panic!("kind not covered",),
    };
    match attribute.as_str() {
        "origin" => eq_tuples_similar(&ray.origin, &desired),
        "direction" => eq_tuples_similar(&ray.direction, &desired),
        _ => panic!("attribute not covered",),
    };
}

#[when("r2 ← transform(r, m)")]
async fn transform_ray(world: &mut MyWorld) {
    let transformation = world.get4x4("m");
    world.r2 = world.r.transform(transformation);
}
