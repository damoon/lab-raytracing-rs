use std::collections::HashMap;

use regex::Regex;

use crate::{
    groups::Group,
    objects::{smooth_triangle, triangle},
    tuples::{point, Tuple},
};

#[derive(Debug)]
pub struct Parser {
    pub ignored_lines: usize,
    pub vertices: Vec<Tuple>,
    pub normals: Vec<Tuple>,
    current_group: String,
    pub groups: HashMap<String, Group>,
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser {
    pub fn new() -> Parser {
        let ignored_lines = 0;
        let vertices = Vec::default();
        let normals = Vec::default();
        let current_group = "default_group".to_string();
        let mut groups = HashMap::default();
        groups.insert(current_group.clone(), Group::default());
        Parser {
            ignored_lines,
            vertices,
            normals,
            current_group,
            groups,
        }
    }

    pub fn parse_obj_file(content: &str) -> Parser {
        let mut p = Parser::new();

        let re_vertex = Regex::new(r"^v +(?P<x>[-0-9.]+) (?P<y>[-0-9.]+) (?P<z>[-0-9.]+)$")
            .expect("vertex regex is invalid");
        let re_normal = Regex::new(r"^vn +(?P<x>[-0-9.]+) (?P<y>[-0-9.]+) (?P<z>[-0-9.]+)$")
            .expect("vertex regex is invalid");
        let re_face = Regex::new(r"^f(( [0-9]+){3,})$").expect("face regex is invalid");
        let re_smooth_face =
            Regex::new(r"^f(( +[0-9]*/[0-9]*/[0-9]*){3,})$").expect("face regex is invalid");
        let re_group = Regex::new(r"^g +(?P<name>\w+)$").expect("group regex is invalid");

        for line in content.lines() {
            let line = line.trim();

            // vertex
            if let Some(cap) = re_vertex.captures(line) {
                let x = cap
                    .name("x")
                    .expect("x not found")
                    .as_str()
                    .parse::<f64>()
                    .expect("x is not a usize");
                let y = cap
                    .name("y")
                    .expect("y not found")
                    .as_str()
                    .parse::<f64>()
                    .expect("y is not a usize");
                let z = cap
                    .name("z")
                    .expect("z not found")
                    .as_str()
                    .parse::<f64>()
                    .expect("z is not a usize");
                p.vertices.push(point(x, y, z));
                continue;
            }

            // vertex normal
            if let Some(cap) = re_normal.captures(line) {
                let x = cap
                    .name("x")
                    .expect("x not found")
                    .as_str()
                    .parse::<f64>()
                    .expect("x is not a usize");
                let y = cap
                    .name("y")
                    .expect("y not found")
                    .as_str()
                    .parse::<f64>()
                    .expect("y is not a usize");
                let z = cap
                    .name("z")
                    .expect("z not found")
                    .as_str()
                    .parse::<f64>()
                    .expect("z is not a usize");
                p.normals.push(point(x, y, z));
                continue;
            }

            // face
            if let Some(cap) = re_face.captures(line) {
                let indices: Vec<usize> = cap[1]
                    .trim()
                    .split_whitespace()
                    .map(|s| s.parse::<usize>().expect("offset not a usize"))
                    .collect();
                for i in 1..(indices.len() - 1) {
                    let idx_1 = indices[0];
                    let idx_2 = indices[i];
                    let idx_3 = indices[i + 1];
                    let p1 = p.vertices[idx_1 - 1].clone();
                    let p2 = p.vertices[idx_2 - 1].clone();
                    let p3 = p.vertices[idx_3 - 1].clone();
                    p.groups
                        .get_mut(&p.current_group)
                        .expect("group is missing")
                        .add_object(triangle(p1, p2, p3));
                }
                continue;
            }

            // smooth face
            if let Some(cap) = re_smooth_face.captures(line) {
                let indices: Vec<Vec<Option<usize>>> = cap[1]
                    .trim()
                    .split_whitespace()
                    .map(|s: &str| {
                        s.split("/")
                            .map(|s| match s.parse::<usize>() {
                                Ok(v) => Some(v),
                                Err(_) => None,
                            })
                            .collect()
                    })
                    .collect();
                for i in 1..(indices.len() - 1) {
                    let vectex_1 = indices[0][0].expect("vectex_1 is missing");
                    let vectex_2 = indices[i][0].expect("vectex_2 is missing");
                    let vectex_3 = indices[i + 1][0].expect("vectex_3 is missing");
                    let normal_1 = indices[0][2].expect("normal_1 is missing");
                    let normal_2 = indices[i][2].expect("normal_2 is missing");
                    let normal_3 = indices[i + 1][2].expect("normal_3 is missing");
                    let p1 = p.vertices[vectex_1 - 1].clone();
                    let p2 = p.vertices[vectex_2 - 1].clone();
                    let p3 = p.vertices[vectex_3 - 1].clone();
                    let n1 = p.normals[normal_1 - 1].clone();
                    let n2 = p.normals[normal_2 - 1].clone();
                    let n3 = p.normals[normal_3 - 1].clone();
                    let obj = smooth_triangle(p1, p2, p3, n1, n2, n3);
                    // let obj = triangle(p1, p2, p3);
                    p.groups
                        .get_mut(&p.current_group)
                        .expect("group is missing")
                        .add_object(obj);
                }
                continue;
            }

            // group
            if let Some(cap) = re_group.captures(line) {
                let name = cap
                    .name("name")
                    .expect("group name is missing")
                    .as_str()
                    .to_string();
                p.groups.insert(name.to_string(), Group::default()); // .expect("group defined twice");
                p.current_group = name;
                continue;
            }

            // ignore
            p.ignored_lines += 1
        }

        p
    }

    pub fn to_group(&self) -> Group {
        let mut g = Group::default();
        for (name, sub_group) in self.groups.iter() {
            eprintln!("add group {} with {} elements", name, sub_group.len());
            g.add_group(sub_group.clone());
        }
        g
    }
}
