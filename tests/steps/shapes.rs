use crate::{
    steps::tuples::{parse_point, parse_vector},
    MyWorld,
};
use cucumber_rust::Steps;
use lab_raytracing_rs::{
    materials::Material,
    matrices::identity_matrix,
    rays::Ray,
    shapes::{Object, Shape},
    tuples::{point, vector, Tuple},
};
use std::{
    any::Any,
    fmt::Debug,
    sync::{Arc, RwLock},
};

pub fn steps() -> Steps<MyWorld> {
    let mut steps: Steps<MyWorld> = Steps::new();

    steps.given_regex(r#"^(s) ← test_shape\(\)$"#, |mut world, ctx| {
        world
            .shapes
            .insert(ctx.matches[1].clone(), TestShape::default());
        world
    });

    steps.then_regex(
        r#"^s.saved_ray.(origin|direction) = (point|vector)\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |world, ctx| {
            let desired = match ctx.matches[2].as_str() {
                "point" => parse_point(&ctx.matches[3..=5]),
                "vector" => parse_vector(&ctx.matches[3..=5]),
                _ => panic!("desired kind not covered"),
            };
            let ray = SAVED_RAY.with(|c| c.read().unwrap().clone());
            let lookup = match ctx.matches[1].as_str() {
                "origin" => ray.origin,
                "direction" => ray.direction,
                _ => panic!("lookup attribute not covered"),
            };
            assert_eq!(lookup, desired);
            world
        },
    );

    steps
}

#[derive(Debug, Clone, PartialEq)]
pub struct TestShape {
    saved_ray: Option<Ray>,
}

impl TestShape {
    pub fn default() -> Object {
        let shape = Box::new(TestShape { saved_ray: None });
        let transform = identity_matrix();
        let material = Material::default();
        Object::new(shape, transform, material)
    }
}

thread_local! {
    static SAVED_RAY: RwLock<Arc<Ray>> = RwLock::new(Arc::new(Ray::new(point(0.0,0.0,0.0), vector(1.0, 1.0, 1.0).normalize())));
}

impl Shape for TestShape {
    fn normal_at(&self, local_point: &Tuple) -> Tuple {
        local_point - point(0.0, 0.0, 0.0)
    }

    fn intersect(&self, ray: &Ray) -> Vec<f64> {
        SAVED_RAY.with(|c| *c.write().unwrap() = Arc::new(ray.clone()));
        vec![]
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn equals(&self, other: &dyn Shape) -> bool {
        other
            .as_any()
            .downcast_ref::<TestShape>()
            .map_or(false, |a| self == a)
    }

    fn fmt_boxed(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.fmt(f)
    }

    fn clone_boxed(&self) -> Box<dyn Shape> {
        Box::new(self.clone())
    }
}
