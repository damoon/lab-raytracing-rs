use crate::spheres::Sphere;

#[derive(Debug, Clone, PartialEq)]
pub struct Intersection {
    pub t: f64,
    pub object: Sphere,
}

pub fn hit(xs: Vec<Intersection>) -> Option<Intersection> {
    match xs.first() {
        None => None,
        Some(_) => {
            let mut intersection = None;
            for current in xs.into_iter() {
                if current.t < 0.0 {
                    continue;
                }
                intersection = match intersection {
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
            intersection
        }
    }
}
