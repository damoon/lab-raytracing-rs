use std::{
    rc::Rc,
    sync::{Arc, RwLock},
};

use crate::{
    intersections::Intersection,
    materials::Material,
    matrices::Matrix4x4,
    rays::Ray,
    tuples::{dot, point, vector, Tuple},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    transform: Matrix4x4,
    transform_inverse: Matrix4x4,
    pub material: Material,
    pub shape: Shape,
    pub throws_shaddow: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    Sphere,
    Plane,
    Cube,
    Cylinder,
    Testshape,
}

thread_local! {
    pub static SAVED_RAY: RwLock<Arc<Ray>> = RwLock::new(Arc::new(Ray::new(point(0.0,0.0,0.0), vector(1.0, 1.0, 1.0).normalize())));
}

impl Shape {
    pub fn intersect(&self, ray: &Ray) -> Vec<f64> {
        match self {
            Shape::Plane => {
                let e = 0.0001;
                if ray.direction.y.abs() < e {
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
            Shape::Cylinder => {
                let a = f64::powf(ray.direction.x, 2.0) + f64::powf(ray.direction.z, 2.0);
                // ray is parallel to the y axis
                if a.abs() < 0.0001 {
                    return vec![];
                }
                let b = 2.0 * ray.origin.x * ray.direction.x + 2.0 * ray.origin.z * ray.direction.z;
                let c = f64::powf(ray.origin.x, 2.0) + f64::powf(ray.origin.z, 2.0) - 1.0;
                let disc = f64::powf(b, 2.0) - 4.0 * a * c;
                // ray does not intersect the cylinder
                if disc < 0.0 {
                    return vec![];
                }
                
                let t0 = (-b - disc.sqrt()) / (2.0 * a);
                let t1 = (-b + disc.sqrt()) / (2.0 * a);
                return vec![t0, t1]
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
            },
            Shape::Cylinder => {
                vector(local_point.x, 0.0, local_point.z)
            }
            Shape::Testshape => local_point - point(0.0, 0.0, 0.0),
        }
    }
}

fn check_axis(origin: f64, direction: f64) -> (f64, f64) {
    let tmin_numerator = -1.0 - origin;
    let tmax_numerator = 1.0 - origin;
    let (mut tmin, mut tmax) = if direction.abs() >= 0.0001 {
        (tmin_numerator / direction, tmax_numerator / direction)
    } else {
        (tmin_numerator * f64::INFINITY, tmax_numerator * f64::INFINITY)
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

pub fn intersect(obj: &Rc<Object>, world_ray: &Ray) -> Vec<Intersection> {
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
