use crate::{
    matrices::Matrix4x4,
    tuples::{cross, Tuple},
};

pub fn translation(x: f64, y: f64, z: f64) -> Matrix4x4 {
    let mut state = [[0.0_f64; 4]; 4];
    for (i, row) in state.iter_mut().enumerate() {
        row[i] += 1.0;
    }
    state[0][3] = x;
    state[1][3] = y;
    state[2][3] = z;
    Matrix4x4::new_from(state)
}

pub fn scaling(x: f64, y: f64, z: f64) -> Matrix4x4 {
    let mut state = [[0.0_f64; 4]; 4];
    state[0][0] = x;
    state[1][1] = y;
    state[2][2] = z;
    state[3][3] = 1.0;
    Matrix4x4::new_from(state)
}

pub fn rotation_x(r: f64) -> Matrix4x4 {
    let mut state = [[0.0_f64; 4]; 4];
    state[0][0] = 1.0;
    state[3][3] = 1.0;
    state[1][1] = r.cos();
    state[1][2] = -r.sin();
    state[2][1] = r.sin();
    state[2][2] = r.cos();
    Matrix4x4::new_from(state)
}

pub fn rotation_y(r: f64) -> Matrix4x4 {
    let mut state = [[0.0_f64; 4]; 4];
    state[1][1] = 1.0;
    state[3][3] = 1.0;
    state[0][0] = r.cos();
    state[0][2] = r.sin();
    state[2][0] = -r.sin();
    state[2][2] = r.cos();
    Matrix4x4::new_from(state)
}

pub fn rotation_z(r: f64) -> Matrix4x4 {
    let mut state = [[0.0_f64; 4]; 4];
    state[2][2] = 1.0;
    state[3][3] = 1.0;
    state[0][0] = r.cos();
    state[0][1] = -r.sin();
    state[1][0] = r.sin();
    state[1][1] = r.cos();
    Matrix4x4::new_from(state)
}

pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix4x4 {
    let mut state = [[0.0_f64; 4]; 4];
    state[0][0] = 1.0;
    state[1][1] = 1.0;
    state[2][2] = 1.0;
    state[3][3] = 1.0;
    state[0][1] = xy;
    state[0][2] = xz;
    state[1][0] = yx;
    state[1][2] = yz;
    state[2][0] = zx;
    state[2][1] = zy;
    Matrix4x4::new_from(state)
}

pub fn view_transform(from: &Tuple, to: &Tuple, up: &Tuple) -> Matrix4x4 {
    let forward = (to - from).normalize();
    let upn = up.normalize();
    let left = cross(&forward, &upn);
    let true_up = cross(&left, &forward);

    let mut state = [[0.0_f64; 4]; 4];
    state[0][0] = left.x;
    state[0][1] = left.y;
    state[0][2] = left.z;
    state[1][0] = true_up.x;
    state[1][1] = true_up.y;
    state[1][2] = true_up.z;
    state[2][0] = -forward.x;
    state[2][1] = -forward.y;
    state[2][2] = -forward.z;
    state[3][3] = 1.0;
    let orientation = Matrix4x4::new_from(state);

    orientation * translation(-from.x, -from.y, -from.z)
}
