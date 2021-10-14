use std::rc::Rc;

use crate::MyWorld;
use cucumber_rust::Steps;
use lab_raytracing_rs::{
    camera::RAY_RECURSION_DEPTH,
    intersections::{color_at, reflected_color, refracted_color, shade_hit},
    lights::Pointlight,
    shapes::{default_sphere, default_testshape},
    transformations::scaling,
    tuples::{color, point},
    world::World,
};

pub fn steps() -> Steps<MyWorld<'static>> {
    let mut steps: Steps<MyWorld> = Steps::new();

    steps.given("w ← world()", |mut world, _ctx| {
        world.w = World::default();
        world
    });

    steps.given("w ← default_world()", |mut world, _ctx| {
        world.w = default_world();
        world
    });

    steps.when("w ← default_world()", |mut world, _ctx| {
        world.w = default_world();
        world
    });

    steps.then("w contains no objects", |world, _ctx| {
        assert_eq!(world.w.objects.len(), 0);
        world
    });

    steps.then("w has no light source", |world, _ctx| {
        assert!(world.w.light.is_none());
        world
    });

    steps.then("w.light = light", |world, _ctx| {
        assert_eq!(world.w.light.as_ref().unwrap(), &world.light);
        world
    });

    steps.then_regex(r#"^w contains (s1|s2)$"#, |world, ctx| {
        let object = world.shapes.get(&ctx.matches[1]).unwrap();
        assert!(world.w.objects.contains(object));
        world
    });

    steps.when("xs ← intersect_world(w, r)", |mut world, _ctx| {
        let xs = world.w.insersect(&world.r);
        world.xs = xs;
        world
    });

    steps.given_regex(
        r#"^(shape|outer|inner|A|B) ← the (first|second) object in w$"#,
        |mut world, ctx| {
            let index = match ctx.matches[2].as_str() {
                "first" => 0,
                "second" => 1,
                _ => panic!("position not covered"),
            };
            let shape = world.w.objects.get(index).unwrap();
            world.shapes.insert(ctx.matches[1].clone(), shape.clone());
            world
        },
    );

    steps.given_regex(
        r#"^(A|B|outer|inner) is the (first|second) object in w$"#,
        |mut world, ctx| {
            let index = match ctx.matches[2].as_str() {
                "first" => 0,
                "second" => 1,
                _ => panic!("position not covered"),
            };
            let object = world.shapes.get(&ctx.matches[1]).unwrap();
            world.w.objects[index] = object.clone();
            world
        },
    );

    steps.when_regex(
        r#"^(c|color) ← shade_hit\(w, comps\)$"#,
        |mut world, ctx| {
            let color = shade_hit(&world.w, &world.comps, RAY_RECURSION_DEPTH);
            world.tuples.insert(ctx.matches[1].clone(), color);
            world
        },
    );

    steps.when("color ← reflected_color(w, comps)", |mut world, _ctx| {
        let color = reflected_color(&world.w, &world.comps, RAY_RECURSION_DEPTH);
        world.tuples.insert("color".to_string(), color);
        world
    });

    steps.when_regex(
        r#"^(color) ← shade_hit\(w, comps, ([0-9]+)\)$"#,
        |mut world, ctx| {
            let remaining = ctx.matches[2].parse::<usize>().unwrap();
            let color = shade_hit(&world.w, &world.comps, remaining);
            world.tuples.insert(ctx.matches[1].clone(), color);
            world
        },
    );

    steps.when(
        "color ← reflected_color(w, comps, 0)",
        |mut world, _ctx| {
            let color = reflected_color(&world.w, &world.comps, 0);
            world.tuples.insert("color".to_string(), color);
            world
        },
    );

    steps.when_regex(
        r#"^c ← refracted_color\(w, comps, ([0-9]+)\)$"#,
        |mut world, ctx| {
            let remaining = ctx.matches[1].parse::<usize>().unwrap();
            let color = refracted_color(&world.w, &world.comps, remaining);
            world.tuples.insert("c".to_string(), color);
            world
        },
    );

    steps.when("c ← color_at(w, r)", |mut world, _ctx| {
        let none = &Rc::new(default_testshape());
        let color = color_at(&world.w, &world.r, RAY_RECURSION_DEPTH, none);
        world.tuples.insert("c".to_string(), color);
        world
    });

    steps.then(
        "color_at(w, r) should terminate successfully",
        |mut world, _ctx| {
            let none = &Rc::new(default_testshape());
            let color = color_at(&world.w, &world.r, RAY_RECURSION_DEPTH, none);
            world.tuples.insert("dummy".to_string(), color); // insert here to avoid removal by compiler
            world
        },
    );

    steps.then("c = inner.material.color", |world, _ctx| {
        let c = world.tuples.get("c").unwrap();
        assert_eq!(c, &world.shapes.get("inner").unwrap().material.color);
        world
    });

    steps.given("in_shadow ← true", |mut world, _ctx| {
        world.in_shadow = true;
        world
    });

    steps.then_regex(r#"^is_shadowed\(w, p\) is (true|false)$"#, |world, ctx| {
        let desired = ctx.matches[1].parse().unwrap();
        let point = world.tuples.get("p").unwrap();
        let computed = world
            .w
            .is_shadowed(point.clone(), &Rc::new(default_testshape()));
        assert_eq!(computed, desired);
        world
    });

    steps.given_regex(
        r#"^(s1|s2|shape|lower|upper|floor|ball) is added to w$"#,
        |mut world, ctx| {
            let shape = world.shapes.get(&ctx.matches[1]).unwrap();
            world.w.objects.push(shape.clone());
            world
        },
    );

    steps
}

pub fn default_world() -> World<'static> {
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

    w.objects = vec![s1, s2];
    w
}
