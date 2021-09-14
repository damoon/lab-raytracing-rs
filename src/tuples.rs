use std::ops;
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

    pub fn magnitude(self) -> f64 {
        ((self.v[0] * self.v[0]) + (self.v[1] * self.v[1]) + (self.v[2] * self.v[2]) + (self.v[3] * self.v[3])).sqrt()
    }

    pub fn normalize(self) -> Tuple {
        self / self.magnitude()
    }

    pub fn approximately(&self, other: Tuple) -> bool {
        let e = 0.0001;
        self.v[0] - other.v[0] < e &&
        self.v[1] - other.v[1] < e &&
        self.v[2] - other.v[2] < e &&
        self.v[3] - other.v[3] < e
    }
}

impl fmt::Display for Tuple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.v[0], self.v[1], self.v[2], self.v[3])
    }
}

impl_op_ex!(+ |a: &Tuple, b: &Tuple| -> Tuple {
    Tuple {
        v: [
            a.v[0] + b.v[0],
            a.v[1] + b.v[1],
            a.v[2] + b.v[2],
            a.v[3] + b.v[3],
        ]
    }
});

impl_op_ex!(- |a: &Tuple, b: &Tuple| -> Tuple {
    Tuple {
        v: [
            a.v[0] - b.v[0],
            a.v[1] - b.v[1],
            a.v[2] - b.v[2],
            a.v[3] - b.v[3],
        ]
    }
});

impl_op_ex!(- |a: &Tuple| -> Tuple {
    Tuple {
        v: [
            - a.v[0],
            - a.v[1],
            - a.v[2],
            - a.v[3],
        ]
    }
});

impl_op_ex_commutative!(* |a: &Tuple, b: &f64| -> Tuple {
    Tuple {
        v: [
            a.v[0] * b,
            a.v[1] * b,
            a.v[2] * b,
            a.v[3] * b,
        ]
    }
});

impl_op_ex!(/ |a: &Tuple, b: &f64| -> Tuple {
    Tuple {
        v: [
            a.v[0] / b,
            a.v[1] / b,
            a.v[2] / b,
            a.v[3] / b,
        ]
    }
});

pub fn point(x: f64, y: f64, z: f64) -> Tuple {
    Tuple::new(x, y, z, 1.0)
}

pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
    Tuple::new(x, y, z, 0.0)
}

pub fn dot(t1: Tuple, t2: Tuple) -> f64 {
    (t1.v[0] * t2.v[0]) + (t1.v[1] * t2.v[1]) + (t1.v[2] * t2.v[2]) + (t1.v[3] * t2.v[3])
}

pub fn cross(v1: Tuple, v2: Tuple) -> Tuple {
    vector(
        (v1.v[1] * v2.v[2]) - (v1.v[2] * v2.v[1]),
        (v1.v[2] * v2.v[0]) - (v1.v[0] * v2.v[2]),
        (v1.v[0] * v2.v[1]) - (v1.v[1] * v2.v[0]),
    )
}
