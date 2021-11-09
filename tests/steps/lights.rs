use super::tuples::parse_point;
use crate::MyWorld;
use cucumber::{given, then, when};
use lab_raytracing_rs::{
    lights::{lighting, Pointlight},
    objects::default_sphere,
};
use std::sync::Arc;

#[when("light ← point_light(position, intensity)")]
async fn assign_light(world: &mut MyWorld) {
    let position = world.tuples.get("position").unwrap();
    let intensity = world.tuples.get("intensity").unwrap();
    world.light = Pointlight::new(position.clone(), intensity.clone());
}

#[then("light.position = position")]
async fn compare_light_position(world: &mut MyWorld) {
    let position = world.tuples.get("position").unwrap();
    assert_eq!(world.light.position, *position);
}

#[then("light.intensity = intensity")]
async fn compare_light_intensity(world: &mut MyWorld) {
    let intensity = world.tuples.get("intensity").unwrap();
    assert_eq!(world.light.intensity, *intensity);
}

#[allow(clippy::too_many_arguments)]
#[allow(clippy::many_single_char_names)]
#[given(
    regex = r"^(w\.)?light ← point_light\(point\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\), color\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)\)$"
)]
async fn assign_light_attribute(
    world: &mut MyWorld,
    target: String,
    x: String,
    y: String,
    z: String,
    r: String,
    g: String,
    b: String,
) {
    let position = parse_point(&[x, y, z]);
    let intensity = parse_point(&[r, g, b]);
    let light = Pointlight::new(position, intensity);
    match target.as_str() {
        "w." => world.w.light = Some(light),
        _ => world.light = light,
    };
}

#[when("result ← lighting(m, light, position, eyev, normalv)")]
async fn compute_lighting(world: &mut MyWorld) {
    let material = &world.m;
    let object = Arc::new(default_sphere());
    let light = &world.light;
    let position = world.tuples.get("position").unwrap();
    let eyev = world.tuples.get("eyev").unwrap();
    let normalv = world.tuples.get("normalv").unwrap();
    let result = lighting(material, &object, light, position, eyev, normalv, false);
    world.tuples.insert("result".to_string(), result);
}

#[when("result ← lighting(m, light, position, eyev, normalv, in_shadow)")]
async fn compute_lighting_in_shadow(world: &mut MyWorld) {
    let material = &world.m;
    let object = Arc::new(default_sphere());
    let light = &world.light;
    let position = world.tuples.get("position").unwrap();
    let eyev = world.tuples.get("eyev").unwrap();
    let normalv = world.tuples.get("normalv").unwrap();
    let in_shadow = world.in_shadow;
    let result = lighting(material, &object, light, position, eyev, normalv, in_shadow);
    world.tuples.insert("result".to_string(), result);
}

#[when(
    regex = r"(c1|c2) ← lighting\(m, light, point\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\), eyev, normalv, false\)$"
)]
async fn compute_lighting_at_point(
    world: &mut MyWorld,
    target: String,
    x: String,
    y: String,
    z: String,
) {
    let material = &world.m;
    let object = Arc::new(default_sphere());
    let light = &world.light;
    let position = parse_point(&[x, y, z]);
    let eyev = world.tuples.get("eyev").unwrap();
    let normalv = world.tuples.get("normalv").unwrap();
    let in_shadow = false;
    let result = lighting(
        material, &object, light, &position, eyev, normalv, in_shadow,
    );
    world.tuples.insert(target, result);
}
