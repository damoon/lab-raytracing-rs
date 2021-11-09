use crate::{steps::tuples::parse_color, MyWorld};
use approx::assert_abs_diff_eq;
use cucumber::{given, then};
use lab_raytracing_rs::materials::Material;
use std::{ops::Deref, sync::Arc};

#[given("m ← material()")]
async fn assign_material(world: &mut MyWorld) {
    world.m = Material::default();
}

#[then("m = material()")]
async fn compare_material(world: &mut MyWorld) {
    assert_eq!(world.m, Material::default());
}

#[then(regex = r"^m.color = color\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$")]
async fn compare_material_color(world: &mut MyWorld, r: String, g: String, b: String) {
    let color = parse_color(&[r, g, b]);
    assert_eq!(world.m.color, color);
}

#[then(
    regex = r"^(m|s.material).(ambient|diffuse|specular|shininess|reflective|transparency|refractive_index) = ([-0-9.]+)$"
)]
async fn compare_material_attribute(
    world: &mut MyWorld,
    material: String,
    attribute: String,
    desired: f64,
) {
    let material = match material.as_str() {
        "m" => world.m.clone(),
        "s.material" => world.objects.get("s").unwrap().material.clone(),
        _ => panic!("material origin not covered"),
    };
    let value = match attribute.as_str() {
        "ambient" => material.ambient,
        "diffuse" => material.diffuse,
        "specular" => material.specular,
        "shininess" => material.shininess,
        "reflective" => material.reflective,
        "transparency" => material.transparency,
        "refractive_index" => material.refractive_index,
        _ => panic!("material attribute not covered"),
    };
    assert_abs_diff_eq!(value, desired);
}

#[given(regex = r"^m.(ambient|diffuse|specular) ← ([-0-9.]+)$")]
async fn assign_material_attribute(world: &mut MyWorld, attribute: String, value: f64) {
    match attribute.as_str() {
        "ambient" => world.m.ambient = value,
        "diffuse" => world.m.diffuse = value,
        "specular" => world.m.specular = value,
        _ => panic!("material attribute not covered"),
    };
}

#[given(regex = r"^(outer|inner|shape).material.ambient ← ([-0-9.]+)$")]
async fn assign_material_ambient(world: &mut MyWorld, material: String, value: f64) {
    let mut object = world.objects.get(&material).unwrap().deref().clone();
    object.material.ambient = value;
    world.objects.insert(material, Arc::new(object));
}
