use std::ops;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn approximately(&self, other: Color) -> bool {
        let e = 0.0001;
        self.r - other.r < e &&
        self.g - other.g < e &&
        self.b - other.b < e
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.r, self.g, self.b)
    }
}

impl_op_ex!(+ |a: &Color, b: &Color| -> Color {
    Color {
        r: a.r + b.r,
        g: a.g + b.g,
        b: a.b + b.b,
    }
});

impl_op_ex!(- |a: &Color, b: &Color| -> Color {
    Color {
        r: a.r - b.r,
        g: a.g - b.g,
        b: a.b - b.b,
    }
});

impl_op_ex!(* |a: &Color, b: &f32| -> Color {
    Color {
        r: a.r * b,
        g: a.g * b,
        b: a.b * b,
    }
});

impl_op_ex!(* |a: &Color, b: &Color| -> Color {
    Color {
        r: a.r * b.r,
        g: a.g * b.g,
        b: a.b * b.b,
    }
});
