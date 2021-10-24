use crate::MyWorld;
use cucumber::{given, then};
use lab_raytracing_rs::objects::Shape;
use std::{ops::Deref, sync::Arc};

#[then(regex = r"^cyl.(minimum|maximum) = (-?infinity)$")]
async fn compare_cylinder_attribute(world: &mut MyWorld, attribute: String, value: String) {
    let obj = world.objects.get("cyl").unwrap();
    match obj.shape {
        Shape::Cylinder(min, max, _closed) => {
            let lookup = match attribute.as_str() {
                "minimum" => min,
                "maximum" => max,
                _ => panic!("cylinder property not covered"),
            };
            let desired = match value.as_str() {
                "-infinity" => f64::NEG_INFINITY,
                "infinity" => f64::INFINITY,
                _ => panic!("desired value not covered"),
            };
            assert!(lookup.eq(&desired));
        }
        _ => panic!("expected shape of kind cylinder"),
    }
}

#[then(regex = r"^cyl.closed = (true|false)$")]
async fn compare_cylinder_closed(world: &mut MyWorld, value: String) {
    let desired = match value.as_str() {
        "true" => true,
        "false" => false,
        _ => panic!("desired value not true of false"),
    };
    let obj = world.objects.get("cyl").unwrap();
    match obj.shape {
        Shape::Cylinder(_min, _max, closed) => {
            assert_eq!(closed, desired);
        }
        _ => panic!("expected shape of kind cylinder"),
    }
}

#[given(regex = r"^(cyl|shape).(minimum|maximum) ← ([-0-9.]+)$")]
async fn assign_cylinder_attribute(
    world: &mut MyWorld,
    shape: String,
    attribute: String,
    value: f64,
) {
    let mut obj = world.objects.get(&shape).unwrap().deref().clone();
    obj.shape = match obj.shape {
        Shape::Cylinder(min, max, closed) => match attribute.as_str() {
            "minimum" => Shape::Cylinder(value, max, closed),
            "maximum" => Shape::Cylinder(min, value, closed),
            _ => panic!("cylinder property not covered"),
        },
        Shape::Cone(min, max, closed) => match attribute.as_str() {
            "minimum" => Shape::Cone(value, max, closed),
            "maximum" => Shape::Cone(min, value, closed),
            _ => panic!("cone property not covered"),
        },
        _ => panic!("expected shape of kind cylinder"),
    };
    world.objects.insert(shape, Arc::new(obj));
}

#[given(regex = r"^(cyl|shape).closed ← (true|false)$")]
async fn assign_cylinder_closed(world: &mut MyWorld, shape: String, value: String) {
    let value = match value.as_str() {
        "true" => true,
        "false" => false,
        _ => panic!("value not true of false"),
    };
    let mut obj = world.objects.get(&shape).unwrap().deref().clone();
    obj.shape = match obj.shape {
        Shape::Cylinder(min, max, _closed) => Shape::Cylinder(min, max, value),
        Shape::Cone(min, max, _closed) => Shape::Cone(min, max, value),
        _ => panic!("expected shape of kind cylinder"),
    };
    world.objects.insert(shape, Arc::new(obj));
}
