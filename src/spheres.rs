use std::{any::Any, fmt::Debug};

use crate::{
    materials::Material,
    matrices::identity_matrix,
    rays::Ray,
    shapes::{Object, Shape},
    tuples::{dot, point, Tuple},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Sphere {}

impl Sphere {
    pub fn default() -> Object {
        let shape = Box::new(Sphere {});
        let transform = identity_matrix();
        let material = Material::default();
        Object {
            shape,
            transform,
            material,
        }
    }
}

impl Shape for Sphere {
    fn normal_at(&self, local_point: &Tuple) -> Tuple {
        local_point - point(0.0, 0.0, 0.0)
    }

    fn intersect(&self, ray: &Ray) -> Vec<f64> {
        let sphere_to_ray = ray.origin - point(0.0, 0.0, 0.0); // Sphere is at 0, 0, 0

        let a = dot(&ray.direction, &ray.direction);
        let b = 2.0 * dot(&ray.direction, &sphere_to_ray);
        let c = dot(&sphere_to_ray, &sphere_to_ray) - 1.0;

        let discriminant = (b * b) - 4.0 * a * c;

        if discriminant < 0.0 {
            return vec![];
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
        vec![t1, t2]
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn equals(&self, other: &dyn Shape) -> bool {
        other
            .as_any()
            .downcast_ref::<Sphere>()
            .map_or(false, |a| self == a)
    }

    fn fmt_boxed(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.fmt(f)
    }

    fn clone_boxed(&self) -> Box<dyn Shape> {
        Box::new(self.clone())
    }
}
