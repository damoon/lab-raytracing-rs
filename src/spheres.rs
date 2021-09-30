use crate::{
    intersections::Intersection,
    materials::Material,
    matrices::{identity_matrix, Matrix4x4},
    rays::Ray,
    tuples::{dot, point, Tuple},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Sphere {
    pub transform: Matrix4x4,
    pub material: Material,
}

impl Sphere {
    pub fn default() -> Self {
        Sphere {
            transform: identity_matrix(),
            material: Material::default(),
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

    pub fn normal_at(&self, world_point: Tuple) -> Tuple {
        let object_point = self.transform.inverse().unwrap() * world_point;
        let object_normal = object_point - point(0.0, 0.0, 0.0);
        let mut world_normal = self.transform.inverse().unwrap().transpose() * object_normal;
        world_normal.w = 0.0;
        world_normal.normalize()
    }
}
