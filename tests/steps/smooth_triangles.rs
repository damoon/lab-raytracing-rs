use std::sync::Arc;

use crate::MyWorld;
use cucumber::{then, when};
use lab_raytracing_rs::objects::{smooth_triangle, Shape};

use super::tuples::parse_point;

#[when("tri ← smooth_triangle(p1, p2, p3, n1, n2, n3)")]
async fn create_smooth_triangle(world: &mut MyWorld) {
    let p1 = world.tuples.get("p1").unwrap().clone();
    let p2 = world.tuples.get("p2").unwrap().clone();
    let p3 = world.tuples.get("p3").unwrap().clone();
    let n1 = world.tuples.get("n1").unwrap().clone();
    let n2 = world.tuples.get("n2").unwrap().clone();
    let n3 = world.tuples.get("n3").unwrap().clone();
    let tri = smooth_triangle(p1, p2, p3, n1, n2, n3);
    world.objects.insert("tri".to_string(), Arc::new(tri));
}

#[then(regex = r"tri.(p1|p2|p3|n1|n2|n3) = (p1|p2|p3|n1|n2|n3)")]
async fn compare_smooth_triangle_attributes(
    world: &mut MyWorld,
    attribute: String,
    desired: String,
) {
    let desired = world.tuples.get(&desired).unwrap().clone();
    if let Shape::SmoothTriangle(tri) = world
        .objects
        .get("tri")
        .unwrap()
        .clone()
        .as_ref()
        .shape
        .clone()
    {
        match attribute.as_str() {
            "p1" => assert_eq!(tri.p1, desired),
            "p2" => assert_eq!(tri.p2, desired),
            "p3" => assert_eq!(tri.p3, desired),
            "n1" => assert_eq!(tri.n1, desired),
            "n2" => assert_eq!(tri.n2, desired),
            "n3" => assert_eq!(tri.n3, desired),
            _ => panic!("attribute not covered"),
        }
    } else {
        panic!("shape not a smooth triangle")
    }
}

#[when(regex = r"^(n) ← normal_at\(tri, point\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\), i\)$")]
async fn calculate_normal_at(world: &mut MyWorld, name: String, x: String, y: String, z: String) {
    let tri = world.objects.get("tri").unwrap();
    let hit = world.intersections.get("i").unwrap();
    let point = parse_point(&[x, y, z]);
    let normal = tri.normal_at(&point, hit);
    world.tuples.insert(name, normal);
}
