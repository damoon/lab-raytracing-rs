use crate::intersections::Intersection;
use crate::lights::Pointlight;
use crate::rays::Ray;
use crate::spheres::Sphere;

#[derive(Debug, Clone)]
pub struct World {
    pub objects: Vec<Sphere>,
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
        let mut v: Vec<Intersection> = Vec::new();
        for obj in self.objects.iter() {
            let mut intersections = obj.intersect(r);
            v.append(&mut intersections);
        }
        v.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        v
    }
}
