use crate::MyWorld;
use cucumber::{gherkin::Step, given, when, then};
use lab_raytracing_rs::{obj_file::Parser, objects::Shape};
use crate::steps::tuples::parse_point;

#[given(regex = r"^(gibberish|file) ← a file containing:$")]
async fn prepare_file(world: &mut MyWorld, target: String, step: &Step) {
    let content = step.docstring.clone().unwrap()[1..].to_string();
    world.files.insert(target, content);
}

#[when(regex = r"^parser ← parse_obj_file\((gibberish|file)\)$")]
async fn parse_file(world: &mut MyWorld, file: String) {
    let content = world.files.get(&file).unwrap();
    world.parser = Parser::parse_obj_file(content);
}

#[when("g ← parser.default_group")]
async fn select_parser_default_group(world: &mut MyWorld) {
    world.g = world.parser.default_group.clone();
}

#[then(regex = r"^parser should have ignored ([-0-9]+) lines$")]
async fn compare_ignored_lines(world: &mut MyWorld, desired: usize) {
    assert_eq!(world.parser.ignored_lines, desired)
}

#[then(regex = r"^parser.vertices\[([0-9]+)\] = point\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$")]
async fn compare_parsed_point(world: &mut MyWorld, index: usize, x: String, y: String, z: String) {
    let desired = parse_point(&[x, y, z]);
    assert_eq!(world.parser.vertices[index-1], desired)
}

#[then(regex = r"^(t1|t2).(p1|p2|p3) = parser.vertices\[([0-9]+)\]$")]
async fn comapre_parsed_face(world: &mut MyWorld, object: String, attribute: String, index: usize) {
    let vertex = world.parser.vertices[index-1].clone();
    let triangle = match world.objects.get(&object).unwrap().shape.clone() {
        Shape::Triangle(t) => t,
        _ => panic!("only triangles are supported"), 
    };
    match attribute.as_str() {
        "p1" => assert_eq!(triangle.p1, vertex),
        "p2" => assert_eq!(triangle.p2, vertex),
        "p3" => assert_eq!(triangle.p3, vertex),
        _ => panic!("attribute not covered"), 
    };
}
