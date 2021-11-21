use crate::MyWorld;
use cucumber::{given, then, when};
use lab_raytracing_rs::{
    csg::CSG,
    groups::GroupMember,
    objects::{default_cube, default_sphere},
};
use std::sync::Arc;

#[given(regex = r#"^csg ← csg\("(union|difference|intersection)", (s1), (s2)\)$"#)]
#[when(regex = r#"^csg ← csg\("(union|difference|intersection)", (s1), (s2)\)$"#)]
async fn create_csg_union(
    world: &mut MyWorld,
    operation: String,
    shape_1: String,
    shape_2: String,
) {
    let shape_1 = GroupMember::Object(world.objects.get(&shape_1).unwrap().clone());
    let shape_2 = GroupMember::Object(world.objects.get(&shape_2).unwrap().clone());
    world.csg = match operation.as_str() {
        "union" => CSG::Union(shape_1, shape_2),
        "difference" => CSG::Difference(shape_1, shape_2),
        "intersection" => CSG::Intersection(shape_1, shape_2),
        _ => panic!("operation not covered"),
    };
}

#[given("csg ← csg(\"union\", sphere(), cube())")]
async fn create_csg_default_union(world: &mut MyWorld) {
    let s1 = GroupMember::Object(Arc::new(default_sphere()));
    let s2 = GroupMember::Object(Arc::new(default_cube()));
    world.csg = CSG::Union(s1, s2);
}

#[then(regex = r#"^csg.operation = "(union|difference|intersection)"$"#)]
async fn compare_csg_operation(world: &mut MyWorld, operation: String) {
    match &world.csg {
        CSG::Union(_, _) => assert_eq!(operation, "union"),
        CSG::Difference(_, _) => assert_eq!(operation, "difference"),
        CSG::Intersection(_, _) => assert_eq!(operation, "intersection"),
    }
}

#[then(regex = r#"^csg.(left|right) = (s1|s2)$"#)]
async fn compare_csg_children(world: &mut MyWorld, child: String, shape: String) {
    let shape = &world.objects.get(&shape).unwrap().clone();
    if let GroupMember::Object(child) = match &world.csg {
        CSG::Union(left, right) => match child.as_str() {
            "left" => left,
            "right" => right,
            _ => panic!("child not covered"),
        },
        CSG::Difference(left, right) => match child.as_str() {
            "left" => left,
            "right" => right,
            _ => panic!("child not covered"),
        },
        CSG::Intersection(left, right) => match child.as_str() {
            "left" => left,
            "right" => right,
            _ => panic!("child not covered"),
        },
    } {
        assert_eq!(child, shape);
    } else {
        panic!("child is not an object")
    }
}

#[when(
    regex = r#"^result ← intersection_allowed\("(union|difference|intersection)", (true|false), (true|false), (true|false)\)$"#
)]
async fn compute_intersection_allowed(
    world: &mut MyWorld,
    operation: String,
    lhit: bool,
    inl: bool,
    inr: bool,
) {
    let s1 = GroupMember::Object(Arc::new(default_sphere()));
    let s2 = GroupMember::Object(Arc::new(default_cube()));
    let csg = match operation.as_str() {
        "union" => CSG::Union(s1, s2),
        "intersection" => CSG::Intersection(s1, s2),
        "difference" => CSG::Difference(s1, s2),
        _ => panic!("operation not covered"),
    };
    world.result = csg.intersection_allowed(lhit, inl, inr);
}

#[then(regex = r#"^result = (true|false)$"#)]
async fn compare_result(world: &mut MyWorld, desired: bool) {
    assert_eq!(world.result, desired);
}

#[when("xs_filtered ← filter_intersections(csg, xs)")]
async fn filter_intersections(world: &mut MyWorld) {
    world.xs_filtered = world.csg.filter_intersections(&world.xs);
}

#[when("xs ← local_intersect(csg, r)")]
async fn intersect_csg(world: &mut MyWorld) {
    world.xs = world.csg.intersect(&world.r);
}
