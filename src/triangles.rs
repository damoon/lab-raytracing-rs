use crate::tuples::{cross, Tuple};

#[derive(Debug, Clone, PartialEq)]
pub struct Triangle {
    pub p1: Tuple,
    pub p2: Tuple,
    pub p3: Tuple,
    pub e1: Tuple,
    pub e2: Tuple,
    pub normal: Tuple,
}

impl Triangle {
    pub fn new(p1: Tuple, p2: Tuple, p3: Tuple) -> Triangle {
        let e1 = &p2 - &p1;
        let e2 = &p3 - &p1;
        let normal = cross(&e2, &e1).normalize();
        Triangle {
            p1,
            p2,
            p3,
            e1,
            e2,
            normal,
        }
    }
}
