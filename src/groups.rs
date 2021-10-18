use crate::{
    intersections::Intersection,
    matrices::{identity_matrix, Matrix4x4},
    objects::Object,
    rays::Ray,
};
use std::sync::Arc;

#[derive(Clone)]
pub enum GroupMember {
    SubGroup(Arc<Group>),
    Object(Arc<Object>),
}

impl PartialEq for GroupMember {
    fn eq(&self, other: &GroupMember) -> bool {
        match (&self, &other) {
            (GroupMember::SubGroup(s), GroupMember::SubGroup(o)) => Arc::ptr_eq(s, o),
            (GroupMember::Object(s), GroupMember::Object(o)) => Arc::ptr_eq(s, o),
            _ => false,
        }
    }
}

impl GroupMember {
    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        match self {
            GroupMember::SubGroup(g) => g.local_intersect(ray),
            GroupMember::Object(o) => o
                .intersect(ray)
                .iter()
                .map(|t| Intersection {
                    t: *t,
                    object: o.clone(),
                })
                .collect(),
        }
    }

    // pub fn update_transform(&mut self, update: &Matrix4x4) {
    //     let new_transform = update * self.transform();
    //     match self {
    //         GroupMember::SubGroup(g) => g.set_transform(new_transform),
    //         GroupMember::Object(o) => o.set_transform(new_transform),
    //     }
    // }

    // pub fn transform(&self) -> &Matrix4x4 {
    //     match self {
    //         GroupMember::SubGroup(g) => g.transform(),
    //         GroupMember::Object(o) => o.transform(),
    //     }
    // }
}

#[derive(Clone)]
pub struct Group {
    transform: Matrix4x4,
    transform_inverse: Matrix4x4,
    elements: Vec<GroupMember>,
}

impl Group {
    pub fn default() -> Self {
        let transform = identity_matrix();
        let transform_inverse = transform.inverse().unwrap();
        let elements = Vec::new();
        Group {
            transform,
            transform_inverse,
            elements,
        }
    }

    pub fn add_group(&mut self, mut e: Group) {
        e.set_transform(e.transform() * &self.transform);
        let e = GroupMember::SubGroup(Arc::new(e));
        self.elements.push(e)
    }

    pub fn add_object(&mut self, mut e: Object) {
        e.set_transform(e.transform() * &self.transform);
        let e = GroupMember::Object(Arc::new(e));
        self.elements.push(e)
    }

    // pub fn contains_object(&self, o: Arc<Object>) -> bool {
    //     self.elements.contains(&GroupMember::Object(o))
    // }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn transform(&self) -> &Matrix4x4 {
        &self.transform
    }

    pub fn set_transform(&mut self, transform: Matrix4x4) {
        // let update = &transform * &self.transform;
        self.transform = transform;
        self.transform_inverse = self.transform.inverse().unwrap();

        if !self.is_empty() {
            panic!("changing a group after adding elements is not supported yet");
        }
        // let list: Vec<GroupMember> = self
        //     .elements
        //     .into_iter()
        //     .map(|e| {
        //         e.update_transform(&update);
        //         e
        //     })
        //     .collect();
        // self.elements = list;
    }

    pub fn local_intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let mut xs = Vec::new();
        for element in self.elements.iter() {
            let ls = &mut element.intersect(ray);
            xs.append(ls);
        }
        xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        xs
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        // dbg!(ray);
        let local_ray = ray.transform(&self.transform_inverse);
        // let local_ray = ray.transform(self.transform());
        // let local_ray = ray.clone();
        // dbg!(&local_ray);
        self.local_intersect(&local_ray)
    }
}
