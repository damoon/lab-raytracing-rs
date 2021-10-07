use crate::{
    materials::Material,
    matrices::identity_matrix,
    shapes::{Object, Shape},
};

pub fn default_plane() -> Object {
    let shape = Shape::Plane;
    let transform = identity_matrix();
    let material = Material::default();
    Object::new(shape, transform, material)
}
