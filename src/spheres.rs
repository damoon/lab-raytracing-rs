use crate::{
    intersections::Intersection,
    matrices::{identity_matrix, Matrix4x4},
    rays::Ray,
    tuples::{dot, point},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Sphere {
    pub transform: Matrix4x4,
}

impl Sphere {
    pub fn default() -> Self {
        Sphere {
            transform: identity_matrix(),
        }
    }

    pub fn intersect(&self, world_ray: &Ray) -> Vec<Intersection> {
        let ray = world_ray.transform(&self.transform.inverse().unwrap());

        let sphere_to_ray = ray.origin - point(0.0, 0.0, 0.0); // Sphere is at 0, 0, 0

        let a = dot(&ray.direction, &ray.direction);
        let b = 2.0 * dot(&ray.direction, &sphere_to_ray);
        let c = dot(&sphere_to_ray, &sphere_to_ray) - 1.0;

        let discriminant = (b * b) - 4.0 * a * c;

        if discriminant < 0.0 {
            return Vec::new();
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        vec![
            Intersection {
                t: t1,
                object: self.clone(),
            },
            Intersection {
                t: t2,
                object: self.clone(),
            },
        ]
    }
}
