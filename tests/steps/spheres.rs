use super::tuples::parse_point;
use crate::MyWorld;
use approx::assert_abs_diff_eq;
use cucumber::{gherkin::Step, given, then, when};
use lab_raytracing_rs::transformations::scaling;
use lab_raytracing_rs::{
    intersections::Intersection,
    matrices::Matrix4x4,
    objects::{
        default_cone, default_cube, default_cylinder, default_plane, default_sphere, glass_sphere,
    },
    patterns::test_pattern,
    transformations::translation,
    tuples::{color, Tuple},
};
use regex::Regex;
use std::{ops::Deref, sync::Arc};

#[given(
    regex = r"^(s|shape|s1|object|c|cyl|s2|s3) ← (sphere|plane|glass_sphere|cube|cylinder|cone)\(\)$"
)]
async fn create_shape(world: &mut MyWorld, name: String, kind: String) {
    let s = match kind.as_str() {
        "sphere" => default_sphere(),
        "plane" => default_plane(),
        "glass_sphere" => glass_sphere(),
        "cube" => default_cube(),
        "cylinder" => default_cylinder(),
        "cone" => default_cone(),
        _ => panic!("object kind not covered"),
    };
    world.objects.insert(name, Arc::new(s));
}

#[given(
    regex = r"^(s1|s2|shape|lower|upper|A|B|C|floor|ball) ← (sphere|plane|glass_sphere)\(\) with:$"
)]
async fn create_shape_with(world: &mut MyWorld, name: String, kind: String, step: &Step) {
    let mut s = match kind.as_str() {
        "sphere" => default_sphere(),
        "plane" => default_plane(),
        "glass_sphere" => glass_sphere(),
        _ => panic!("object kind not covered"),
    };
    for row in &step.table.as_ref().unwrap().rows {
        let key = row.get(0).unwrap();
        let value = row.get(1).unwrap();
        match key.as_str() {
            "material.color" => s.material.color = color_from_string(value),
            "material.ambient" => s.material.ambient = value.parse::<f64>().unwrap(),
            "material.diffuse" => s.material.diffuse = value.parse::<f64>().unwrap(),
            "material.specular" => s.material.specular = value.parse::<f64>().unwrap(),
            "material.reflective" => s.material.reflective = value.parse::<f64>().unwrap(),
            "material.transparency" => s.material.transparency = value.parse::<f64>().unwrap(),
            "material.refractive_index" => {
                s.material.refractive_index = value.parse::<f64>().unwrap()
            }
            "transform" => s.set_transform(transform_from_string(value)),
            _ => panic!("object property not covered"),
        }
    }
    world.objects.insert(name, Arc::new(s));
}

#[given(regex = r"^(shape|A|B) has:$")]
async fn shape_with(world: &mut MyWorld, name: String, step: &Step) {
    let mut s = world.objects.get(&name).unwrap().deref().clone();
    for row in &step.table.as_ref().unwrap().rows {
        let key = row.get(0).unwrap();
        let value = row.get(1).unwrap();
        match (key.as_str(), value.as_str()) {
            ("material.ambient", value) => s.material.ambient = value.parse::<f64>().unwrap(),
            ("material.pattern", "test_pattern()") => {
                s.material.pattern = Some(Box::new(test_pattern()))
            }
            ("material.transparency", value) => {
                s.material.transparency = value.parse::<f64>().unwrap()
            }
            ("material.refractive_index", value) => {
                s.material.refractive_index = value.parse::<f64>().unwrap()
            }
            _ => panic!("object property not covered"),
        }
    }
    world.objects.insert(name, Arc::new(s));
}

#[when(regex = r"^m ← s.material$")]
async fn select_material(world: &mut MyWorld) {
    world.m = world.objects.get("s").unwrap().material.clone();
}

#[when(regex = r"^(s).material ← m$")]
async fn assign_material(world: &mut MyWorld, name: String) {
    let mut obj = world
        .objects
        .get_mut(&name)
        .unwrap()
        .deref()
        .deref()
        .clone();
    obj.material = world.m.clone();
    world.objects.insert(name, Arc::new(obj));
}

#[then(regex = r"^(s).material = m$")]
async fn match_material(world: &mut MyWorld, name: String) {
    let s = world.objects.get(&name).unwrap();
    assert_eq!(s.material, world.m);
}

#[given(regex = r"^set_transform\((s), (m|t)\)$")]
#[when(regex = r"^set_transform\((s), (m|t)\)$")]
async fn set_translation(world: &mut MyWorld, shape: String, matrix: String) {
    let transformation = world.get4x4(&matrix).clone();
    let mut obj = world
        .objects
        .get_mut(&shape)
        .unwrap()
        .deref()
        .deref()
        .clone();
    obj.set_transform(transformation);
    world.objects.insert(shape, Arc::new(obj));
}

#[given(
    regex = r"^set_transform\((s|object|shape|s2|s3), (scaling|translation)\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)\)$"
)]
#[when(
    regex = r"^set_transform\((s|object|shape|s2|s3), (scaling|translation)\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)\)$"
)]
async fn create_set_translation(
    world: &mut MyWorld,
    shape: String,
    transformation: String,
    x: f64,
    y: f64,
    z: f64,
) {
    let transformation = match transformation.as_str() {
        "scaling" => scaling(x, y, z),
        "translation" => translation(x, y, z),
        _ => panic!("transformation not covered"),
    };
    let mut obj = world.objects.get(&shape).unwrap().deref().clone();
    obj.set_transform(transformation);
    world.objects.insert(shape, Arc::new(obj));
}

#[then(regex = r"^(s).transform = (identity_matrix|t)$")]
async fn compare_translation(world: &mut MyWorld, name: String, translation: String) {
    let lookup = world.objects.get(&name).unwrap().transform();
    let desired = world.get4x4(&translation);
    assert_eq!(lookup, desired);
}

#[when(regex = r"^xs ← intersect\(s, r\)$")]
async fn calculate_intersections(world: &mut MyWorld) {
    let s = world.objects.get("s").unwrap();
    world.xs = s
        .intersect(&world.r)
        .iter()
        .map(|t| Intersection {
            t: *t,
            object: s.clone(),
            u: 0.0,
            v: 0.0,
        })
        .collect();
}

#[then(regex = r"^xs.count = ([-0-9.]+)$")]
async fn count_intersections(world: &mut MyWorld, desired: usize) {
    assert_eq!(world.xs.len(), desired);
}

#[then(regex = r"^xs\[([-0-9.]+)\] = ([-0-9.]+)$")]
async fn check_intersection_distance(world: &mut MyWorld, index: usize, desired: f64) {
    let value = world.xs.get(index).unwrap().t;
    assert_abs_diff_eq!(value, desired);
}

#[then(regex = r"^xs\[([-0-9.]+)\].object = (s)$")]
async fn check_intersection_object(world: &mut MyWorld, index: usize, object: String) {
    let desired = world.objects.get(&object).unwrap();
    let lookup = world.xs.get(index).unwrap().object.clone();
    assert_eq!(&lookup, desired);
}

#[when(
    regex = r"^(n) ← normal_at\(s, point\((√3/3|[-0-9.]+), (√3/3|√2/2|[-0-9.]+), (√3/3|-√2/2|[-0-9.]+)\)\)$"
)]
async fn calculate_normal(world: &mut MyWorld, name: String, x: String, y: String, z: String) {
    let point = parse_point(&[x, y, z]);
    let normal = world.objects.get("s").unwrap().normal_at(&point);
    world.tuples.insert(name, normal);
}

fn color_from_string(s: &str) -> Tuple {
    let re = Regex::new(r#"\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)"#).unwrap();
    let captures = re.captures(s).unwrap();
    let r = captures.get(1).unwrap().as_str().parse::<f64>().unwrap();
    let g = captures.get(2).unwrap().as_str().parse::<f64>().unwrap();
    let b = captures.get(3).unwrap().as_str().parse::<f64>().unwrap();
    color(r, g, b)
}

fn transform_from_string(s: &str) -> Matrix4x4 {
    let re = Regex::new(r#"(scaling|translation)\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)"#).unwrap();
    let captures = re.captures(s).expect("transform not covered");
    let x = captures.get(2).unwrap().as_str().parse::<f64>().unwrap();
    let y = captures.get(3).unwrap().as_str().parse::<f64>().unwrap();
    let z = captures.get(4).unwrap().as_str().parse::<f64>().unwrap();
    match captures.get(1).unwrap().as_str() {
        "scaling" => scaling(x, y, z),
        "translation" => translation(x, y, z),
        _ => panic!("transformation not covered"),
    }
}
