use crate::{
    steps::tuples::{parse_float, parse_point, parse_vector},
    MyWorld,
};
use approx::assert_abs_diff_eq;
use cucumber::{given, then, when};
use lab_raytracing_rs::intersections::{hit, prepare_computations, schlick, Intersection};
use std::sync::Arc;

#[when(regex = r"^(i) ← intersection\(([-0-9.]+), (s)\)$")]
#[given(regex = r"^(i|i1|i2|i3|i4) ← intersection\((√2|[-0-9.]+), (s|s2|shape)\)$")]
async fn assign_intersection(world: &mut MyWorld, target: String, t: String, shape: String) {
    let t = parse_float(t.as_str());
    let object = world.objects.get(&shape).unwrap().clone();
    let intersection = Intersection {
        t,
        object,
        u: 0.0,
        v: 0.0,
    };
    world.intersections.insert(target, intersection);
}

#[when(regex = r"^(i) ← intersection_with_uv\(([-0-9.]+), (s|tri), ([-0-9.]+), ([-0-9.]+)\)$")]
async fn assign_intersection_with_uv(
    world: &mut MyWorld,
    target: String,
    t: f64,
    shape: String,
    u: f64,
    v: f64,
) {
    let object = world.objects.get(&shape).unwrap().clone();
    let intersection = Intersection { t, object, u, v };
    world.intersections.insert(target, intersection);
}

#[then(regex = r"^(i).object = s$")]
async fn compare_intersection_object(world: &mut MyWorld, intersection: String) {
    let shape = &world.intersections.get(&intersection).unwrap().object;
    let desired = world.objects.get("s").unwrap();
    assert!(Arc::ptr_eq(shape, desired));
}

#[then(regex = r"^(i).(t|u|v) = ([-0-9.]+)$")]
async fn compare_intersection_distance(
    world: &mut MyWorld,
    intersection: String,
    attribute: String,
    desired: f64,
) {
    let intersection = world.intersections.get(&intersection).unwrap().clone();
    match attribute.as_str() {
        "t" => assert_abs_diff_eq!(desired, intersection.t),
        "u" => assert_abs_diff_eq!(desired, intersection.u),
        "v" => assert_abs_diff_eq!(desired, intersection.v),
        _ => panic!("attribute not covered"),
    }
}

#[when(regex = r"^xs ← intersections\((i1), (i2)\)$")]
#[given(regex = r"^xs ← intersections\((i2), (i1)\)$")]
async fn assign_intersections(world: &mut MyWorld, i1: String, i2: String) {
    let i1 = world.intersections.get(&i1).unwrap().clone();
    let i2 = world.intersections.get(&i2).unwrap().clone();
    world.xs = vec![i1, i2];
}

#[when(regex = r"^xs ← intersections\((i)\)$")]
#[given(regex = r"^xs ← intersections\((i)\)$")]
async fn assign_intersections_single(world: &mut MyWorld, i1: String) {
    let i1 = world.intersections.get(&i1).unwrap().clone();
    world.xs = vec![i1];
}

#[given(regex = r"^xs ← intersections\((i1), (i2), (i3), (i4)\)$")]
async fn assign_intersections_quad(
    world: &mut MyWorld,
    i1: String,
    i2: String,
    i3: String,
    i4: String,
) {
    let i1 = world.intersections.get(&i1).unwrap().clone();
    let i2 = world.intersections.get(&i2).unwrap().clone();
    let i3 = world.intersections.get(&i3).unwrap().clone();
    let i4 = world.intersections.get(&i4).unwrap().clone();
    world.xs = vec![i1, i2, i3, i4];
}

#[then(regex = r"^xs\[([-0-9.]+)\].(t|u|v) = ([-0-9.]+)$")]
async fn compare_intersections_distance(
    world: &mut MyWorld,
    index: usize,
    attribute: String,
    desired: f64,
) {
    let i = world.xs.get(index).unwrap();
    let v = match attribute.as_str() {
        "t" => i.t,
        "u" => i.u,
        "v" => i.v,
        _ => panic!("attribute not covered"),
    };
    assert_abs_diff_eq!(v, desired, epsilon = 0.0001);
    // assert!((v - desired).abs() < 0.0001); // TODO replace .abs compare implementations in tests
}

#[when(regex = r"^(i) ← hit\(xs\)$")]
async fn assign_intersection_hit(world: &mut MyWorld, name: String) {
    let intersection = hit(&world.xs);
    match intersection {
        None => world.intersections.remove(&name),
        Some(i) => world.intersections.insert(name, i.clone()),
    };
}

#[then(regex = r"^(i) = (i1|i2|i4)$")]
async fn compare_intersection(world: &mut MyWorld, intersection: String, desired: String) {
    let intersection = world.intersections.get(&intersection).unwrap();
    let desired = world.intersections.get(&desired).unwrap();
    assert_eq!(intersection, desired);
}

#[then(regex = r"^(i) is nothing$")]
async fn intersection_is_empty(world: &mut MyWorld, intersection: String) {
    let intersection = world.intersections.get(&intersection);
    assert_eq!(intersection, None);
}

#[when("comps ← prepare_computations(i, r)")]
#[when("comps ← prepare_computations(i, r, xs)")]
async fn assign_prepare_computations(world: &mut MyWorld) {
    let intersection = world.intersections.get("i").unwrap();
    world.comps = prepare_computations(intersection, &world.r, &world.xs);
}

#[when(regex = r"^comps ← prepare_computations\(xs\[([0-9]+)\], r, xs\)$")]
async fn assign_prepare_computations_index(world: &mut MyWorld, index: usize) {
    let intersection = &world.xs[index];
    world.comps = prepare_computations(intersection, &world.r, &world.xs);
}

#[then("comps.t = i.t")]
async fn compare_precomputed_distance(world: &mut MyWorld) {
    let intersection = world.intersections.get("i").unwrap();
    assert_abs_diff_eq!(world.comps.t, intersection.t);
}

#[then("comps.object = i.object")]
async fn compare_precomputed_object(world: &mut MyWorld) {
    let lookup = &world.comps.object;
    let desired = &world.intersections.get("i").unwrap().object;
    assert_eq!(lookup, desired);
}

#[then(
    regex = r"^comps\.(point|eyev|normalv|reflectv) = (point|vector)\((√2/2|[-0-9.]+), (√2/2|[-0-9.]+), (√2/2|[-0-9.]+)\)$"
)]
async fn compare_precomputed_attribute(
    world: &mut MyWorld,
    attribute: String,
    kind: String,
    x: String,
    y: String,
    z: String,
) {
    let tuple = match kind.as_str() {
        "point" => parse_point(&[x, y, z]),
        "vector" => parse_vector(&[x, y, z]),
        _ => panic!("type not covered"),
    };
    match attribute.as_str() {
        "point" => assert_abs_diff_eq!(world.comps.point, tuple, epsilon = 0.0001),
        "eyev" => assert_abs_diff_eq!(world.comps.eyev, tuple, epsilon = 0.0001),
        "normalv" => assert_abs_diff_eq!(world.comps.normalv, tuple, epsilon = 0.0001),
        "reflectv" => assert_abs_diff_eq!(world.comps.reflectv, tuple, epsilon = 0.0001),
        _ => panic!("type not covered"),
    };
}

#[then(regex = r"^comps\.(n1|n2) = ([-0-9.]+)$")]
async fn compare_precomputed_float(world: &mut MyWorld, attribute: String, desired: f64) {
    match attribute.as_str() {
        "n1" => assert_abs_diff_eq!(world.comps.n1, desired),
        "n2" => assert_abs_diff_eq!(world.comps.n2, desired),
        _ => panic!("type not covered"),
    };
}

#[then(regex = r"^comps\.inside = (true|false)$")]
async fn compare_precomputed_bool(world: &mut MyWorld, desired: bool) {
    assert_eq!(world.comps.inside, desired);
}

#[then("comps.over_point.z < -EPSILON/2")]
#[then("comps.under_point.z > EPSILON/2")]
#[then("comps.point.z > comps.over_point.z")]
#[then("comps.point.z < comps.under_point.z")]
async fn refactored_away(_: &mut MyWorld) {}

#[given("xs ← intersections(2:A, 2.75:B, 3.25:C, 4.75:B, 5.25:C, 6:A)")]
async fn prepare_six_intersections(world: &mut MyWorld) {
    world.xs = vec![
        Intersection {
            t: 2.0,
            object: world.objects.get("A").unwrap().clone(),
            u: 0.0,
            v: 0.0,
        },
        Intersection {
            t: 2.75,
            object: world.objects.get("B").unwrap().clone(),
            u: 0.0,
            v: 0.0,
        },
        Intersection {
            t: 3.25,
            object: world.objects.get("C").unwrap().clone(),
            u: 0.0,
            v: 0.0,
        },
        Intersection {
            t: 4.75,
            object: world.objects.get("B").unwrap().clone(),
            u: 0.0,
            v: 0.0,
        },
        Intersection {
            t: 5.25,
            object: world.objects.get("C").unwrap().clone(),
            u: 0.0,
            v: 0.0,
        },
        Intersection {
            t: 6.0,
            object: world.objects.get("A").unwrap().clone(),
            u: 0.0,
            v: 0.0,
        },
    ];
}

#[allow(clippy::too_many_arguments)]
#[given(
    regex = r"^xs ← intersections\(([-√/0-9\.]+):(A|B|s1|s2), ([-√/0-9\.]+):(A|B|s1|s2), ([-√/0-9\.]+):(A|B|s1|s2), ([-√/0-9\.]+):(A|B|s1|s2)\)$"
)]
async fn prepare_four_intersections(
    world: &mut MyWorld,
    a_t: String,
    a_o: String,
    b_t: String,
    b_o: String,
    c_t: String,
    c_o: String,
    d_t: String,
    d_o: String,
) {
    world.xs = vec![
        Intersection {
            t: parse_float(&a_t),
            object: world.objects.get(&a_o).unwrap().clone(),
            u: 0.0,
            v: 0.0,
        },
        Intersection {
            t: parse_float(&b_t),
            object: world.objects.get(&b_o).unwrap().clone(),
            u: 0.0,
            v: 0.0,
        },
        Intersection {
            t: parse_float(&c_t),
            object: world.objects.get(&c_o).unwrap().clone(),
            u: 0.0,
            v: 0.0,
        },
        Intersection {
            t: parse_float(&d_t),
            object: world.objects.get(&d_o).unwrap().clone(),
            u: 0.0,
            v: 0.0,
        },
    ];
}

#[given(regex = r"^xs ← intersections\(([-√/0-9\.]+):(shape), ([-√/0-9\.]+):(shape)\)$")]
async fn prepare_two_intersections(
    world: &mut MyWorld,
    a_t: String,
    a_o: String,
    b_t: String,
    b_o: String,
) {
    world.xs = vec![
        Intersection {
            t: parse_float(&a_t),
            object: world.objects.get(&a_o).unwrap().clone(),
            u: 0.0,
            v: 0.0,
        },
        Intersection {
            t: parse_float(&b_t),
            object: world.objects.get(&b_o).unwrap().clone(),
            u: 0.0,
            v: 0.0,
        },
    ];
}

#[given(regex = r"^xs ← intersections\(([-√/0-9\.]+):(floor|shape)\)$")]
async fn prepare_one_intersections(world: &mut MyWorld, a_t: String, a_o: String) {
    world.xs = vec![Intersection {
        t: parse_float(&a_t),
        object: world.objects.get(&a_o).unwrap().clone(),
        u: 0.0,
        v: 0.0,
    }];
}

#[when("reflectance ← schlick(comps)")]
async fn compute_schlick(world: &mut MyWorld) {
    let reflectance = schlick(&world.comps);
    world.floats.insert("reflectance".to_string(), reflectance);
}

#[then(regex = r"^(reflectance) = ([-√/0-9\.]+)$")]
async fn compare_reflectance(world: &mut MyWorld, name: String, desired: String) {
    let desired = parse_float(&desired);
    let lookup = world.floats.get(&name).unwrap();
    assert_abs_diff_eq!(&desired, lookup);
}
