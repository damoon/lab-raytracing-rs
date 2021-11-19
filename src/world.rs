use crate::groups::{Group, GroupMember};
use crate::intersections::Intersection;
use crate::lights::Pointlight;
use crate::objects::Object;
use crate::rays::Ray;
use crate::tuples::Tuple;
use std::sync::Arc;

#[derive(Debug)]
pub struct World {
    pub objects: Vec<GroupMember>,
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
        self.objects.push(GroupMember::Object(Arc::new(obj)));
    }

    pub fn add_group(&mut self, obj: Group) {
        self.objects.push(GroupMember::SubGroup(Arc::new(obj)));
    }

    pub fn insersect(&self, ray: &Ray) -> Vec<Intersection> {
        let mut v = Vec::with_capacity(self.objects.len());
        for obj in self.objects.iter() {
            let mut intersections = obj.intersect(ray);
            v.append(&mut intersections);
        }
        v.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        v
    }

    pub fn is_shadowed(&self, point: Tuple) -> bool {
        let v = &self.light.as_ref().unwrap().position - &point;
        let distance = v.magnitude();
        let direction = v.normalize();
        let r = Ray::new(point, direction);

        for i in self.insersect(&r).iter() {
            if i.t < 0.0001 {
                continue;
            }
            if i.t > distance {
                continue;
            }
            if i.object.throws_shaddow {
                return true;
            }
        }

        false
    }
}
