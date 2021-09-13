use cucumber_rust::{Steps};
use lab_raytracing_rs::{
    Tuple, point, vector, dot, cross
};

use crate::MyWorld;

pub fn steps() -> Steps<MyWorld> {
    let mut steps: Steps<MyWorld> = Steps::new();

    steps.given_regex(r#"(\w+) ← tuple\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#, |mut world, ctx| {
        let name = ctx.matches[1].clone();
        let x =  ctx.matches[2].parse::<f64>().unwrap();
        let y =  ctx.matches[3].parse::<f64>().unwrap();
        let z =  ctx.matches[4].parse::<f64>().unwrap();
        let w =  ctx.matches[5].parse::<f64>().unwrap();
        let t = Tuple::new(x, y, z, w);
        world.tuples.insert(name, t);
        world
    });

    steps.then_regex(r#"^(\w+).(\w+) = ([-0-9.]+)$"#, |world, ctx| {
        let name = ctx.matches[1].clone();
        let attr = ctx.matches[2].clone();
        let desired =  ctx.matches[3].parse::<f64>().unwrap();
        let tuple = world.tuples.get(&name).unwrap();
        let value = match attr.as_str() {
            "x" => tuple.x(),
            "y" => tuple.y(),
            "z" => tuple.z(),
            "w" => tuple.w(),
            _ => panic!("Invalid attribute checked"),
        };
        assert_eq!(value, desired);

        world
    });

    steps.then_regex(r#"^(\w) is a point$"#, |world, ctx| {
        let name = ctx.matches[1].clone();
        let tuple = world.tuples.get(&name).unwrap();
        assert_eq!(true, tuple.is_point());

        world
    });

    steps.then_regex(r#"^(\w+) is not a point$"#, |world, ctx| {
        let name = ctx.matches[1].clone();
        let tuple = world.tuples.get(&name).unwrap();
        assert_eq!(false, tuple.is_point());

        world
    });

    steps.then_regex(r#"^(\w+) is a vector$"#, |world, ctx| {
        let name = ctx.matches[1].clone();
        let tuple = world.tuples.get(&name).unwrap();
        assert_eq!(true, tuple.is_vector());

        world
    });

    steps.then_regex(r#"^(\w+) is not a vector$"#, |world, ctx| {
        let name = ctx.matches[1].clone();
        let tuple = world.tuples.get(&name).unwrap();
        assert_eq!(false, tuple.is_vector());

        world
    });

    steps.given_regex(r#"^(\w+) ← vector\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#, |mut world, ctx| {
        let name = ctx.matches[1].clone();
        let x =  ctx.matches[2].parse::<f64>().unwrap();
        let y =  ctx.matches[3].parse::<f64>().unwrap();
        let z =  ctx.matches[4].parse::<f64>().unwrap();
        let t = vector(x, y, z);
        world.tuples.insert(name, t);

        world
    });

    steps.given_regex(r#"^(\w+) ← point\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#, |mut world, ctx| {
        let name = ctx.matches[1].clone();
        let x =  ctx.matches[2].parse::<f64>().unwrap();
        let y =  ctx.matches[3].parse::<f64>().unwrap();
        let z =  ctx.matches[4].parse::<f64>().unwrap();
        let t = point(x, y, z);
        world.tuples.insert(name, t);

        world
    });

    steps.then_regex(r#"^(\w+) = tuple\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#, |world, ctx| {
        let name = ctx.matches[1].clone();
        let x =  ctx.matches[2].parse::<f64>().unwrap();
        let y =  ctx.matches[3].parse::<f64>().unwrap();
        let z =  ctx.matches[4].parse::<f64>().unwrap();
        let w =  ctx.matches[5].parse::<f64>().unwrap();
        let t = Tuple::new(x, y, z, w);
        let value = world.tuples.get(&name).unwrap();
        assert_eq!(value, &t);

        world
    });

    steps.then_regex(r#"^(\w+) \+ (\w+) = tuple\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#, |world, ctx| {
        let name1 = ctx.matches[1].clone();
        let name2 = ctx.matches[2].clone();
        let tuple1 = world.tuples.get(&name1).unwrap();
        let tuple2 = world.tuples.get(&name2).unwrap();
        let added_tuple = tuple1 + tuple2;
        let x = ctx.matches[3].parse::<f64>().unwrap();
        let y = ctx.matches[4].parse::<f64>().unwrap();
        let z = ctx.matches[5].parse::<f64>().unwrap();
        let w = ctx.matches[6].parse::<f64>().unwrap();
        let desired = Tuple::new(x, y, z, w);
        assert_eq!(added_tuple, desired);

        world
    });

    steps.then_regex(r#"^(\w+) \- (\w+) = vector\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#, |world, ctx| {
        let name1 = ctx.matches[1].clone();
        let name2 = ctx.matches[2].clone();
        let tuple1 = world.tuples.get(&name1).unwrap();
        let tuple2 = world.tuples.get(&name2).unwrap();
        let added_tuple = tuple1 - tuple2;
        let x = ctx.matches[3].parse::<f64>().unwrap();
        let y = ctx.matches[4].parse::<f64>().unwrap();
        let z = ctx.matches[5].parse::<f64>().unwrap();
        let desired = vector(x, y, z);
        assert_eq!(added_tuple, desired);

        world
    });

    steps.then_regex(r#"^(\w+) \- (\w+) = point\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#, |world, ctx| {
        let name1 = ctx.matches[1].clone();
        let name2 = ctx.matches[2].clone();
        let tuple1 = world.tuples.get(&name1).unwrap();
        let tuple2 = world.tuples.get(&name2).unwrap();
        let added_tuple = tuple1 - tuple2;
        let x = ctx.matches[3].parse::<f64>().unwrap();
        let y = ctx.matches[4].parse::<f64>().unwrap();
        let z = ctx.matches[5].parse::<f64>().unwrap();
        let desired = point(x, y, z);
        assert_eq!(added_tuple, desired);

        world
    });

    steps.then_regex(r#"^-(\w+) = tuple\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#, |world, ctx| {
        let name = ctx.matches[1].clone();
        let tuple = world.tuples.get(&name).unwrap();
        let x = ctx.matches[2].parse::<f64>().unwrap();
        let y = ctx.matches[3].parse::<f64>().unwrap();
        let z = ctx.matches[4].parse::<f64>().unwrap();
        let w = ctx.matches[5].parse::<f64>().unwrap();
        let desired = Tuple::new(x, y, z, w);
        assert_eq!(-tuple, desired);

        world
    });

    steps.then_regex(r#"^(\w+) \* ([-0-9.]+) = tuple\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#, |world, ctx| {
        let name = ctx.matches[1].clone();
        let tuple = world.tuples.get(&name).unwrap();
        let m = ctx.matches[2].parse::<f64>().unwrap();
        let x = ctx.matches[3].parse::<f64>().unwrap();
        let y = ctx.matches[4].parse::<f64>().unwrap();
        let z = ctx.matches[5].parse::<f64>().unwrap();
        let w = ctx.matches[6].parse::<f64>().unwrap();
        let desired = Tuple::new(x, y, z, w);
        assert_eq!(tuple * m, desired);

        world
    });

    steps.then_regex(r#"^(\w+) / ([-0-9.]+) = tuple\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#, |world, ctx| {
        let name = ctx.matches[1].clone();
        let tuple = world.tuples.get(&name).unwrap();
        let d = ctx.matches[2].parse::<f64>().unwrap();
        let x = ctx.matches[3].parse::<f64>().unwrap();
        let y = ctx.matches[4].parse::<f64>().unwrap();
        let z = ctx.matches[5].parse::<f64>().unwrap();
        let w = ctx.matches[6].parse::<f64>().unwrap();
        let desired = Tuple::new(x, y, z, w);
        assert_eq!(tuple / d, desired);

        world
    });

    steps.then_regex(r#"^magnitude\((\w+)\) = ([-0-9.]+)$"#, |world, ctx| {
        let name = ctx.matches[1].clone();
        let tuple = world.tuples.get(&name).unwrap();
        let d = ctx.matches[2].parse::<f64>().unwrap();
        assert_eq!(tuple.magnitude(), d);

        world
    });

    steps.then_regex(r#"^magnitude\((\w+)\) = √([-0-9.]+)$"#, |world, ctx| {
        let name = ctx.matches[1].clone();
        let tuple = world.tuples.get(&name).unwrap();
        let d = ctx.matches[2].parse::<f64>().unwrap();
        assert_eq!(tuple.magnitude(), d.sqrt());

        world
    });

    steps.then_regex(r#"^normalize\((\w+)\) = vector\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#, |world, ctx| {
        let name = ctx.matches[1].clone();
        let tuple = world.tuples.get(&name).unwrap();
        let x = ctx.matches[2].parse::<f64>().unwrap();
        let y = ctx.matches[3].parse::<f64>().unwrap();
        let z = ctx.matches[4].parse::<f64>().unwrap();
        let v = vector(x, y, z);
        assert_eq!(tuple.normalize(), v);

        world
    });

    steps.then_regex(r#"^normalize\((\w+)\) = approximately vector\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#, |world, ctx| {
        let name = ctx.matches[1].clone();
        let tuple = world.tuples.get(&name).unwrap();
        let x = ctx.matches[2].parse::<f64>().unwrap();
        let y = ctx.matches[3].parse::<f64>().unwrap();
        let z = ctx.matches[4].parse::<f64>().unwrap();
        let v = vector(x, y, z);
        assert_eq!(true, tuple.normalize().approximately(v));

        world
    });

    steps.when_regex(r#"^(\w+) ← normalize\((\w+)\)$"#, |mut world, ctx| {
        let name1 = ctx.matches[1].clone();
        let name2 = ctx.matches[2].clone();
        let tuple = world.tuples.get(&name2).unwrap().normalize();
        world.tuples.insert(name1, tuple);

        world
    });

    steps.then_regex(r#"^dot\((\w+), (\w+)\) = ([-0-9.]+)$"#, |world, ctx| {
        let name1 = ctx.matches[1].clone();
        let name2 = ctx.matches[2].clone();
        let tuple1 = world.tuples.get(&name1).unwrap().clone();
        let tuple2 = world.tuples.get(&name2).unwrap().clone();
        let dot = dot(tuple1, tuple2);
        let desired = ctx.matches[3].parse::<f64>().unwrap();
        assert_eq!(dot, desired);

        world
    });

    steps.then_regex(r#"^cross\((\w+), (\w+)\) = vector\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#, |world, ctx| {
        let name1 = ctx.matches[1].clone();
        let name2 = ctx.matches[2].clone();
        let tuple1 = world.tuples.get(&name1).unwrap().clone();
        let tuple2 = world.tuples.get(&name2).unwrap().clone();
        let cross = cross(tuple1, tuple2);
        let x = ctx.matches[3].parse::<f64>().unwrap();
        let y = ctx.matches[4].parse::<f64>().unwrap();
        let z = ctx.matches[5].parse::<f64>().unwrap();
        let v = vector(x, y, z);
        assert_eq!(cross, v);

        world
    });

    steps
}
