use crate::{matrices::Matrix4x4, tuples::Tuple};

#[derive(Debug, Clone, PartialEq)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Ray {
        Ray { origin, direction }
    }

    pub fn position(&self, t: f64) -> Tuple {
        self.origin + self.direction * t
    }

    pub fn transform(&self, transformation: &Matrix4x4) -> Self {
        Ray {
            origin: self.origin * transformation,
            direction: self.direction * transformation,
        }
    }
}
