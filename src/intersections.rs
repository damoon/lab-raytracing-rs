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

pub fn hit(xs: &[Intersection]) -> Option<Intersection> {
    let mut r = None;
    for current in xs.iter() {
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
    r.cloned()
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
    pub under_point: Tuple,
    pub n1: f64,
    pub n2: f64,
}

pub fn prepare_computations(
    intersection: Intersection,
    ray: &Ray,
    xs: &[Intersection],
) -> IntersectionPrecomputations {
    let mut containers: Vec<Rc<Object>> = Vec::new();
    let mut n1 = 0.0;
    let mut n2 = 0.0;
    for i in xs.iter() {
        if i == &intersection {
            if containers.is_empty() {
                n1 = 1.0;
            } else {
                n1 = containers.last().unwrap().material.refractive_index;
            }
        }

        if containers.contains(&i.object) {
            let index = containers.iter().position(|x| x == &i.object).unwrap();
            containers.remove(index);
        } else {
            containers.push(i.object.clone());
        }

        if i == &intersection {
            if containers.is_empty() {
                n2 = 1.0;
            } else {
                n2 = containers.last().unwrap().material.refractive_index
            }
        }
    }

    let t = intersection.t;
    let object = intersection.object;
    let point = ray.position(t);
    let eyev = -&ray.direction;
    let mut normalv = (&object.normal_at(&point)).clone();
    let mut inside = false;
    if dot(&normalv, &eyev) < 0.0 {
        inside = true;
        normalv = -normalv;
    }
    let reflectv = reflect(&ray.direction, &normalv);
    let e = 0.0001;
    let over_point = &point + (&normalv * e);
    let under_point = &point - (&normalv * e);

    IntersectionPrecomputations {
        t,
        object,
        point,
        eyev,
        normalv,
        reflectv,
        inside,
        over_point,
        under_point,
        n1,
        n2,
    }
}

pub fn color_at(world: &World, ray: &Ray, remaining: usize) -> Tuple {
    let intersections = world.insersect(ray);
    let hit = hit(&intersections);
    match hit {
        None => color(0.0, 0.0, 0.0),
        Some(intersection) => {
            let precomputations = prepare_computations(intersection, ray, &intersections);
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
    let refracted = refracted_color(world, comps, remaining);

    let material = &comps.object.material;
    if material.reflective > 0.0 && material.transparency > 0.0 {
        let reflectance = schlick(comps);
        return surface + reflected * reflectance + refracted * (1.0 - reflectance);
    }

    surface + reflected + refracted
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

pub fn refracted_color(
    world: &World,
    comps: &IntersectionPrecomputations,
    remaining: usize,
) -> Tuple {
    if comps.object.material.transparency == 0.0 {
        return color(0.0, 0.0, 0.0);
    }
    if remaining == 0 {
        return color(0.0, 0.0, 0.0);
    }

    // Find the ratio of first index of refraction to the second.
    // (Yup, this is inverted from the definition of Snell's Law.)
    let n_ratio = comps.n1 / comps.n2;
    // cos(theta_i) is the same as the dot product of the two vectors
    let cos_i = dot(&comps.eyev, &comps.normalv);
    // Find sin(theta_t)^2 via trigonometric identity
    let sin2_t = (n_ratio * n_ratio) * (1.0 - (cos_i * cos_i));
    if sin2_t > 1.0 {
        // total internal reflection
        return color(0.0, 0.0, 0.0);
    }

    // Find cos(theta_t) via trigonometric identity
    let cos_t = (1.0 - sin2_t).sqrt();
    // Compute the direction of the refracted ray
    let direction = &comps.normalv * (n_ratio * cos_i - cos_t) - &comps.eyev * n_ratio;
    // Create the refracted ray
    let refract_ray = Ray::new(comps.under_point.clone(), direction);
    // Find the color of the refracted ray, making sure to multiply
    // by the transparency value to account for any opacity
    color_at(world, &refract_ray, remaining - 1) * comps.object.material.transparency
}

pub fn schlick(comps: &IntersectionPrecomputations) -> f64 {
    // find the cosine of the angle between the eye and normal vectors
    let mut cos = dot(&comps.eyev, &comps.normalv);

    // total internal reflection can only occur if n1 > n2
    if comps.n1 > comps.n2 {
        let n = comps.n1 / comps.n2;
        let sin2_t = f64::powf(n, 2.0) * (1.0 - (cos * cos));
        if sin2_t > 1.0 {
            return 1.0;
        }

        // compute cosine of theta_t using trig identity
        let cos_t = (1.0 - sin2_t).sqrt();
        // when n1 > n2, use cos(theta_t) instead
        cos = cos_t
    }

    let r0 = f64::powf((comps.n1 - comps.n2) / (comps.n1 + comps.n2), 2.0);
    r0 + (1.0 - r0) * f64::powf(1.0 - cos, 5.0)
}
