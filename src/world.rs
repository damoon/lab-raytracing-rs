use crate::intersections::{hit, Intersection};
use crate::lights::Pointlight;
use crate::rays::Ray;
use crate::shapes::Object;
use crate::tuples::Tuple;

pub struct World {
    pub objects: Vec<Object>,
    pub light: Option<Pointlight>,
}

impl World {
    pub fn default() -> Self {
        World {
            objects: Vec::new(),
            light: None,
        }
    }

    pub fn insersect(&self, r: &Ray) -> Vec<Intersection> {
        let mut v = Vec::new();
        for obj in self.objects.iter() {
            let mut intersections = obj.intersect(r);
            v.append(&mut intersections);
        }
        v.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        v
    }

    pub fn is_shadowed(&self, point: Tuple) -> bool {
        let v = self.light.clone().unwrap().position - point;
        let distance = v.magnitude();
        let direction = v.normalize();
        let r = Ray::new(point, direction);
        let intersections = self.insersect(&r);
        let h = hit(&intersections);
        match h {
            None => false,
            Some(i) => i.t < distance,
        }
    }
}
