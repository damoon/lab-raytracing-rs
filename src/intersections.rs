use std::rc::Rc;

use crate::{
    lights::lighting,
    rays::Ray,
    shapes::Object,
    tuples::{color, dot, reflect, Tuple},
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
    pub reflectv: Tuple,
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
    let reflectv = reflect(&ray.direction, &normalv);
    let e = 0.0001;
    let over_point = &point + &normalv * e;
    IntersectionPrecomputations {
        t,
        object,
        point,
        eyev,
        normalv,
        reflectv,
        inside,
        over_point,
    }
}

pub fn color_at(world: &World, ray: &Ray, remaining: usize) -> Tuple {
    let intersections = world.insersect(ray);
    let hit = hit(intersections);
    match hit {
        None => color(0.0, 0.0, 0.0),
        Some(intersection) => {
            let precomputations = prepare_computations(intersection, ray);
            shade_hit(world, &precomputations, remaining)
        }
    }
}

pub fn shade_hit(world: &World, comps: &IntersectionPrecomputations, remaining: usize) -> Tuple {
    let in_shadow = world.is_shadowed(comps.over_point.clone());
    let surface = lighting(
        &comps.object.material,
        &comps.object,
        world.light.as_ref().unwrap(),
        &comps.over_point,
        &comps.eyev,
        &comps.normalv,
        in_shadow,
    );
    let reflected = reflected_color(world, comps, remaining);
    // if (comps.object.material.reflective - 1.0).abs() < f64::EPSILON {
    //     return reflected;
    // }
    surface + reflected
}

pub fn reflected_color(
    world: &World,
    comps: &IntersectionPrecomputations,
    remaining: usize,
) -> Tuple {
    if comps.object.material.reflective == 0.0 {
        return color(0.0, 0.0, 0.0);
    }
    if remaining == 0 {
        return color(0.0, 0.0, 0.0);
    }
    let reflect_ray = Ray::new(comps.over_point.clone(), comps.reflectv.clone());
    let color = color_at(world, &reflect_ray, remaining - 1);
    color * comps.object.material.reflective
}
