use cucumber_rust::{Steps};
use lab_raytracing_rs::{
    Tuple, point, vector
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

    steps
}