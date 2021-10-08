use std::rc::Rc;

use crate::{
    lights::lighting,
    rays::Ray,
    shapes::Object,
    tuples::{color, dot, Tuple},
    world::World,
};

// #[derive(PartialEq)]
#[derive(Debug, PartialEq, Clone)]
pub struct Intersection {
    pub t: f64,
    pub object: Rc<Object>,
}

pub fn hit(xs: Vec<Intersection>) -> Option<Intersection> {
    let mut r = None;
    for current in xs.into_iter() {
        if current.t < 0.0 {
            continue;
        }
        r = match r {
            None => Some(current),
            Some(previous) => {
                if current.t < previous.t {
                    Some(current)
                } else {
                    Some(previous)
                }
            }
        }
    }
    r
}

pub struct IntersectionPrecomputations {
    pub t: f64,
    pub object: Rc<Object>,
    pub point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool,
    pub over_point: Tuple,
}

pub fn prepare_computations(intersection: Intersection, ray: &Ray) -> IntersectionPrecomputations {
    let t = intersection.t;
    let object = intersection.object;
    let point = ray.position(t);
    let eyev = -&ray.direction;
    let mut normalv = object.normal_at(&point);
    let mut inside = false;
    if dot(&normalv, &eyev) < 0.0 {
        inside = true;
        normalv = -normalv;
    }
    let e = 0.0001;
    let over_point = &point + &normalv * e;
    IntersectionPrecomputations {
        t,
        object,
        point,
        eyev,
        normalv,
        inside,
        over_point,
    }
}

pub fn shade_hit(world: &World, comps: &IntersectionPrecomputations) -> Tuple {
    let in_shadow = world.is_shadowed(comps.over_point.clone());
    lighting(
        &comps.object.material,
        &comps.object,
        world.light.as_ref().unwrap(),
        &comps.over_point,
        &comps.eyev,
        &comps.normalv,
        in_shadow,
    )
}

pub fn color_at(world: &World, ray: &Ray) -> Tuple {
    let intersections = world.insersect(ray);
    let hit = hit(intersections);
    match hit {
        None => color(0.0, 0.0, 0.0),
        Some(intersection) => {
            let precomputations = prepare_computations(intersection, ray);
            shade_hit(world, &precomputations)
        }
    }
}
