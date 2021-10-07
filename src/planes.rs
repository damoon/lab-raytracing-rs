use std::{any::Any, fmt::Debug};

use crate::{
    materials::Material,
    matrices::identity_matrix,
    rays::Ray,
    shapes::{Object, Shape},
    tuples::{vector, Tuple},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Plane {}

impl Plane {
    pub fn default() -> Object {
        let shape = Box::new(Plane {});
        let transform = identity_matrix();
        let material = Material::default();
        Object::new(shape, transform, material)
    }
}

impl Shape for Plane {
    fn normal_at(&self, _local_point: &Tuple) -> Tuple {
        vector(0.0, 1.0, 0.0)
    }

    fn intersect(&self, ray: &Ray) -> Vec<f64> {
        let e = 0.0001;
        if ray.direction.y.abs() < e {
            return vec![];
        }

        let t = -ray.origin.y / ray.direction.y;
        vec![t]
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn equals(&self, other: &dyn Shape) -> bool {
        other
            .as_any()
            .downcast_ref::<Plane>()
            .map_or(false, |a| self == a)
    }

    fn fmt_boxed(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.fmt(f)
    }

    fn clone_boxed(&self) -> Box<dyn Shape> {
        Box::new(self.clone())
    }
}
