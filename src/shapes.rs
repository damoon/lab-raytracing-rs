use crate::{
    intersections::Intersection, materials::Material, matrices::Matrix4x4, rays::Ray, tuples::Tuple,
};
use std::{any::Any, ops::Deref};

#[derive(Debug)]
pub struct Object {
    transform: Matrix4x4,
    transform_inverse: Matrix4x4,
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
    pub fn new(shape: Box<dyn Shape>, transform: Matrix4x4, material: Material) -> Object {
        let transform_inverse = transform.inverse().unwrap();
        Object {
            shape,
            transform,
            transform_inverse,
            material,
        }
    }

    pub fn intersect(&self, world_ray: &Ray) -> Vec<Intersection> {
        let local_ray = world_ray.transform(&self.transform_inverse);
        self.shape
            .intersect(&local_ray)
            .iter()
            .map(|&i| Intersection {
                t: i,
                object: self.clone(),
            })
            .collect()
    }

    pub fn set_transform(&mut self, transform: Matrix4x4) {
        self.transform = transform;
        self.transform_inverse = self.transform.inverse().unwrap();
    }

    pub fn transform(&self) -> &Matrix4x4 {
        &self.transform
    }

    pub fn transform_inverse(&self) -> &Matrix4x4 {
        &self.transform_inverse
    }

    pub fn normal_at(&self, world_point: &Tuple) -> Tuple {
        let local_point = &self.transform_inverse * world_point;
        let local_normal = self.shape.normal_at(&local_point);
        let mut world_normal = self.transform_inverse.transpose() * local_normal;
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
            transform_inverse: self.transform.inverse().unwrap(),
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
