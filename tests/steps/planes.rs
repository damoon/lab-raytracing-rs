use crate::{steps::tuples::parse_point, MyWorld};
use cucumber::{given, then, when};
use lab_raytracing_rs::{intersections::Intersection, objects::default_plane};
use std::sync::Arc;

#[given("p ← plane()")]
async fn assign_default_plane(world: &mut MyWorld) {
    let p = default_plane();
    world.objects.insert("p".to_string(), Arc::new(p));
}

#[when(regex = r"^xs ← local_intersect\((p|c|cyl|shape|t|tri), r\)$")]
async fn local_intersect(world: &mut MyWorld, shape_name: String) {
    let obj = world.objects.get(&shape_name).unwrap();
    world.xs = obj.intersect(&world.r, obj);
}

#[then("xs is empty")]
async fn intersections_are_empty(world: &mut MyWorld) {
    assert_eq!(world.xs.len(), 0);
}

#[then(regex = r"^xs\[([-0-9.]+)\].object = (p|s2|s1)$")]
async fn intersecting_object(world: &mut MyWorld, index: usize, object_name: String) {
    let desired = world.objects.get(&object_name).unwrap();
    let lookup = &world.xs.get(index).unwrap().object;
    assert_eq!(lookup, desired);
}

#[when(
    regex = r"^(n|n1|n2|n3) ← local_normal_at\((p|cyl|shape|t), point\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)\)$"
)]
async fn local_normal_at_point(
    world: &mut MyWorld,
    normal_name: String,
    object_name: String,
    x: String,
    y: String,
    z: String,
) {
    let point = parse_point(&[x, y, z]);
    let obj = world.objects.get(&object_name).unwrap();
    let hit = &Intersection {
        t: 0.0,
        object: obj.clone(),
        u: 0.0,
        v: 0.0,
    };
    let normal = obj.shape.normal_at(&point, hit);
    world.tuples.insert(normal_name, normal);
}

#[when(regex = r"^(n|n1|n2|n3|normal) ← local_normal_at\((c), (p)\)$")]
async fn local_normal_at(
    world: &mut MyWorld,
    normal_name: String,
    object_name: String,
    point_name: String,
) {
    let point = world.tuples.get(&point_name).unwrap();
    let obj = world.objects.get(&object_name).unwrap();
    let hit = &Intersection {
        t: 0.0,
        object: obj.clone(),
        u: 0.0,
        v: 0.0,
    };
    let normal = obj.shape.normal_at(point, hit);
    world.tuples.insert(normal_name, normal);
}
