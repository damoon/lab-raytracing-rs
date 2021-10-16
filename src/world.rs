use crate::intersections::Intersection;
use crate::lights::Pointlight;
use crate::rays::Ray;
use crate::shapes::{intersect, Object};
use crate::tuples::Tuple;
use std::sync::Arc;

pub struct World {
    pub objects: Vec<Arc<Object>>,
    pub light: Option<Pointlight>,
}

impl World {
    pub fn default() -> Self {
        World {
            objects: Vec::new(),
            light: None,
        }
    }

    pub fn add_object(&mut self, obj: Object) {
        self.objects.push(Arc::new(obj));
    }

    pub fn insersect(&self, r: &Ray) -> Vec<Intersection> {
        let mut v = Vec::with_capacity(self.objects.len());
        for obj in self.objects.iter() {
            let mut intersections = intersect(obj, r);
            v.append(&mut intersections);
        }
        v.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        v
    }

    pub fn is_shadowed(&self, point: Tuple, object: Option<&Arc<Object>>) -> bool {
        let v = &self.light.as_ref().unwrap().position - &point;
        let distance = v.magnitude();
        let direction = v.normalize();
        let r = Ray::new(point, direction);

        for i in self.insersect(&r).iter() {
            if i.t < 0.0 {
                continue;
            }
            if i.t > distance {
                continue;
            }
            match object {
                None => {}
                Some(object) => {
                    if object == &i.object && i.t.abs() < 1024.0 * f64::EPSILON {
                        continue;
                    }
                }
            }
            if i.object.throws_shaddow {
                return true;
            }
        }

        false
    }
}
