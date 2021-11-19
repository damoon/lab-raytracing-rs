use crate::MyWorld;
use approx::assert_abs_diff_eq;
use cucumber::{given, then, when};
use lab_raytracing_rs::tuples::{color, cross, dot, point, reflect, vector, Tuple};

pub fn parse_float(s: &str) -> f64 {
    match s {
        "√2" => 2.0_f64.sqrt(),
        "-√2" => -(2.0_f64.sqrt()),
        "√2/2" => 2.0_f64.sqrt() / 2.0_f64,
        "-√2/2" => -(2.0_f64.sqrt()) / 2.0_f64,
        "√3/3" => 3.0_f64.sqrt() / 3.0_f64,
        "-√3/3" => -(3.0_f64.sqrt()) / 3.0_f64,
        s => s.parse::<f64>().unwrap(),
    }
}

pub fn parse_tuple(ss: &[String]) -> Tuple {
    let x = parse_float(ss[0].as_str());
    let y = parse_float(ss[1].as_str());
    let z = parse_float(ss[2].as_str());
    let w = parse_float(ss[3].as_str());
    Tuple::new(x, y, z, w)
}

pub fn parse_point(ss: &[String]) -> Tuple {
    let x = parse_float(ss[0].as_str());
    let y = parse_float(ss[1].as_str());
    let z = parse_float(ss[2].as_str());
    point(x, y, z)
}

pub fn parse_vector(ss: &[String]) -> Tuple {
    let x = parse_float(ss[0].as_str());
    let y = parse_float(ss[1].as_str());
    let z = parse_float(ss[2].as_str());
    vector(x, y, z)
}

pub fn parse_color(ss: &[String]) -> Tuple {
    let r = parse_float(ss[0].as_str());
    let g = parse_float(ss[1].as_str());
    let b = parse_float(ss[2].as_str());
    color(r, g, b)
}

#[given(regex = r"^(a|a1|a2|n|b) ← tuple\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$")]
async fn set_tuple(world: &mut MyWorld, name: String, x: f64, y: f64, z: f64, w: f64) {
    let tuple = Tuple::new(x, y, z, w);
    world.tuples.insert(name, tuple);
}

#[then(regex = r"^(a|c).(x|y|z|w|red|green|blue) = ([-0-9.]+)$")]
async fn compare_value(world: &mut MyWorld, name: String, attribute: String, desired: f64) {
    let tuple = world.tuples.get(&name).unwrap();
    let value = match attribute.as_str() {
        "x" => tuple.x,
        "y" => tuple.y,
        "z" => tuple.z,
        "w" => tuple.w,
        "red" => tuple.x,
        "green" => tuple.y,
        "blue" => tuple.z,
        _ => panic!("Invalid attribute checked"),
    };
    assert_abs_diff_eq!(desired, value);
}

#[then(regex = r"^a is (not )?a (point|vector)$")]
async fn tuple_kind(world: &mut MyWorld, not: String, kind: String) {
    let tuple = world.tuples.get("a").unwrap();
    assert!(match (not.as_str(), kind.as_str()) {
        ("", "point") => tuple.is_point(),
        ("not ", "point") => !tuple.is_point(),
        ("", "vector") => tuple.is_vector(),
        ("not ", "vector") => !tuple.is_vector(),
        (_, _) => false,
    });
}

#[given(
    regex = r"^(a|b|p|v|p1|p2|p3|v1|v2|n1|n2|n3|zero|c|c1|c2|c3|n|red|from|to|up|origin|direction|intensity|eyev|normalv|black|white|position) ← (point|vector|color)\(([-0-9.]+|-?√2/2), ([-0-9.]+|-?√2/2), ([-0-9.]+|-?√2/2)\)$"
)]
async fn set_tuple_kind(
    world: &mut MyWorld,
    name: String,
    kind: String,
    x: String,
    y: String,
    z: String,
) {
    let tuple = match kind.as_str() {
        "point" => parse_point(&[x, y, z]),
        "vector" => parse_vector(&[x, y, z]),
        "color" => parse_color(&[x, y, z]),
        _ => panic!("type not covered"),
    };
    world.tuples.insert(name, tuple);
}

#[when(regex = r"^(r) ← reflect\((v), (n)\)$")]
async fn calculate_reflection(world: &mut MyWorld, ray: String, vector: String, normal: String) {
    let vector = world.tuples.get(&vector).unwrap();
    let normal = world.tuples.get(&normal).unwrap();
    let reflected = reflect(vector, normal);
    world.tuples.insert(ray, reflected);
}

#[then(regex = r"^(-?)(p|v|a) = tuple\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$")]
async fn negate_tuple(
    world: &mut MyWorld,
    negation: String,
    name: String,
    x: String,
    y: String,
    z: String,
    w: String,
) {
    let desired_tuple = parse_tuple(&[x, y, z, w]);
    let mut tuple = world.tuples.get(&name).unwrap().clone();
    if &negation == "-" {
        tuple = -tuple;
    }
    eq_tuples_similar(&desired_tuple, &tuple);
}

#[then(regex = r"^(a1) \+ (a2) = tuple\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$")]
async fn compare_tuple(
    world: &mut MyWorld,
    this: String,
    other: String,
    x: String,
    y: String,
    z: String,
    w: String,
) {
    let desired_tuple = parse_tuple(&[x, y, z, w]);
    let tuple1 = world.tuples.get(&this).unwrap();
    let tuple2 = world.tuples.get(&other).unwrap();
    let computed_tuple = tuple1 + tuple2;
    eq_tuples_similar(&computed_tuple, &desired_tuple);
}

#[then(regex = r"^(p2|p3|p4) = point\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$")]
async fn compare_point(world: &mut MyWorld, name: String, x: String, y: String, z: String) {
    let point = world.tuples.get(&name).unwrap().clone();
    let desired_color = parse_color(&[x, y, z]);
    eq_tuples_similar(&point, &desired_color);
}

#[then(
    regex = r"^(n|r|n1|n2|n3|normal) = vector\(([-0-9.]+|\-?√2|\-?√2/2|\-?√3/3), ([-0-9.]+|\-?√2|\-?√2/2|\-?√3/3), ([-0-9.]+|\-?√2|\-?√2/2|\-?√3/3)\)$"
)]
async fn compare_vector(world: &mut MyWorld, name: String, x: String, y: String, z: String) {
    let tuple = world.tuples.get(&name).unwrap().clone();
    let desired_vector = parse_vector(&[x, y, z]);
    eq_tuples_similar(&tuple, &desired_vector);
}

#[then(regex = r"^(c1) \+ (c2) = color\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$")]
async fn add_colors(
    world: &mut MyWorld,
    color_1: String,
    color_2: String,
    x: String,
    y: String,
    z: String,
) {
    let color_1 = world.tuples.get(&color_1).unwrap().clone();
    let color_2 = world.tuples.get(&color_2).unwrap().clone();
    let color = color_1 + color_2;
    let desired_color = parse_color(&[x, y, z]);
    eq_tuples_similar(&color, &desired_color);
}

#[then(regex = r"^(c|c1|c2|color) = color\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$")]
async fn compare_color(world: &mut MyWorld, name: String, x: String, y: String, z: String) {
    let color = world.tuples.get(&name).unwrap().clone();
    let desired_color = parse_color(&[x, y, z]);
    eq_tuples_similar(&color, &desired_color);
}

#[then(regex = r"^(c) = (white)$")]
async fn compare_tuples(world: &mut MyWorld, this: String, other: String) {
    let lookup = world.tuples.get(&this).unwrap();
    let desired = world.tuples.get(&other).unwrap();
    eq_tuples_similar(lookup, desired);
}

#[then(
    regex = r"^(p|p1|v1|zero|c1) \- (v|p2|v2|c2) = (vector|point|color)\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"
)]
async fn substract_tuples(
    world: &mut MyWorld,
    this: String,
    other: String,
    kind: String,
    x: String,
    y: String,
    z: String,
) {
    let tuple1 = world.tuples.get(&this).unwrap();
    let tuple2 = world.tuples.get(&other).unwrap();
    let tuple = tuple1 - tuple2;
    let desired_tuple = match kind.as_str() {
        "point" => parse_point(&[x, y, z]),
        "vector" => parse_vector(&[x, y, z]),
        "color" => parse_color(&[x, y, z]),
        _ => panic!("type not covered"),
    };
    eq_tuples_similar(&desired_tuple, &tuple);
}

#[then(regex = r"^a (\*|/) ([-0-9.]+) = tuple\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$")]
async fn scale_tuples(
    world: &mut MyWorld,
    operation: String,
    factor: f64,
    x: String,
    y: String,
    z: String,
    w: String,
) {
    let tuple = world.tuples.get("a").unwrap();
    let calculated = match operation.as_str() {
        "*" => tuple * factor,
        "/" => tuple / factor,
        _ => panic!("operation not covered"),
    };
    let desired_tuple = parse_tuple(&[x, y, z, w]);
    eq_tuples_similar(&calculated, &desired_tuple);
}

#[then(regex = r"^(c) \* ([-0-9.]+) = color\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$")]
async fn scale_color(
    world: &mut MyWorld,
    name: String,
    factor: f64,
    x: String,
    y: String,
    z: String,
) {
    let tuple = world.tuples.get(&name).unwrap();
    let calculated = tuple * factor;
    let desired_color = parse_color(&[x, y, z]);
    eq_tuples_similar(&calculated, &desired_color);
}

#[then(regex = r"^(c1) \* (c2) = color\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$")]
async fn multiply_color(
    world: &mut MyWorld,
    this: String,
    other: String,
    x: String,
    y: String,
    z: String,
) {
    let color_1 = world.tuples.get(&this).unwrap();
    let color_2 = world.tuples.get(&other).unwrap();
    let calculated = color_1 * color_2;
    let desired_color = parse_color(&[x, y, z]);
    eq_tuples_similar(&calculated, &desired_color);
}

#[then(regex = r"^(result) = color\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$")]
async fn assign_color(world: &mut MyWorld, name: String, x: String, y: String, z: String) {
    let color = world.tuples.get(&name).unwrap();
    let desired_color = parse_color(&[x, y, z]);
    eq_tuples_similar(color, &desired_color);
}

#[then(regex = r"^magnitude\((v|norm)\) = (√14|[-0-9.]+)$")]
async fn compare_magnitude(world: &mut MyWorld, name: String, desired: String) {
    let calculated = world.tuples.get(&name).unwrap().magnitude();
    let desired = match desired.as_str() {
        "√14" => 14.0_f64.sqrt(),
        a => a.parse::<f64>().unwrap(),
    };
    assert_abs_diff_eq!(calculated, desired);
}

#[then(
    regex = r"^normalize\((v)\) = (approximately )?vector\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"
)]
async fn compare_normalize(
    world: &mut MyWorld,
    name: String,
    approximately: String,
    x: String,
    y: String,
    z: String,
) {
    let calculated = world.tuples.get(&name).unwrap().normalize();
    let desired = parse_vector(&[x, y, z]);
    if approximately == "approximately " {
        eq_tuples_similar(&desired, &calculated);
    } else {
        assert_eq!(desired, calculated);
    }
}

#[given(regex = r"^(direction) ← normalize\(vector\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)\)$")]
async fn assign_normalized_vector(
    world: &mut MyWorld,
    name: String,
    x: String,
    y: String,
    z: String,
) {
    let normalized = parse_vector(&[x, y, z]).normalize();
    world.tuples.insert(name, normalized);
}

#[when(regex = r"^(norm) ← normalize\((v)\)$")]
async fn assign_normalized_tuple(world: &mut MyWorld, target: String, origin: String) {
    let tuple = world.tuples.get(&origin).unwrap().normalize();
    world.tuples.insert(target, tuple);
}

#[then(regex = r"^(n) = normalize\((n)\)$")]
async fn check_is_normalized(world: &mut MyWorld, desired: String, origin: String) {
    let tuple = world.tuples.get(&origin).unwrap().normalize();
    world.tuples.insert(desired, tuple);
}

#[then(regex = r"^dot\((a), (b)\) = ([-0-9.]+)$")]
async fn compute_dot(world: &mut MyWorld, this: String, other: String, desired: f64) {
    let tuple1 = world.tuples.get(&this).unwrap();
    let tuple2 = world.tuples.get(&other).unwrap();
    let dot = dot(tuple1, tuple2);
    assert_abs_diff_eq!(dot, desired);
}

#[then(regex = r"^cross\((a|b), (a|b)\) = vector\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$")]
async fn compute_cross(
    world: &mut MyWorld,
    this: String,
    other: String,
    x: String,
    y: String,
    z: String,
) {
    let tuple1 = world.tuples.get(&this).unwrap();
    let tuple2 = world.tuples.get(&other).unwrap();
    let cross = cross(tuple1, tuple2);
    let desired = parse_vector(&[x, y, z]);
    assert_eq!(cross, desired);
}

pub fn eq_tuples_similar(this: &Tuple, other: &Tuple) -> bool {
    if (this.x - other.x).abs() > 0.0001 {
        return false;
    }
    if (this.y - other.y).abs() > 0.0001 {
        return false;
    }
    if (this.z - other.z).abs() > 0.0001 {
        return false;
    }
    if (this.w - other.w).abs() > 0.0001 {
        return false;
    }
    true
}
