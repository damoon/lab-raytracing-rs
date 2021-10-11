use crate::{
    materials::Material,
    matrices::identity_matrix,
    shapes::{Object, Shape},
    tuples::color,
};

pub fn default_plane() -> Object {
    let shape = Shape::Plane;
    let transform = identity_matrix();
    let material = Material::default();
    Object::new(shape, transform, material)
}

pub fn default_cube() -> Object {
    let shape = Shape::Cube;
    let transform = identity_matrix();
    let material = Material::default();
    Object::new(shape, transform, material)
}

pub fn glass_sphere() -> Object {
    let shape = Shape::Sphere;
    let transform = identity_matrix();
    let mut material = Material::default();
    material.transparency = 1.0;
    material.refractive_index = 1.5;
    material.reflective = 1.0;
    material.color = color(0.0, 0.0, 0.0);
    material.ambient = 0.1;
    material.diffuse = 0.1;
    material.shininess = 300.0;
    Object::new(shape, transform, material)
}
