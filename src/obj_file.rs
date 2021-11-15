use regex::Regex;

use crate::{groups::Group, objects::triangle, tuples::{Tuple, point}};

#[derive(Debug)]
pub struct Parser {
    pub ignored_lines: usize,
    pub vertices: Vec<Tuple>,
    pub default_group: Group,
}

impl Parser {
    pub fn new() -> Parser {
        let ignored_lines = 0;
        let vertices = Vec::new();
        let default_group = Group::default();
        Parser {
            ignored_lines,
            vertices,
            default_group,
        }
    }

    pub fn parse_obj_file(content: &str) -> Parser {
        let mut p = Parser::new();

        let re_vertex = Regex::new(r"v (?P<x>[-0-9.]+) (?P<y>[-0-9.]+) (?P<z>[-0-9.]+)").unwrap();
        // let re_face = Regex::new(r"f (?P<p1>[0-9]+) (?P<p2>[0-9]+) (?P<p3>[0-9]+)").unwrap();
        let re_face = Regex::new(r"f(( [0-9]+){3,})").unwrap();
        let re_face_element = Regex::new(r" ([0-9]+)").unwrap();

        for line in content.lines() {
            // vertex
            if let Some(cap)  = re_vertex.captures(line) {
                let x = cap.name("x").unwrap().as_str().parse::<f64>().unwrap();
                let y = cap.name("y").unwrap().as_str().parse::<f64>().unwrap();
                let z = cap.name("z").unwrap().as_str().parse::<f64>().unwrap();
                p.vertices.push(point(x, y, z));
                continue
            }

            // face
            if let Some(cap)  = re_face.captures(line) {
                let idx_1 = cap[1];
                let idx_2 = cap.name("p2").unwrap().as_str().parse::<usize>().unwrap() - 1;
                let idx_3 = cap.name("p3").unwrap().as_str().parse::<usize>().unwrap() - 1;
                let p1 = p.vertices[idx_1].clone();
                let p2 = p.vertices[idx_2].clone();
                let p3 = p.vertices[idx_3].clone();
                p.default_group.add_object(triangle(p1, p2, p3));
                continue
            }

            // face
            if let Some(cap)  = re_face.captures(line) {
                let idx_1 = cap.name("p1").unwrap().as_str().parse::<usize>().unwrap() - 1;
                let idx_2 = cap.name("p2").unwrap().as_str().parse::<usize>().unwrap() - 1;
                let idx_3 = cap.name("p3").unwrap().as_str().parse::<usize>().unwrap() - 1;
                let p1 = p.vertices[idx_1].clone();
                let p2 = p.vertices[idx_2].clone();
                let p3 = p.vertices[idx_3].clone();
                p.default_group.add_object(triangle(p1, p2, p3));
                continue
            }

            // ignore
            p.ignored_lines += 1
        }

        p
    }
}
