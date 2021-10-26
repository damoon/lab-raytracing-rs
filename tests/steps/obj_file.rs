use crate::MyWorld;
use cucumber::{gherkin::Step, given, when};
use lab_raytracing_rs::obj_file::Parser;

#[given(regex = r"^(gibberish) ← a file containing:$")]
async fn prepare_file(world: &mut MyWorld, target: String, step: &Step) {
    let content = step.docstring.clone().unwrap();
    world.files.insert(target, content);
}

#[when(regex = r"^parser ← parse_obj_file\((gibberish)\)$")]
async fn parse_file(world: &mut MyWorld, file: String) {
    let content = world.files.get(&file).unwrap();
    world.parser = Some(Parser::parse_obj_file(content));
}
