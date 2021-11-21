use std::sync::Arc;

use crate::{
    groups::{Group, GroupMember, AABB},
    intersections::Intersection,
    materials::Material,
    matrices::Matrix4x4,
    objects::Object,
    rays::Ray,
};

#[derive(Clone, Debug, PartialEq)]
pub enum CSG {
    Union(GroupMember, GroupMember),
    Intersection(GroupMember, GroupMember),
    Difference(GroupMember, GroupMember),
}

impl CSG {
    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let (l, r) = match self {
            CSG::Union(l, r) => (l, r),
            CSG::Intersection(l, r) => (l, r),
            CSG::Difference(l, r) => (l, r),
        };

        let mut leftxs = l.intersect(ray);
        let mut rightxs = r.intersect(ray);
        let mut xs = Vec::new();
        xs.append(&mut leftxs);
        xs.append(&mut rightxs);

        xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

        self.filter_intersections(&xs)
    }

    pub fn intersection_allowed(&self, lhit: bool, inl: bool, inr: bool) -> bool {
        match self {
            CSG::Union(_, _) => (lhit && !inr) || (!lhit && !inl),
            CSG::Intersection(_, _) => (lhit && inr) || (!lhit && inl),
            CSG::Difference(_, _) => (lhit && !inr) || (!lhit && inl),
        }
    }

    pub fn objects(&self) -> Vec<Arc<Object>> {
        match self {
            CSG::Union(l, r) => {
                let v = Vec::new();
                l.objects().append(&mut l.objects());
                l.objects().append(&mut r.objects());
                v
            }
            CSG::Intersection(l, r) => {
                let v = Vec::new();
                l.objects().append(&mut l.objects());
                l.objects().append(&mut r.objects());
                v
            }
            CSG::Difference(l, r) => {
                let v = Vec::new();
                l.objects().append(&mut l.objects());
                l.objects().append(&mut r.objects());
                v
            }
        }
    }

    pub fn bounds(&self) -> Option<AABB> {
        match self {
            CSG::Union(l, r) => Group::outer_bounds(&l.bounds(), &r.bounds()),
            CSG::Intersection(l, r) => Group::outer_bounds(&l.bounds(), &r.bounds()),
            CSG::Difference(l, r) => Group::outer_bounds(&l.bounds(), &r.bounds()),
        }
    }

    pub fn set_material(&mut self, m: &Material) {
        match self {
            CSG::Union(l, r) => {
                l.set_material(m);
                r.set_material(m);
            }
            CSG::Intersection(l, r) => {
                l.set_material(m);
                r.set_material(m);
            }
            CSG::Difference(l, r) => {
                l.set_material(m);
                r.set_material(m);
            }
        }
    }

    pub fn update_transform(&self, update: &Matrix4x4) -> Self {
        match self {
            CSG::Union(l, r) => CSG::Union(l.update_transform(update), r.update_transform(update)),
            CSG::Intersection(l, r) => {
                CSG::Intersection(l.update_transform(update), r.update_transform(update))
            }
            CSG::Difference(l, r) => {
                CSG::Difference(l.update_transform(update), r.update_transform(update))
            }
        }
    }

    pub fn includes(&self, obj: &Arc<Object>) -> bool {
        match self {
            CSG::Union(l, r) => l.includes(obj) || r.includes(obj),
            CSG::Intersection(l, r) => l.includes(obj) || r.includes(obj),
            CSG::Difference(l, r) => l.includes(obj) || r.includes(obj),
        }
    }

    pub fn left(&self) -> &GroupMember {
        match self {
            CSG::Union(left, _) => left,
            CSG::Intersection(left, _) => left,
            CSG::Difference(left, _) => left,
        }
    }

    pub fn filter_intersections(&self, xs: &[Intersection]) -> Vec<Intersection> {
        // begin outside of both children
        let mut inl = false;
        let mut inr = false;

        // prepare a list to receive the filtered intersections
        let mut result = Vec::new();

        for i in xs {
            // if i.object is part of the "left" child, then lhit is true
            let lhit = self.left().includes(&i.object);

            if self.intersection_allowed(lhit, inl, inr) {
                result.push(i.clone())
            }

            // depending on which object was hit, toggle either inl or inr
            if lhit {
                inl = !inl
            } else {
                inr = !inr
            }
        }

        result
    }
}
