use approx::AbsDiffEq;
use auto_ops::{impl_op_ex, impl_op_ex_commutative};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Tuple {
        Tuple { x, y, z, w }
    }

    pub fn is_point(&self) -> bool {
        self.w != 0.0
    }

    pub fn is_vector(&self) -> bool {
        !self.is_point()
    }

    pub fn magnitude(&self) -> f64 {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z) + (self.w * self.w)).sqrt()
    }

    pub fn normalize(&self) -> Tuple {
        let m = self.magnitude();
        self / m
    }
}

impl AbsDiffEq for Tuple {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        0.0001
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        f64::abs_diff_eq(&self.x, &other.x, epsilon)
            && f64::abs_diff_eq(&self.y, &other.y, epsilon)
            && f64::abs_diff_eq(&self.z, &other.z, epsilon)
            && f64::abs_diff_eq(&self.w, &other.w, epsilon)
    }
}

impl fmt::Display for Tuple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

impl_op_ex!(+ |a: &Tuple, b: &Tuple| -> Tuple {
    Tuple {
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z,
        w: a.w + b.w,
    }
});

impl_op_ex!(-|a: &Tuple, b: &Tuple| -> Tuple {
    Tuple {
        x: a.x - b.x,
        y: a.y - b.y,
        z: a.z - b.z,
        w: a.w - b.w,
    }
});

impl_op_ex!(-|a: &Tuple| -> Tuple {
    Tuple {
        x: -a.x,
        y: -a.y,
        z: -a.z,
        w: -a.w,
    }
});

impl_op_ex_commutative!(*|a: &Tuple, b: &f64| -> Tuple {
    Tuple {
        x: a.x * b,
        y: a.y * b,
        z: a.z * b,
        w: a.w * b,
    }
});

impl_op_ex!(*|a: &Tuple, b: &Tuple| -> Tuple {
    Tuple {
        x: a.x * b.x,
        y: a.y * b.y,
        z: a.z * b.z,
        w: a.w * b.w,
    }
});

impl_op_ex!(/ |a: &Tuple, b: &f64| -> Tuple {
    Tuple {
        x: a.x / b,
        y: a.y / b,
        z: a.z / b,
        w: a.w / b,
    }
});

pub fn point(x: f64, y: f64, z: f64) -> Tuple {
    Tuple::new(x, y, z, 1.0)
}

pub fn color(r: f64, g: f64, b: f64) -> Tuple {
    Tuple::new(r, g, b, 0.0)
}

pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
    Tuple::new(x, y, z, 0.0)
}

pub fn dot(t1: &Tuple, t2: &Tuple) -> f64 {
    (t1.x * t2.x) + (t1.y * t2.y) + (t1.z * t2.z) + (t1.w * t2.w)
}

pub fn cross(v1: &Tuple, v2: &Tuple) -> Tuple {
    vector(
        (v1.y * v2.z) - (v1.z * v2.y),
        (v1.z * v2.x) - (v1.x * v2.z),
        (v1.x * v2.y) - (v1.y * v2.x),
    )
}

pub fn reflect(in_: &Tuple, normal: &Tuple) -> Tuple {
    in_ - normal * 2.0 * dot(in_, normal)
}

impl std::cmp::PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        if (self.x - other.x).abs() > f64::EPSILON {
            return false;
        }
        if (self.y - other.y).abs() > f64::EPSILON {
            return false;
        }
        if (self.z - other.z).abs() > f64::EPSILON {
            return false;
        }
        true
    }
}
