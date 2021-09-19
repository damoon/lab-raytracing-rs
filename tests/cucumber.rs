use cucumber_rust::{async_trait, Cucumber, World};
use lab_raytracing_rs::canvas::Canvas;
use lab_raytracing_rs::colors::Color;
use lab_raytracing_rs::matrices::{Matrix2x2, Matrix3x3, Matrix4x4};
use lab_raytracing_rs::tuples::Tuple;
use std::collections::HashMap;
use std::convert::Infallible;

mod steps;

pub struct MyWorld {
    tuples: HashMap<String, Tuple>,
    colors: HashMap<String, Color>,
    canvases: HashMap<String, Canvas>,
    strings: HashMap<String, String>,
    matrices: HashMap<String, Matrix>,
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
        Ok(Self {
            tuples: HashMap::new(),
            colors: HashMap::new(),
            canvases: HashMap::new(),
            strings: HashMap::new(),
            matrices: HashMap::new(),
        })
    }
}

#[tokio::main]
async fn main() {
    Cucumber::<MyWorld>::new()
        .features(&["./features"])
        .steps(steps::canvas::steps())
        .steps(steps::colors::steps())
        .steps(steps::tuples::steps())
        .steps(steps::matrices::steps())
        .steps(steps::transformations::steps())
        .cli()
        .run_and_exit()
        .await
}
