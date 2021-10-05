use crate::{
    intersections::Intersection, materials::Material, matrices::Matrix4x4, rays::Ray, tuples::Tuple,
};
use std::{any::Any, ops::Deref};

#[derive(Debug)]
pub struct Object {
    pub transform: Matrix4x4,
    pub material: Material,
    pub shape: Box<dyn Shape>,
}

pub trait Shape {
    fn intersect(&self, local_ray: &Ray) -> Vec<f64>;
    fn normal_at(&self, local_point: &Tuple) -> Tuple;
    fn as_any(&self) -> &dyn Any;
    fn equals(&self, other: &dyn Shape) -> bool;
    fn fmt_boxed(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result;
    fn clone_boxed(&self) -> Box<dyn Shape>;
}

impl Object {
    pub fn intersect(&self, world_ray: &Ray) -> Vec<Intersection> {
        let local_ray = world_ray.transform(&self.transform.inverse().unwrap());
        self.shape
            .intersect(&local_ray)
            .iter()
            .map(|&i| Intersection {
                t: i,
                object: self.clone(),
            })
            .collect()
    }

    pub fn normal_at(&self, world_point: &Tuple) -> Tuple {
        let local_point = self.transform.inverse().unwrap() * world_point;
        let local_normal = self.shape.normal_at(&local_point);
        let mut world_normal = self.transform.inverse().unwrap().transpose() * local_normal;
        world_normal.w = 0.0;
        world_normal.normalize()
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        self.transform == other.transform
            && self.material == other.material
            && self.shape.equals(other.shape.deref())
    }
}

impl Clone for Object {
    fn clone(&self) -> Self {
        Object {
            shape: self.shape.clone(),
            transform: self.transform.clone(),
            material: self.material.clone(),
        }
    }
}

// impl PartialEq for dyn Shape {
//     fn eq(&self, other: &Self) -> bool {
//         self.equals(other)
//     }
// }

impl std::fmt::Debug for dyn Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.fmt_boxed(f)
    }
}

impl Clone for Box<dyn Shape> {
    fn clone(&self) -> Self {
        self.clone_boxed()
    }
}
