use crate::{
    materials::Material,
    matrices::identity_matrix,
    shapes::{Object, Shape},
};
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq)]
pub struct Sphere {}

pub fn default_sphere() -> Object {
    let shape = Shape::Sphere;
    let transform = identity_matrix();
    let material = Material::default();
    Object::new(shape, transform, material)
}

pub fn default_testshape() -> Object {
    let shape = Shape::Testshape;
    let transform = identity_matrix();
    let material = Material::default();
    Object::new(shape, transform, material)
}

pub fn default_cylinder() -> Object {
    let shape = Shape::Cylinder;
    let transform = identity_matrix();
    let material = Material::default();
    Object::new(shape, transform, material)
}
