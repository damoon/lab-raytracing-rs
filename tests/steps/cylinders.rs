use crate::MyWorld;
use cucumber::Steps;
use lab_raytracing_rs::objects::Shape;
use std::{ops::Deref, sync::Arc};

pub fn steps() -> Steps<MyWorld> {
    let mut steps: Steps<MyWorld> = Steps::new();

    steps.then_regex(r#"cyl.(minimum|maximum) = (-?infinity)"#, |world, ctx| {
        let obj = world.objects.get("cyl").unwrap();
        match obj.shape {
            Shape::Cylinder(min, max, _closed) => {
                let lookup = match ctx.matches[1].as_str() {
                    "minimum" => min,
                    "maximum" => max,
                    _ => panic!("cylinder property not covered"),
                };
                let desired = match ctx.matches[2].as_str() {
                    "-infinity" => f64::NEG_INFINITY,
                    "infinity" => f64::INFINITY,
                    _ => panic!("desired value not covered"),
                };
                assert!(lookup.eq(&desired));
            }
            _ => panic!("expected shape of kind cylinder"),
        }
        world
    });

    steps.then_regex(r#"cyl.(closed) = (true|false)"#, |world, ctx| {
        let desired = match ctx.matches[2].as_str() {
            "true" => true,
            "false" => false,
            _ => panic!("desired value not true of false"),
        };
        let obj = world.objects.get("cyl").unwrap();
        match obj.shape {
            Shape::Cylinder(_min, _max, closed) => {
                let lookup = match ctx.matches[1].as_str() {
                    "closed" => closed,
                    _ => panic!("cylinder property not covered"),
                };
                assert_eq!(lookup, desired);
            }
            _ => panic!("expected shape of kind cylinder"),
        }
        world
    });

    steps.given_regex(
        r#"(cyl|shape).(minimum|maximum) ← ([-0-9.]+)"#,
        |mut world, ctx| {
            let value = ctx.matches[3].parse::<f64>().unwrap();
            let mut obj = world.objects.get(&ctx.matches[1]).unwrap().deref().clone();
            obj.shape = match obj.shape {
                Shape::Cylinder(min, max, closed) => match ctx.matches[2].as_str() {
                    "minimum" => Shape::Cylinder(value, max, closed),
                    "maximum" => Shape::Cylinder(min, value, closed),
                    _ => panic!("cylinder property not covered"),
                },
                Shape::Cone(min, max, closed) => match ctx.matches[2].as_str() {
                    "minimum" => Shape::Cone(value, max, closed),
                    "maximum" => Shape::Cone(min, value, closed),
                    _ => panic!("cone property not covered"),
                },
                _ => panic!("expected shape of kind cylinder"),
            };
            world.objects.insert(ctx.matches[1].clone(), Arc::new(obj));
            world
        },
    );

    steps.given_regex(
        r#"(cyl|shape).(closed) ← (true|false)"#,
        |mut world, ctx| {
            let value = match ctx.matches[3].as_str() {
                "true" => true,
                "false" => false,
                _ => panic!("value not true of false"),
            };
            let mut obj = world.objects.get(&ctx.matches[1]).unwrap().deref().clone();
            obj.shape = match obj.shape {
                Shape::Cylinder(min, max, _closed) => match ctx.matches[2].as_str() {
                    "closed" => Shape::Cylinder(min, max, value),
                    _ => panic!("cylinder property not covered"),
                },
                Shape::Cone(min, max, _closed) => match ctx.matches[2].as_str() {
                    "closed" => Shape::Cone(min, max, value),
                    _ => panic!("cone property not covered"),
                },
                _ => panic!("expected shape of kind cylinder"),
            };
            world.objects.insert(ctx.matches[1].clone(), Arc::new(obj));
            world
        },
    );

    steps
}
