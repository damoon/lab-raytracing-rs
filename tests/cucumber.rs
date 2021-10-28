use async_trait::async_trait;
use cucumber::{cli, given, then, when, WorldInit, WriterExt};
use lab_raytracing_rs::camera::Camera;
use lab_raytracing_rs::canvas::Canvas;
use lab_raytracing_rs::groups::Group;
use lab_raytracing_rs::intersections::{
    prepare_computations, Intersection, IntersectionPrecomputations,
};
use lab_raytracing_rs::lights::Pointlight;
use lab_raytracing_rs::materials::Material;
use lab_raytracing_rs::matrices::{identity_matrix, Matrix2x2, Matrix3x3, Matrix4x4};
use lab_raytracing_rs::objects::{default_sphere, Object};
use lab_raytracing_rs::patterns::{test_pattern, Pattern};
use lab_raytracing_rs::rays::Ray;
use lab_raytracing_rs::tuples::{color, point, vector, Tuple};
use lab_raytracing_rs::world::World;
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;

mod steps;

#[derive(Debug, WorldInit)]
pub struct MyWorld {
    tuples: HashMap<String, Tuple>,
    floats: HashMap<String, f64>,
    usizes: HashMap<String, usize>,
    canvas: Canvas,
    image: Canvas,
    camera: Camera,
    ppm: String,
    in_shadow: bool,
    matrices: HashMap<String, Matrix>,
    intersections: HashMap<String, Intersection>,
    r: Ray,
    r2: Ray,
    objects: HashMap<String, Arc<Object>>,
    xs: Vec<Intersection>,
    light: Pointlight,
    m: Material,
    w: World,
    comps: IntersectionPrecomputations,
    pattern: Pattern,
    g: Group,
    g1: Group,
    g2: Group,
}

#[derive(Debug)]
enum Matrix {
    M2x2(Matrix2x2),
    M3x3(Matrix3x3),
    M4x4(Matrix4x4),
}

#[async_trait(?Send)]
impl cucumber::World for MyWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        let mut world = Self {
            tuples: HashMap::new(),
            floats: HashMap::new(),
            usizes: HashMap::new(),
            camera: Camera::new(0, 0, 0.0),
            canvas: Canvas::new(0, 0),
            image: Canvas::new(0, 0),
            ppm: "".to_string(),
            in_shadow: true,
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
            objects: HashMap::new(),
            xs: Vec::new(),
            light: Pointlight::new(point(0.0, 0.0, 0.0), color(1.0, 1.0, 1.0)),
            m: Material::default(),
            w: World::default(),
            comps: prepare_computations(
                &Intersection {
                    t: 1.0,
                    object: Arc::new(default_sphere()),
                },
                &Ray {
                    origin: point(2.0, 0.0, 0.0),
                    direction: vector(1.0, 0.0, 0.0),
                },
                &Vec::new(),
            ),
            pattern: test_pattern(),
            g: Group::default(),
            g1: Group::default(),
            g2: Group::default(),
        };
        world.insert4x4("identity_matrix".to_string(), identity_matrix());
        Ok(world)
    }
}

impl MyWorld {
    pub fn get4x4(&self, name: &str) -> &Matrix4x4 {
        match &self.matrices.get(name).unwrap() {
            Matrix::M4x4(m) => m,
            _ => panic!("not a 4x4 matrix"),
        }
    }
    pub fn insert4x4(&mut self, name: String, m: Matrix4x4) {
        self.matrices.insert(name, Matrix::M4x4(m));
    }
}

#[tokio::main]
async fn main() {
    MyWorld::cucumber()
        .repeat_failed()
        //.repeat_skipped()
        // .steps(steps::camera::steps())
        // .steps(steps::canvas::steps())
        // .steps(steps::cylinders::steps())
        // .steps(steps::groups::steps())
        // .steps(steps::intersections::steps())
        // .steps(steps::lights::steps())
        // .steps(steps::materials::steps())
        // .steps(steps::matrices::steps())
        // .steps(steps::patterns::steps())
        // .steps(steps::planes::steps())
        // .steps(steps::rays::steps())
        // .steps(steps::objects::steps())
        // .steps(steps::spheres::steps())
        // .steps(steps::transformations::steps())
        // .steps(steps::tuples::steps())
        // .steps(steps::world::steps())
        .run_and_exit("./features")
        .await
}
