use cucumber_rust::{async_trait, Cucumber, World};
use lab_raytracing_rs::canvas::Canvas;
use lab_raytracing_rs::intersections::Intersection;
use lab_raytracing_rs::matrices::{identity_matrix, Matrix2x2, Matrix3x3, Matrix4x4};
use lab_raytracing_rs::rays::Ray;
use lab_raytracing_rs::spheres::Sphere;
use lab_raytracing_rs::tuples::{point, vector, Tuple};
use std::collections::HashMap;
use std::convert::Infallible;

mod steps;

pub struct MyWorld {
    tuples: HashMap<String, Tuple>,
    c: Canvas,
    ppm: String,
    matrices: HashMap<String, Matrix>,
    intersections: HashMap<String, Intersection>,
    r: Ray,
    r2: Ray,
    s: Sphere,
    xs: Vec<Intersection>,
}
enum Matrix {
    M2x2(Matrix2x2),
    M3x3(Matrix3x3),
    M4x4(Matrix4x4),
}

#[async_trait(?Send)]
impl World for MyWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        let mut world = Self {
            tuples: HashMap::new(),
            c: Canvas::new(0, 0),
            ppm: "".to_string(),
            matrices: HashMap::new(),
            intersections: HashMap::new(),
            r: Ray {
                origin: point(0.0, 0.0, 0.0),
                direction: vector(1.0, 1.0, 1.0),
            },
            r2: Ray {
                origin: point(0.0, 0.0, 0.0),
                direction: vector(1.0, 1.0, 1.0),
            },
            s: Sphere::default(),
            xs: Vec::new(),
        };
        world.insert4x4("identity_matrix".to_string(), identity_matrix());
        Ok(world)
    }
}

impl MyWorld {
    pub fn get4x4(&self, name: &str) -> Matrix4x4 {
        match &self.matrices.get(name).unwrap() {
            Matrix::M4x4(m) => m.clone(),
            _ => panic!("not a 4x4 matrix"),
        }
    }
    pub fn insert4x4(&mut self, name: String, m: Matrix4x4) {
        self.matrices.insert(name, Matrix::M4x4(m));
    }
}

#[tokio::main]
async fn main() {
    Cucumber::<MyWorld>::new()
        .features(&["./features"])
        .steps(steps::canvas::steps())
        .steps(steps::tuples::steps())
        .steps(steps::matrices::steps())
        .steps(steps::transformations::steps())
        .steps(steps::rays::steps())
        .steps(steps::spheres::steps())
        .steps(steps::intersections::steps())
        .cli()
        .run_and_exit()
        .await
}
