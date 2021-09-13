use cucumber_rust::{async_trait, Cucumber, World};
use lab_raytracing_rs::{Tuple};
use std::convert::Infallible;
use std::collections::HashMap;

mod steps;

pub struct MyWorld {
    pub tuples: HashMap<String, Tuple>
}

#[async_trait(?Send)]
impl World for MyWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self{
            tuples: HashMap::new()
        })
    }
}

#[tokio::main]
async fn main() {
    Cucumber::<MyWorld>::new()
        .features(&["./features"])
        .steps(steps::tuples::steps())
        .cli()
        .run_and_exit()
        .await
}
