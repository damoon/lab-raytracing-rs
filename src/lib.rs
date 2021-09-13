use std::ops::Add;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Tuple {
    v: [f64; 4]
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Tuple {
        Tuple { 
            v: [x, y, z, w]
        }
    }
    pub fn x(self) -> f64 {
        self.v[0]
    }
    pub fn y(self) -> f64 {
        self.v[1]
    }
    pub fn z(self) -> f64 {
        self.v[2]
    }
    pub fn w(self) -> f64 {
        self.v[3]
    }

    pub fn is_point(self) -> bool {
        self.w() != 0.0
    }
    pub fn is_vector(self) -> bool {
        !self.is_point()
    }
}

impl fmt::Display for Tuple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.v[0], self.v[1], self.v[2], self.v[3])
    }
}

impl Add for &Tuple {
    type Output = Tuple;

    fn add(self, other: Self) -> Tuple {
        Tuple {
            v: [
                self.v[0] + other.v[0],
                self.v[1] + other.v[1],
                self.v[2] + other.v[2],
                self.v[3] + other.v[3],
            ]
        }
    }
}

pub fn point(x: f64, y: f64, z: f64) -> Tuple {
    Tuple::new(x, y, z, 1.0)
}

pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
    Tuple::new(x, y, z, 0.0)
}
