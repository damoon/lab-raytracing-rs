use crate::MyWorld;
use cucumber_rust::Steps;
use lab_raytracing_rs::{
    intersections::{color_at, shade_hit},
    lights::Pointlight,
    spheres::Sphere,
    transformations::scaling,
    tuples::{color, point},
    world::World,
};

pub fn steps() -> Steps<MyWorld> {
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
        let object = match ctx.matches[1].as_str() {
            "s1" => &world.s1,
            "s2" => &world.s2,
            _ => panic!("object not covered"),
        };
        assert!(world.w.objects.contains(object));
        world
    });

    steps.when("xs ← intersect_world(w, r)", |mut world, _ctx| {
        world.xs = world.w.insersect(&world.r);
        world
    });

    steps.given_regex(
        r#"^(shape|outer|inner) ← the (first|second) object in w$"#,
        |mut world, ctx| {
            let index = match ctx.matches[2].as_str() {
                "first" => 0,
                "second" => 1,
                _ => panic!("position not covered"),
            };
            match ctx.matches[1].as_str() {
                "shape" => world.shape = world.w.objects.get(index).unwrap().clone(),
                "outer" => world.outer = world.w.objects.get(index).unwrap().clone(),
                "inner" => world.inner = world.w.objects.get(index).unwrap().clone(),
                _ => panic!("object not covered"),
            };

            world
        },
    );

    steps.when("c ← shade_hit(w, comps)", |mut world, _ctx| {
        let color = shade_hit(&world.w, &world.comps);
        world.tuples.insert("c".to_string(), color);
        world
    });

    steps.when("c ← color_at(w, r)", |mut world, _ctx| {
        let color = color_at(&world.w, &world.r);
        world.tuples.insert("c".to_string(), color);
        world
    });

    steps.then("c = inner.material.color", |world, _ctx| {
        let c = world.tuples.get("c").unwrap();
        assert_eq!(c, &world.inner.material.color);
        world
    });

    steps
}

pub fn default_world() -> World {
    let mut w = World::default();
    w.light = Some(Pointlight::new(
        point(-10.0, 10.0, -10.0),
        color(1.0, 1.0, 1.0),
    ));

    let mut s1 = Sphere::default();
    s1.material.color = color(0.8, 1.0, 0.6);
    s1.material.diffuse = 0.7;
    s1.material.specular = 0.2;

    let mut s2 = Sphere::default();
    s2.transform = scaling(0.5, 0.5, 0.5);

    w.objects = vec![s1, s2];
    w
}
