use super::tuples::Tuple;
use auto_ops::{impl_op_ex, impl_op_ex_commutative};
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Matrix2x2 {
    state: [[f64; 2]; 2],
}

#[derive(Debug, PartialEq)]
pub struct Matrix3x3 {
    state: [[f64; 3]; 3],
}

#[derive(Debug, Clone)]
pub struct Matrix4x4 {
    state: [[f64; 4]; 4],
}

impl fmt::Display for Matrix4x4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({:.5}, {:.5}, {:.5}, {:.5})\n({:.5}, {:.5}, {:.5}, {:.5})\n({:.5}, {:.5}, {:.5}, {:.5})\n({:.5}, {:.5}, {:.5}, {:.5})",
            self.at(0, 0),
            self.at(0, 1),
            self.at(0, 2),
            self.at(0, 3),
            self.at(1, 0),
            self.at(1, 1),
            self.at(1, 2),
            self.at(1, 3),
            self.at(2, 0),
            self.at(2, 1),
            self.at(2, 2),
            self.at(2, 3),
            self.at(3, 0),
            self.at(3, 1),
            self.at(3, 2),
            self.at(3, 3)
        )
    }
}

impl Default for Matrix2x2 {
    fn default() -> Self {
        Self::new()
    }
}

impl Matrix2x2 {
    pub fn new() -> Matrix2x2 {
        Matrix2x2 {
            state: [[0.0_f64; 2]; 2],
        }
    }

    pub fn new_from(state: [[f64; 2]; 2]) -> Matrix2x2 {
        Matrix2x2 { state }
    }

    pub fn at(&self, w: usize, h: usize) -> f64 {
        self.state[w][h]
    }

    pub fn determinant(&self) -> f64 {
        (self.state[0][0] * self.state[1][1]) - (self.state[1][0] * self.state[0][1])
    }

    pub fn invertible(&self) -> bool {
        self.determinant() != 0.0
    }
}

impl Default for Matrix3x3 {
    fn default() -> Self {
        Self::new()
    }
}

impl Matrix3x3 {
    pub fn new() -> Matrix3x3 {
        Matrix3x3 {
            state: [[0.0_f64; 3]; 3],
        }
    }

    pub fn new_from(state: [[f64; 3]; 3]) -> Matrix3x3 {
        Matrix3x3 { state }
    }

    pub fn at(&self, w: usize, h: usize) -> f64 {
        self.state[w][h]
    }

    pub fn submatrix(&self, w: usize, h: usize) -> Matrix2x2 {
        let mut state = [[0.0_f64; 2]; 2];
        for (i, row) in state.iter_mut().enumerate() {
            let mut x = i;
            if i >= w {
                x += 1;
            }

            for (j, cell) in row.iter_mut().enumerate() {
                let mut y = j;
                if j >= h {
                    y += 1;
                }

                *cell = self.at(x, y);
            }
        }
        Matrix2x2 { state }
    }

    pub fn minor(&self, w: usize, h: usize) -> f64 {
        self.submatrix(w, h).determinant()
    }

    pub fn cofactor(&self, w: usize, h: usize) -> f64 {
        match (w + h) % 2 == 0 {
            true => self.submatrix(w, h).determinant(),
            false => -self.submatrix(w, h).determinant(),
        }
    }

    pub fn determinant(&self) -> f64 {
        (self.state[0][0] * self.cofactor(0, 0))
            + (self.state[0][1] * self.cofactor(0, 1))
            + (self.state[0][2] * self.cofactor(0, 2))
    }

    pub fn invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    pub fn invert(&self) -> Result<Self, String> {
        let determinant = self.determinant();
        if determinant == 0.0 {
            return Err("not invertible".to_string());
        }

        let mut state = [[0.0_f64; 3]; 3];

        for (w, row) in state.iter_mut().enumerate() {
            for (h, cell) in row.iter_mut().enumerate() {
                *cell = self.cofactor(w, h) / determinant;
            }
        }

        Ok(Self { state })
    }
}

impl Default for Matrix4x4 {
    fn default() -> Self {
        Self::new()
    }
}

impl Matrix4x4 {
    pub fn new() -> Matrix4x4 {
        Matrix4x4 {
            state: [[0.0_f64; 4]; 4],
        }
    }

    pub fn new_from(state: [[f64; 4]; 4]) -> Matrix4x4 {
        Matrix4x4 { state }
    }

    pub fn at(&self, w: usize, h: usize) -> f64 {
        self.state[w][h]
    }

    pub fn transpose(&self) -> Self {
        let mut state = [[0.0_f64; 4]; 4];
        for (w, row) in state.iter_mut().enumerate() {
            for (h, cell) in row.iter_mut().enumerate() {
                *cell = self.at(h, w);
            }
        }
        Self { state }
    }

    pub fn submatrix(&self, w: usize, h: usize) -> Matrix3x3 {
        let mut state = [[0.0_f64; 3]; 3];
        for (i, row) in state.iter_mut().enumerate() {
            let mut x = i;
            if i >= w {
                x += 1;
            }

            for (j, cell) in row.iter_mut().enumerate() {
                let mut y = j;
                if j >= h {
                    y += 1;
                }

                *cell = self.at(x, y);
            }
        }
        Matrix3x3 { state }
    }

    pub fn cofactor(&self, w: usize, h: usize) -> f64 {
        match (w + h) % 2 == 0 {
            true => self.submatrix(w, h).determinant(),
            false => -self.submatrix(w, h).determinant(),
        }
    }

    pub fn determinant(&self) -> f64 {
        (self.state[0][0] * self.cofactor(0, 0))
            + (self.state[0][1] * self.cofactor(0, 1))
            + (self.state[0][2] * self.cofactor(0, 2))
            + (self.state[0][3] * self.cofactor(0, 3))
    }

    pub fn invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    pub fn inverse(&self) -> Result<Self, String> {
        let determinant = self.determinant();
        if determinant == 0.0 {
            return Err("not invertible".to_string());
        }

        let mut state = [[0.0_f64; 4]; 4];

        for (w, row) in state.iter_mut().enumerate() {
            for (h, cell) in row.iter_mut().enumerate() {
                *cell = self.cofactor(h, w) / determinant;
            }
        }

        Ok(Self { state })
    }
}

pub fn identity_matrix() -> Matrix4x4 {
    let mut state = [[0.0_f64; 4]; 4];
    for (i, row) in state.iter_mut().enumerate() {
        row[i] += 1.0;
    }
    Matrix4x4 { state }
}

impl_op_ex!(*|a: &Matrix4x4, b: &Matrix4x4| -> Matrix4x4 {
    let mut state = [[0.0_f64; 4]; 4];
    for (w, row) in state.iter_mut().enumerate() {
        for (h, cell) in row.iter_mut().enumerate() {
            for i in 0..4 {
                *cell += a.at(w, i) * b.at(i, h);
            }
        }
    }
    Matrix4x4 { state }
});

impl std::cmp::PartialEq for Matrix4x4 {
    fn eq(&self, other: &Self) -> bool {
        for w in 0..4 {
            for h in 0..4 {
                let mut delta = self.at(w, h) - other.at(w, h);
                if delta < 0.0 {
                    delta = -delta;
                }
                let e = 0.0001;
                if delta > e {
                    return false;
                }
            }
        }
        true
    }
}

impl_op_ex_commutative!(*|a: &Matrix4x4, b: &Tuple| -> Tuple {
    let x = (a.at(0, 0) * b.x) + (a.at(0, 1) * b.y) + (a.at(0, 2) * b.z) + (a.at(0, 3) * b.w);
    let y = (a.at(1, 0) * b.x) + (a.at(1, 1) * b.y) + (a.at(1, 2) * b.z) + (a.at(1, 3) * b.w);
    let z = (a.at(2, 0) * b.x) + (a.at(2, 1) * b.y) + (a.at(2, 2) * b.z) + (a.at(2, 3) * b.w);
    let w = (a.at(3, 0) * b.x) + (a.at(3, 1) * b.y) + (a.at(3, 2) * b.z) + (a.at(3, 3) * b.w);

    Tuple::new(x, y, z, w)
});
