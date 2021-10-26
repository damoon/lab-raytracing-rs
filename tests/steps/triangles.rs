use crate::{
    steps::tuples::{parse_point, parse_vector},
    MyWorld,
};
use cucumber::{given, then};
use lab_raytracing_rs::objects::{triangle, Shape};
use std::sync::Arc;

#[given(regex = r"^(t) ← triangle\((p1), (p2), (p3)\)$")]
async fn assign_triangle(world: &mut MyWorld, target: String, p1: String, p2: String, p3: String) {
    let p1 = world.tuples.get(&p1).unwrap().clone();
    let p2 = world.tuples.get(&p2).unwrap().clone();
    let p3 = world.tuples.get(&p3).unwrap().clone();
    let t = triangle(p1, p2, p3);
    world.objects.insert(target, Arc::new(t));
}

#[allow(clippy::too_many_arguments)]
#[given(
    regex = r"^(t) ← triangle\(point\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\), point\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\), point\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)\)$"
)]
async fn assign_created_triangle(
    world: &mut MyWorld,
    target: String,
    p1x: String,
    p1y: String,
    p1z: String,
    p2x: String,
    p2y: String,
    p2z: String,
    p3x: String,
    p3y: String,
    p3z: String,
) {
    let p1 = parse_point(&[p1x, p1y, p1z]);
    let p2 = parse_point(&[p2x, p2y, p2z]);
    let p3 = parse_point(&[p3x, p3y, p3z]);
    let t = triangle(p1, p2, p3);
    world.objects.insert(target, Arc::new(t));
}

#[then(regex = r"^t.(p1|p2|p3) = (p1|p2|p3)$")]
async fn compare_triangle_point(world: &mut MyWorld, point: String, desired: String) {
    let desired = world.tuples.get(&desired).unwrap().clone();
    match &world.objects.get("t").unwrap().as_ref().shape {
        Shape::Triangle(t) => match point.as_str() {
            "p1" => assert_eq!(t.p1, desired),
            "p2" => assert_eq!(t.p2, desired),
            "p3" => assert_eq!(t.p3, desired),
            _ => panic!("point property not covered"),
        },
        _ => panic!("shape not covered"),
    }
}

#[then(regex = r"^t.(e1|e2|normal) = vector\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$")]
async fn compare_triangle_attributes(
    world: &mut MyWorld,
    attribute: String,
    x: String,
    y: String,
    z: String,
) {
    let tuple = parse_vector(&[x, y, z]);
    match &world.objects.get("t").unwrap().as_ref().shape {
        Shape::Triangle(t) => match attribute.as_str() {
            "e1" => assert_eq!(t.e1, tuple),
            "e2" => assert_eq!(t.e2, tuple),
            "normal" => assert_eq!(t.normal, tuple),
            _ => panic!("point property not covered"),
        },
        _ => panic!("shape not covered"),
    }
}

#[then(regex = r"^(n1|n2|n3) = t.normal$")]
async fn compare_triangle_normal(world: &mut MyWorld, desired: String) {
    let desired = world.tuples.get(&desired).unwrap();
    match &world.objects.get("t").unwrap().as_ref().shape {
        Shape::Triangle(t) => assert_eq!(desired, &t.normal),
        _ => panic!("shape not covered"),
    }
}
