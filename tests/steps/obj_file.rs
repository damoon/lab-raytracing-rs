use crate::steps::tuples::parse_point;
use crate::MyWorld;
use cucumber::{gherkin::Step, given, then, when};
use lab_raytracing_rs::{obj_file::Parser, objects::Shape};
use std::fs;
use std::path::Path;

#[given(regex = r"^(gibberish|file) ← a file containing:$")]
async fn prepare_file(world: &mut MyWorld, target: String, step: &Step) {
    let content = step.docstring.clone().unwrap()[1..].to_string();
    world.files.insert(target, content);
}

#[given(regex = r#"^(file) ← the file "([\w\.]+)"$"#)]
async fn read_file(world: &mut MyWorld, target: String, path: String) {
    let path = Path::new("./features/").join(path);
    let content = fs::read_to_string(path).unwrap();
    world.files.insert(target, content);
}

#[given(regex = r"^parser ← parse_obj_file\((gibberish|file)\)$")]
#[when(regex = r"^parser ← parse_obj_file\((gibberish|file)\)$")]
async fn parse_file(world: &mut MyWorld, file: String) {
    let content = world.files.get(&file).unwrap();
    world.parser = Parser::parse_obj_file(content);
}

#[given("g ← parser.default_group")]
#[when("g ← parser.default_group")]
async fn select_parser_default_group(world: &mut MyWorld) {
    world.g = world.parser.groups.get("default_group").unwrap().clone();
}

#[when(regex = r#"(g1|g2) ← "(\w+)" from parser"#)]
async fn select_group_from_parser(world: &mut MyWorld, target: String, group_name: String) {
    let g = world.parser.groups.get(&group_name).expect("group missing").clone();
    match target.as_str() {
        "g1" => world.g1 = g,
        "g2" => world.g2 = g,
        _ => panic!("group name not covered"),
    }
}

#[when("g ← obj_to_group(parser)")]
async fn parser_to_group(world: &mut MyWorld) {
    world.g = world.parser.to_group();
}

#[then(regex = r#"g includes "(FirstGroup|SecondGroup)" from parser"#)]
async fn compare_group_from_parser(world: &mut MyWorld, group_name: String) {
    let g = world.parser.groups.get(&group_name).expect("group missing");
    assert!(world.g.contains_group(g));
}

#[then(regex = r"^parser should have ignored ([-0-9]+) lines$")]
async fn compare_ignored_lines(world: &mut MyWorld, desired: usize) {
    assert_eq!(world.parser.ignored_lines, desired)
}

#[then(regex = r"^parser.vertices\[([0-9]+)\] = point\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$")]
async fn compare_parsed_point(world: &mut MyWorld, index: usize, x: String, y: String, z: String) {
    let desired = parse_point(&[x, y, z]);
    assert_eq!(world.parser.vertices[index - 1], desired)
}

#[then(regex = r"^(t1|t2|t3).(p1|p2|p3) = parser.vertices\[([0-9]+)\]$")]
async fn comapre_parsed_face(world: &mut MyWorld, object: String, attribute: String, index: usize) {
    let vertex = world.parser.vertices[index - 1].clone();
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
