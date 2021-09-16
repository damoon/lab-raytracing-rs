use cucumber_rust::{async_trait, Cucumber, World};
use lab_raytracing_rs::canvas::Canvas;
use lab_raytracing_rs::colors::Color;
use lab_raytracing_rs::tuples::Tuple;
use std::collections::HashMap;
use std::convert::Infallible;

mod steps;

pub struct MyWorld {
    pub tuples: HashMap<String, Tuple>,
    pub colors: HashMap<String, Color>,
    pub canvases: HashMap<String, Canvas>,
    pub strings: HashMap<String, String>,
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
        .cli()
        .run_and_exit()
        .await
}
