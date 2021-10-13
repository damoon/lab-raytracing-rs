use std::rc::Rc;

use crate::{
    lights::lighting,
    rays::Ray,
    shapes::Object,
    tuples::{color, dot, reflect, Tuple},
    world::World,
};

#[derive(Debug, Clone)]
pub struct Intersection {
    pub t: f64,
    pub object: Rc<Object>,
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Intersection) -> bool {
        self.t.eq(&other.t) && Rc::ptr_eq(&self.object, &other.object)
    }
}

pub fn hit(xs: &[Intersection], object: &Rc<Object>) -> Option<Intersection> {
    let mut r = None;
    for current in xs.iter() {
        if current.t < 0.0 {
            continue;
        }
        if Rc::ptr_eq(object, &current.object) && current.t < 1024.0 * f64::EPSILON {
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
    pub n1: f64,
    pub n2: f64,
}

pub fn prepare_computations(
    intersection: &Intersection,
    ray: &Ray,
    xs: &[Intersection],
) -> IntersectionPrecomputations {
    let mut containers: Vec<Rc<Object>> = Vec::with_capacity(xs.len());
    let mut n1 = 0.0;
    let mut n2 = 0.0;
    for i in xs.iter() {
        if i == intersection {
            if containers.is_empty() {
                n1 = 1.0;
            } else {
                n1 = containers.last().unwrap().material.refractive_index;
            }
        }

        match containers.iter().position(|x| Rc::ptr_eq(x, &i.object)) {
            Some(index) => {
                containers.remove(index);
            }
            None => {
                containers.push(i.object.clone());
            }
        }

        if i == intersection {
            if containers.is_empty() {
                n2 = 1.0;
            } else {
                n2 = containers.last().unwrap().material.refractive_index
            }
        }
    }

    let t = intersection.t;
    let object = &intersection.object;
    let point = ray.position(t);
    let eyev = -&ray.direction;
    let mut normalv = object.normal_at(&point);
    let mut inside = false;
    if dot(&normalv, &eyev) < 0.0 {
        inside = true;
        normalv = -normalv;
    }
    let reflectv = reflect(&ray.direction, &normalv);

    IntersectionPrecomputations {
        t,
        object: object.clone(),
        point,
        eyev,
        normalv,
        reflectv,
        inside,
        n1,
        n2,
    }
}

pub fn color_at(world: &World, ray: &Ray, remaining: usize, object: &Rc<Object>) -> Tuple {
    let intersections = world.insersect(ray);
    let hit = hit(&intersections, object);
    match hit {
        None => color(0.0, 0.0, 0.0),
        Some(intersection) => {
            let precomputations = prepare_computations(&intersection, ray, &intersections);
            shade_hit(world, &precomputations, remaining)
        }
    }
}

pub fn shade_hit(world: &World, comps: &IntersectionPrecomputations, remaining: usize) -> Tuple {
    let in_shadow = world.is_shadowed(comps.point.clone(), &comps.object);
    let surface = lighting(
        &comps.object.material,
        &comps.object,
        world.light.as_ref().unwrap(),
        &comps.point,
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
    let reflect_ray = Ray::new(comps.point.clone(), comps.reflectv.clone());
    let color = color_at(world, &reflect_ray, remaining - 1, &comps.object);
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
    let refract_ray = Ray::new(comps.point.clone(), direction);
    // Find the color of the refracted ray, making sure to multiply
    // by the transparency value to account for any opacity
    color_at(world, &refract_ray, remaining - 1, &comps.object) * comps.object.material.transparency
}

pub fn schlick(comps: &IntersectionPrecomputations) -> f64 {
    // find the cosine of the angle between the eye and normal vectors
    let mut cos = dot(&comps.eyev, &comps.normalv);

    // total internal reflection can only occur if n1 > n2
    if comps.n1 > comps.n2 {
        let n = comps.n1 / comps.n2;
        let sin2_t = n.powi(2) * (1.0 - (cos * cos));
        if sin2_t > 1.0 {
            return 1.0;
        }

        // compute cosine of theta_t using trig identity
        let cos_t = (1.0 - sin2_t).sqrt();
        // when n1 > n2, use cos(theta_t) instead
        cos = cos_t
    }

    let r0 = ((comps.n1 - comps.n2) / (comps.n1 + comps.n2)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cos).powi(5)
}
