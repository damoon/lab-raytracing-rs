use crate::{
    intersections::Intersection,
    materials::{Material, REFRACTIVE_INDEX_GLASS},
    matrices::{identity_matrix, Matrix4x4},
    rays::Ray,
    tuples::{color, dot, point, vector, Tuple},
};
use std::sync::{Arc, RwLock};

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
    material.refractive_index = REFRACTIVE_INDEX_GLASS;
    material.reflective = 1.0;
    material.color = color(0.0, 0.0, 0.0);
    material.ambient = 0.1;
    material.diffuse = 0.1;
    material.shininess = 300.0;
    Object::new(shape, transform, material)
}

pub fn default_cylinder() -> Object {
    let shape = Shape::Cylinder(f64::NEG_INFINITY, f64::INFINITY, false);
    let transform = identity_matrix();
    let material = Material::default();
    Object::new(shape, transform, material)
}

pub fn default_cone() -> Object {
    let shape = Shape::Cone(f64::NEG_INFINITY, f64::INFINITY, false);
    let transform = identity_matrix();
    let material = Material::default();
    Object::new(shape, transform, material)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    transform: Matrix4x4,
    transform_inverse: Matrix4x4,
    pub material: Material,
    pub shape: Shape,
    pub throws_shaddow: bool,
}

//impl<'a> PartialEq for &Object<'a> {
//    fn eq(&self, other: &Self) -> bool {
//        *self as *const Object == *other as *const Object
//    }
//}
//impl PartialEq for Object {
//    fn eq(&self, other: &Self) -> bool {
//        self as *const Object == other as *const Object
//    }
//}
//impl PartialEq for Object {
//    fn eq(&self, other: &Self) -> bool {
//        self.id.eq(&other.id)
//    }
//}

#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    Sphere,
    Plane,
    Cube,
    Cylinder(f64, f64, bool),
    Cone(f64, f64, bool),
    Testshape,
}

thread_local! {
    pub static SAVED_RAY: RwLock<Arc<Ray>> = RwLock::new(Arc::new(Ray::new(point(0.0,0.0,0.0), vector(1.0, 1.0, 1.0).normalize())));
}

impl Shape {
    pub fn intersect(&self, ray: &Ray) -> Vec<f64> {
        match self {
            Shape::Plane => {
                if ray.direction.y.abs() < f64::EPSILON {
                    return vec![];
                }

                let t = -ray.origin.y / ray.direction.y;
                vec![t]
            }
            Shape::Sphere => {
                let sphere_to_ray = &ray.origin - point(0.0, 0.0, 0.0); // Sphere is at 0, 0, 0

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
            Shape::Cube => {
                let (xtmin, xtmax) = check_axis(ray.origin.x, ray.direction.x);
                let (ytmin, ytmax) = check_axis(ray.origin.y, ray.direction.y);

                let tmin = if xtmin > ytmin { xtmin } else { ytmin };
                let tmax = if xtmax < ytmax { xtmax } else { ytmax };

                if tmin > tmax {
                    return vec![];
                }

                let (ztmin, ztmax) = check_axis(ray.origin.z, ray.direction.z);

                let tmin = if ztmin > tmin { ztmin } else { tmin };
                let tmax = if ztmax < tmax { ztmax } else { tmax };

                if tmin > tmax {
                    return vec![];
                }

                vec![tmin, tmax]
            }
            Shape::Cylinder(min, max, closed) => {
                let mut xs = Vec::with_capacity(2);

                let a = ray.direction.x.powi(2) + ray.direction.z.powi(2);
                // ray is parallel to the y axis
                if a.abs() < f64::EPSILON {
                    intersect_caps_cylinder(min, max, closed, ray, &mut xs);
                    return xs;
                }
                let b = 2.0 * ray.origin.x * ray.direction.x + 2.0 * ray.origin.z * ray.direction.z;
                let c = ray.origin.x.powi(2) + ray.origin.z.powi(2) - 1.0;
                let disc = b.powi(2) - 4.0 * a * c;
                // ray does not intersect the cylinder
                if disc < 0.0 {
                    return vec![];
                }

                let mut t0 = (-b - disc.sqrt()) / (2.0 * a);
                let mut t1 = (-b + disc.sqrt()) / (2.0 * a);
                if t0 > t1 {
                    std::mem::swap(&mut t0, &mut t1)
                }

                let y0 = ray.origin.y + t0 * ray.direction.y;
                if min < &y0 && &y0 < max {
                    xs.push(t0);
                }
                let y1 = ray.origin.y + t1 * ray.direction.y;
                if min < &y1 && &y1 < max {
                    xs.push(t1);
                }

                intersect_caps_cylinder(min, max, closed, ray, &mut xs);

                xs
            }
            Shape::Cone(min, max, closed) => {
                let mut xs = Vec::with_capacity(4);

                let a = ray.direction.x.powi(2) - ray.direction.y.powi(2) + ray.direction.z.powi(2);
                let b = 2.0 * ray.origin.x * ray.direction.x - 2.0 * ray.origin.y * ray.direction.y
                    + 2.0 * ray.origin.z * ray.direction.z;
                let c = ray.origin.x.powi(2) - ray.origin.y.powi(2) + ray.origin.z.powi(2);

                if a.abs() < f64::EPSILON && b.abs() > f64::EPSILON {
                    let t = -c / (2.0 * b);
                    xs.push(t);
                }

                if a.abs() > f64::EPSILON {
                    let disc = b.powi(2) - 4.0 * a * c;
                    // ray does not intersect the cylinder
                    if disc >= 0.0 {
                        let mut t0 = (-b - disc.sqrt()) / (2.0 * a);
                        let mut t1 = (-b + disc.sqrt()) / (2.0 * a);
                        if t0 > t1 {
                            std::mem::swap(&mut t0, &mut t1)
                        }

                        let y0 = ray.origin.y + t0 * ray.direction.y;
                        if min < &y0 && &y0 < max {
                            xs.push(t0);
                        }
                        let y1 = ray.origin.y + t1 * ray.direction.y;
                        if min < &y1 && &y1 < max {
                            xs.push(t1);
                        }
                    }
                }

                intersect_caps_cone(min, max, closed, ray, &mut xs);

                xs
            }
            Shape::Testshape => {
                SAVED_RAY.with(|c| *c.write().unwrap() = Arc::new(ray.clone()));
                vec![]
            }
        }
    }

    pub fn normal_at(&self, local_point: &Tuple) -> Tuple {
        match self {
            Shape::Plane => vector(0.0, 1.0, 0.0),
            Shape::Sphere => local_point - point(0.0, 0.0, 0.0),
            Shape::Cube => {
                let xabs = local_point.x.abs();
                let yabs = local_point.y.abs();
                let zabs = local_point.z.abs();
                match max_index(xabs, yabs, zabs) {
                    0 => vector(local_point.x, 0.0, 0.0),
                    1 => vector(0.0, local_point.y, 0.0),
                    _ => vector(0.0, 0.0, local_point.z),
                }
            }
            Shape::Cylinder(minimum, maximum, _closed) => {
                // compute the square of the distance from the y axis
                let dist = local_point.x.powi(2) + local_point.z.powi(2);
                if dist < 1.0 && local_point.y >= maximum - f64::EPSILON {
                    return vector(0.0, 1.0, 0.0);
                }
                if dist < 1.0 && local_point.y <= minimum + f64::EPSILON {
                    return vector(0.0, -1.0, 0.0);
                }
                vector(local_point.x, 0.0, local_point.z)
            }
            Shape::Cone(minimum, maximum, _closed) => {
                // compute the square of the distance from the y axis
                let dist = local_point.x.powi(2) + local_point.z.powi(2);
                if dist < maximum.powi(2) && local_point.y >= maximum - f64::EPSILON {
                    return vector(0.0, 1.0, 0.0);
                }
                if dist < minimum.powi(2) && local_point.y <= minimum + f64::EPSILON {
                    return vector(0.0, -1.0, 0.0);
                }
                let mut y = dist.sqrt();
                if local_point.y > 0.0 {
                    y = -y;
                }
                vector(local_point.x, y, local_point.z)
            }
            Shape::Testshape => local_point - point(0.0, 0.0, 0.0),
        }
    }
}

fn check_axis(origin: f64, direction: f64) -> (f64, f64) {
    let tmin_numerator = -1.0 - origin;
    let tmax_numerator = 1.0 - origin;
    let (mut tmin, mut tmax) = if direction.abs() >= f64::EPSILON {
        (tmin_numerator / direction, tmax_numerator / direction)
    } else {
        (
            tmin_numerator * f64::INFINITY,
            tmax_numerator * f64::INFINITY,
        )
    };
    if tmin > tmax {
        std::mem::swap(&mut tmin, &mut tmax)
    }
    (tmin, tmax)
}

fn max_index(a: f64, b: f64, c: f64) -> usize {
    let mut n = 0;
    let mut max = a;
    if b > max {
        max = b;
        n = 1;
    }
    if c > max {
        n = 2
    }
    n
}

// a helper function to reduce duplication.
// checks to see if the intersection at `t` is within a radius
// of 1 (the radius of your cylinders) from the y axis.
fn check_cap(ray: &Ray, t: f64, r: f64) -> bool {
    let x = ray.origin.x + t * ray.direction.x;
    let z = ray.origin.z + t * ray.direction.z;
    x.powi(2) + z.powi(2) <= r.powi(2)
}

fn intersect_caps_cylinder(
    minimum: &f64,
    maximum: &f64,
    closed: &bool,
    ray: &Ray,
    xs: &mut Vec<f64>,
) {
    // caps only matter if the cylinder is closed, and might possibly be
    // intersected by the ray.
    if !closed || ray.direction.y.abs() < f64::EPSILON {
        return;
    }

    // check for an intersection with the lower end cap by intersecting
    // the ray with the plane at y=cyl.minimum
    let t = (minimum - ray.origin.y) / ray.direction.y;
    if check_cap(ray, t, 1.0) {
        xs.push(t);
    }

    // check for an intersection with the upper end cap by intersecting
    // the ray with the plane at y=cyl.maximum
    let t = (maximum - ray.origin.y) / ray.direction.y;
    if check_cap(ray, t, 1.0) {
        xs.push(t);
    }
}

fn intersect_caps_cone(minimum: &f64, maximum: &f64, closed: &bool, ray: &Ray, xs: &mut Vec<f64>) {
    // caps only matter if the cylinder is closed, and might possibly be
    // intersected by the ray.
    if !closed || ray.direction.y.abs() < f64::EPSILON {
        return;
    }

    // check for an intersection with the lower end cap by intersecting
    // the ray with the plane at y=cyl.minimum
    let t = (minimum - ray.origin.y) / ray.direction.y;
    if check_cap(ray, t, *minimum) {
        xs.push(t);
    }

    // check for an intersection with the upper end cap by intersecting
    // the ray with the plane at y=cyl.maximum
    let t = (maximum - ray.origin.y) / ray.direction.y;
    if check_cap(ray, t, *maximum) {
        xs.push(t);
    }
}

pub fn intersect(obj: &Arc<Object>, world_ray: &Ray) -> Vec<Intersection> {
    let local_ray = world_ray.transform(&obj.transform_inverse);
    obj.shape
        .intersect(&local_ray)
        .iter()
        .map(|t| Intersection {
            t: *t,
            object: obj.clone(),
        })
        .collect()
}

impl Object {
    pub fn new(shape: Shape, transform: Matrix4x4, material: Material) -> Object {
        let transform_inverse = transform.inverse().unwrap();
        Object {
            transform,
            transform_inverse,
            material,
            shape,
            throws_shaddow: true,
        }
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
