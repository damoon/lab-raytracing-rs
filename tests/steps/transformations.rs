use cucumber_rust::Steps;

use crate::MyWorld;

pub fn steps() -> Steps<MyWorld> {
    let steps: Steps<MyWorld> = Steps::new();

    steps
}
