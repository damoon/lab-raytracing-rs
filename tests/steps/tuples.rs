use approx::assert_abs_diff_eq;
use cucumber_rust::Steps;
use lab_raytracing_rs::tuples::{color, cross, dot, point, reflect, vector, Tuple};

use crate::MyWorld;

pub fn parse_float(s: &str) -> f64 {
    match s {
        "√2" => 2.0_f64.sqrt(),
        "-√2" => -2.0_f64.sqrt(),
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

pub fn steps() -> Steps<MyWorld> {
    let mut steps: Steps<MyWorld> = Steps::new();

    steps.given_regex(
        r#"(a|a1|a2|n|b) ← tuple\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |mut world, ctx| {
            let tuple = parse_tuple(&ctx.matches[2..=5]);
            world.tuples.insert(ctx.matches[1].clone(), tuple);
            world
        },
    );

    steps.given_regex(
        r#"(position) ← point\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |mut world, ctx| {
            let tuple = parse_point(&ctx.matches[2..=4]);
            world.tuples.insert(ctx.matches[1].clone(), tuple);
            world
        },
    );

    steps.then_regex(
        r#"^(a|c).(x|y|z|w|red|green|blue) = ([-0-9.]+)$"#,
        |world, ctx| {
            let desired = ctx.matches[3].parse::<f64>().unwrap();
            let tuple = world.tuples.get(&ctx.matches[1]).unwrap();
            let value = match ctx.matches[2].as_str() {
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

            world
        },
    );

    steps.then_regex(r#"^(a) is (not )?a (point|vector)$"#, |world, ctx| {
        let tuple = world.tuples.get(&ctx.matches[1]).unwrap();
        assert!(match (ctx.matches[2].as_str(), ctx.matches[3].as_str()) {
            ("", "point") => tuple.is_point(),
            ("not ", "point") => !tuple.is_point(),
            ("", "vector") => tuple.is_vector(),
            ("not ", "vector") => !tuple.is_vector(),
            (_, _) => false,
        });
        world
    });

    steps.given_regex(
        r#"^(a|b|p|v|p1|p2|v1|v2|zero|c|c1|c2|c3|n|red|from|to|up|origin|direction|intensity|eyev|normalv|black|white) ← (point|vector|color)\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |mut world, ctx| {
            let tuple = match ctx.matches[2].as_str() {
                "point" => parse_point(&ctx.matches[3..=5]),
                "vector" => parse_vector(&ctx.matches[3..=5]),
                "color" => parse_color(&ctx.matches[3..=5]),
                _ => panic!("type not covered"),
            };
            world.tuples.insert(ctx.matches[1].clone(), tuple);
            world
        },
    );

    steps.given_regex(
        r#"^(n|eyev) ← vector\(([-0-9.]+|-?√2/2), ([-0-9.]+|-?√2/2), ([-0-9.]+|-?√2/2)\)$"#,
        |mut world, ctx| {
            let vector = parse_vector(&ctx.matches[2..=4]);
            world.tuples.insert(ctx.matches[1].clone(), vector);
            world
        },
    );

    steps.when("r ← reflect(v, n)", |mut world, _ctx| {
        let vector = world.tuples.get("v").unwrap();
        let normal = world.tuples.get("n").unwrap();
        let reflected = reflect(vector, normal);
        world.tuples.insert("r".to_string(), reflected);
        world
    });

    steps.then_regex(
        r#"^(-?)(p|v|a) = tuple\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |world, ctx| {
            let desired_tuple = parse_tuple(&ctx.matches[3..=6]);
            let mut tuple = world.tuples.get(&ctx.matches[2]).unwrap().clone();
            if &ctx.matches[1] == "-" {
                tuple = -tuple;
            }
            eq_tuples_similar(&desired_tuple, &tuple);
            world
        },
    );

    steps.then_regex(
        r#"^(a1) \+ (a2) = tuple\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |world, ctx| {
            let desired_tuple = parse_tuple(&ctx.matches[3..=6]);
            let tuple1 = world.tuples.get(&ctx.matches[1]).unwrap();
            let tuple2 = world.tuples.get(&ctx.matches[2]).unwrap();
            let computed_tuple = tuple1 + tuple2;
            eq_tuples_similar(&computed_tuple, &desired_tuple);
            world
        },
    );

    steps.then_regex(
        r#"^(p2|p3|p4) = point\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |world, ctx| {
            let point = world.tuples.get(&ctx.matches[1]).unwrap().clone();
            let desired_color = parse_color(&ctx.matches[2..=4]);
            eq_tuples_similar(&point, &desired_color);
            world
        },
    );

    steps.then_regex(
        r#"^(n|r|n1|n2|n3|normal) = vector\(([-0-9.]+|\-?√2/2), ([-0-9.|\-?√2/2]+), ([-0-9.]+|\-?√2/2)\)$"#,
        |world, ctx| {
            let tuple = world.tuples.get(&ctx.matches[1]).unwrap().clone();
            let desired_vector = parse_vector(&ctx.matches[2..=4]);
            eq_tuples_similar(&tuple, &desired_vector);
            world
        },
    );

    steps.then_regex(
        r#"^(n) = vector\(√3/3, √3/3, √3/3\)$"#,
        |world, ctx| {
            let tuple = world.tuples.get(&ctx.matches[1]).unwrap().clone();
            let desired_vector = vector(
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
            );
            eq_tuples_similar(&tuple, &desired_vector);
            world
        },
    );

    steps.then_regex(
        r#"^(c1) \+ (c2) = color\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |world, ctx| {
            let desired_color = parse_color(&ctx.matches[3..=5]);
            let color1 = world.tuples.get(&ctx.matches[1]).unwrap();
            let color2 = world.tuples.get(&ctx.matches[2]).unwrap();
            let computed_color = color1 + color2;
            eq_tuples_similar(&computed_color, &desired_color);
            world
        },
    );

    steps.then_regex(
        r#"^(c|c1|c2|color) = color\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |world, ctx| {
            let desired_color = parse_color(&ctx.matches[2..=4]);
            let color = world.tuples.get(&ctx.matches[1]).unwrap();
            eq_tuples_similar(color, &desired_color);
            world
        },
    );

    steps.then_regex(r#"^(c) = (white)$"#, |world, ctx| {
        let lookup = world.tuples.get(&ctx.matches[1]).unwrap();
        let desired = world.tuples.get(&ctx.matches[2]).unwrap();
        eq_tuples_similar(lookup, desired);
        world
    });

    steps.then_regex(
        r#"^(p|p1|v1|zero|c1) \- (v|p2|v2|c2) = (vector|point|color)\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |world, ctx| {
            let tuple1 = world.tuples.get(&ctx.matches[1]).unwrap();
            let tuple2 = world.tuples.get(&ctx.matches[2]).unwrap();
            let vector = tuple1 - tuple2;
            let desired_tuple = match ctx.matches[3].as_str() {
                "point" => parse_point(&ctx.matches[4..=6]),
                "vector" => parse_vector(&ctx.matches[4..=6]),
                "color" => parse_color(&ctx.matches[4..=6]),
                _ => panic!("type not covered"),
            };
            eq_tuples_similar(&desired_tuple, &vector);
            world
        },
    );

    steps.then_regex(
        r#"^(a) (\*|/) ([-0-9.]+) = tuple\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |world, ctx| {
            let tuple = world.tuples.get(&ctx.matches[1]).unwrap();
            let multiplicator = ctx.matches[3].parse::<f64>().unwrap();
            let calculated = match ctx.matches[2].as_str() {
                "*" => tuple * multiplicator,
                "/" => tuple / multiplicator,
                _ => panic!("operator not covered"),
            };
            let desired_tuple = parse_tuple(&ctx.matches[4..=7]);
            eq_tuples_similar(&calculated, &desired_tuple);
            world
        },
    );

    steps.then_regex(
        r#"^(c) \* ([-0-9.]+) = color\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |world, ctx| {
            let tuple = world.tuples.get(&ctx.matches[1]).unwrap();
            let multiplicator = ctx.matches[2].parse::<f64>().unwrap();
            let calculated = tuple * multiplicator;
            let desired_color = parse_color(&ctx.matches[3..=5]);
            eq_tuples_similar(&calculated, &desired_color);
            world
        },
    );

    steps.then_regex(
        r#"^(c1) \* (c2) = color\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |world, ctx| {
            let color1 = world.tuples.get(&ctx.matches[1]).unwrap();
            let color2 = world.tuples.get(&ctx.matches[2]).unwrap();
            let calculated = color1 * color2;
            let desired_color = parse_color(&ctx.matches[3..=5]);
            eq_tuples_similar(&calculated, &desired_color);
            world
        },
    );

    steps.then_regex(
        r#"^(result) = color\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |world, ctx| {
            let color = world.tuples.get(&ctx.matches[1]).unwrap();
            let desired_color = parse_color(&ctx.matches[2..=4]);
            eq_tuples_similar(color, &desired_color);
            world
        },
    );

    steps.then_regex(
        r#"^magnitude\((v|norm)\) = (√14|[-0-9.]+)$"#,
        |world, ctx| {
            let calculated = world.tuples.get(&ctx.matches[1]).unwrap().magnitude();
            let desired = match ctx.matches[2].as_str() {
                "√14" => 14.0_f64.sqrt(),
                a => a.parse::<f64>().unwrap(),
            };
            assert_abs_diff_eq!(calculated, desired);
            world
        },
    );

    steps.then_regex(
        r#"^normalize\((v)\) = (approximately )?vector\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |world, ctx| {
            let calculated = world.tuples.get(&ctx.matches[1]).unwrap().normalize();
            let desired = parse_vector(&ctx.matches[3..=5]);
            if ctx.matches[2] == "approximately " {
                eq_tuples_similar(&desired, &calculated);
            } else {
                assert_eq!(desired, calculated);
            }
            world
        },
    );

    steps.given_regex(r#"^(direction) ← normalize\(vector\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)\)$"#, |mut world, ctx| {
        let normalized = parse_vector(&ctx.matches[2..=4]).normalize();
        world.tuples.insert(ctx.matches[1].clone(), normalized);
        world
    });

    steps.when_regex(r#"^(norm) ← normalize\((v)\)$"#, |mut world, ctx| {
        let normalized = world.tuples.get(&ctx.matches[2]).unwrap().normalize();
        world.tuples.insert(ctx.matches[1].clone(), normalized);
        world
    });

    steps.then_regex(r#"^(n) = normalize\((n)\)$"#, |world, ctx| {
        let desired = world.tuples.get(&ctx.matches[1]).unwrap();
        let vector = world.tuples.get(&ctx.matches[2]).unwrap().normalize();
        assert_eq!(desired, &vector);
        world
    });

    steps.then_regex(r#"^dot\((a), (b)\) = ([-0-9.]+)$"#, |world, ctx| {
        let tuple1 = world.tuples.get(&ctx.matches[1]).unwrap();
        let tuple2 = world.tuples.get(&ctx.matches[2]).unwrap();
        let desired = ctx.matches[3].parse::<f64>().unwrap();
        let dot = dot(tuple1, tuple2);
        assert_abs_diff_eq!(dot, desired);
        world
    });

    steps.then_regex(
        r#"^cross\((a|b), (a|b)\) = vector\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |world, ctx| {
            let tuple1 = world.tuples.get(&ctx.matches[1]).unwrap();
            let tuple2 = world.tuples.get(&ctx.matches[2]).unwrap();
            let cross = cross(tuple1, tuple2);
            let desired = parse_vector(&ctx.matches[3..=5]);
            assert_eq!(cross, desired);

            world
        },
    );

    steps
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
