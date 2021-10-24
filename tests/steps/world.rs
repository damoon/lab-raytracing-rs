use crate::MyWorld;
use cucumber::{given, then, when};
use lab_raytracing_rs::{
    camera::RAY_RECURSION_DEPTH,
    groups::GroupMember,
    intersections::{color_at, reflected_color, refracted_color, shade_hit},
    lights::Pointlight,
    objects::default_sphere,
    transformations::scaling,
    tuples::{color, point},
    world::World,
};
use std::sync::Arc;

#[given("w ← world()")]
async fn empty_world(world: &mut MyWorld) {
    world.w = World::default();
}

#[given("w ← default_world()")]
#[when("w ← default_world()")]
async fn create_default_world(world: &mut MyWorld) {
    world.w = default_world();
}

#[then("w contains no objects")]
async fn world_is_empty(world: &mut MyWorld) {
    assert_eq!(world.w.objects.len(), 0);
}

#[then("w has no light source")]
async fn world_is_dark(world: &mut MyWorld) {
    assert!(world.w.light.is_none());
}

#[then("w.light = light")]
async fn compare_world_light(world: &mut MyWorld) {
    assert_eq!(world.w.light.as_ref().unwrap(), &world.light);
}

#[then(regex = r"^w contains (s1|s2)$")]
async fn world_contains(world: &mut MyWorld, shape: String) {
    let object = world.objects.get(&shape).unwrap().as_ref().clone();
    assert!(world.w.objects.iter().any(|i| {
        match i {
            GroupMember::Object(o) => object == o.as_ref().clone(),
            GroupMember::SubGroup(_) => panic!("matching groups is not supported"),
        }
    }));
}

#[when("xs ← intersect_world(w, r)")]
async fn intersect_world(world: &mut MyWorld) {
    world.xs = world.w.insersect(&world.r);
}

#[given(regex = r"^(shape|outer|inner|A|B) ← the (first|second) object in w$")]
async fn extract_from_world(world: &mut MyWorld, shape_name: String, position: String) {
    let index = match position.as_str() {
        "first" => 0,
        "second" => 1,
        _ => panic!("position not covered"),
    };
    let shape = world.w.objects.get(index).unwrap();
    match shape {
        GroupMember::Object(o) => world
            .objects
            .insert(shape_name, Arc::new(o.as_ref().clone())),
        GroupMember::SubGroup(_) => panic!("only objects are supported"),
    };
}

#[given(regex = r"^(A|B|outer|inner) is the (first|second) object in w$")]
async fn replace_in_world(world: &mut MyWorld, shape: String, position: String) {
    let index = match position.as_str() {
        "first" => 0,
        "second" => 1,
        _ => panic!("position not covered"),
    };
    let object = world.objects.get(&shape).unwrap();
    world.w.objects[index] = GroupMember::Object(Arc::new(object.as_ref().clone()));
}

#[when(regex = r"^(c|color) ← shade_hit\(w, comps\)$")]
async fn compute_shade_hit(world: &mut MyWorld, color: String) {
    let shaded_color = shade_hit(&world.w, &world.comps, RAY_RECURSION_DEPTH);
    world.tuples.insert(color, shaded_color);
}

#[when("color ← reflected_color(w, comps)")]
async fn compute_reflected_color(world: &mut MyWorld) {
    let color = reflected_color(&world.w, &world.comps, RAY_RECURSION_DEPTH);
    world.tuples.insert("color".to_string(), color);
}

#[when(regex = r"^color ← shade_hit\(w, comps, ([0-9]+)\)$")]
async fn compute_shade_hit_with_depth(world: &mut MyWorld, remaining: usize) {
    let color = shade_hit(&world.w, &world.comps, remaining);
    world.tuples.insert("color".to_string(), color);
}

#[when("color ← reflected_color(w, comps, 0)")]
async fn compute_reflected_color_end(world: &mut MyWorld) {
    let color = reflected_color(&world.w, &world.comps, 0);
    world.tuples.insert("color".to_string(), color);
}

#[when(regex = r"^c ← refracted_color\(w, comps, ([0-9]+)\)$")]
async fn compute_refracted_color(world: &mut MyWorld, remaining: usize) {
    let color = refracted_color(&world.w, &world.comps, remaining);
    world.tuples.insert("c".to_string(), color);
}

#[when("c ← color_at(w, r)")]
async fn compute_color_at(world: &mut MyWorld) {
    let color = color_at(&world.w, &world.r, RAY_RECURSION_DEPTH, None);
    world.tuples.insert("c".to_string(), color);
}

#[then("color_at(w, r) should terminate successfully")]
async fn color_at_terminates(world: &mut MyWorld) {
    let color = color_at(&world.w, &world.r, RAY_RECURSION_DEPTH, None);
    world.tuples.insert("dummy".to_string(), color); // insert here to avoid removal by compiler
}

#[then("c = inner.material.color")]
async fn compare_color(world: &mut MyWorld) {
    let c = world.tuples.get("c").unwrap();
    assert_eq!(c, &world.objects.get("inner").unwrap().material.color);
}

#[given("in_shadow ← true")]
async fn assign_shadowed(world: &mut MyWorld) {
    world.in_shadow = true;
}

#[then(regex = r"^is_shadowed\(w, p\) is (true|false)$")]
async fn is_shadowed(world: &mut MyWorld, value: String) {
    let desired = value.parse().unwrap();
    let point = world.tuples.get("p").unwrap();
    let computed = world.w.is_shadowed(point.clone(), None);
    assert_eq!(computed, desired);
}

#[given(regex = r"^(s1|s2|shape|lower|upper|floor|ball) is added to w$")]
async fn add_to_world(world: &mut MyWorld, shape: String) {
    let shape = world.objects.get(&shape).unwrap();
    world.w.add_object(shape.as_ref().clone());
}

pub fn default_world() -> World {
    let mut w = World::default();
    w.light = Some(Pointlight::new(
        point(-10.0, 10.0, -10.0),
        color(1.0, 1.0, 1.0),
    ));

    let mut s1 = default_sphere();
    s1.material.color = color(0.8, 1.0, 0.6);
    s1.material.diffuse = 0.7;
    s1.material.specular = 0.2;

    let mut s2 = default_sphere();
    s2.set_transform(scaling(0.5, 0.5, 0.5));

    w.add_object(s1);
    w.add_object(s2);

    w
}
