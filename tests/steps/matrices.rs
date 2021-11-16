use crate::steps::tuples::{parse_point, parse_tuple};
use crate::Matrix;
use crate::MyWorld;
use approx::assert_abs_diff_eq;
use cucumber::gherkin::Step;
use cucumber::{given, then, when};
use lab_raytracing_rs::matrices::{identity_matrix, Matrix2x2, Matrix3x3, Matrix4x4};

#[given(regex = r"^the following (2x2|3x3|4x4) matrix (A|B|M):$")]
async fn create_sized_matrix(world: &mut MyWorld, size: String, name: String, step: &Step) {
    let t = step.table.as_ref().unwrap();
    world.matrices.insert(
        name,
        match size.as_str() {
            "2x2" => Matrix::M2x2(form_vec_2x2(&t.rows)),
            "3x3" => Matrix::M3x3(form_vec_3x3(&t.rows)),
            "4x4" => Matrix::M4x4(form_vec_4x4(&t.rows)),
            _ => panic!("matrix size not covered"),
        },
    );
}

#[given(regex = r"^the following matrix (A|B):$")]
async fn create_matrix(world: &mut MyWorld, name: String, step: &Step) {
    let t = step.table.as_ref().unwrap();
    let m = form_vec_4x4(&t.rows);
    world.matrices.insert(name, Matrix::M4x4(m));
}

#[then(regex = r"^(M|B)\[([0-9]+),([0-9]+)\] = ([-0-9.]+)$")]
async fn compare_matrix_cell(world: &mut MyWorld, name: String, w: usize, h: usize, desired: f64) {
    let value = match world.matrices.get(&name).unwrap() {
        Matrix::M2x2(m) => m.at(w, h),
        Matrix::M3x3(m) => m.at(w, h),
        Matrix::M4x4(m) => m.at(w, h),
    };
    assert_abs_diff_eq!(desired, value);
}

#[then(regex = r"^(M|B)\[([0-9]+),([0-9]+)\] = ([-0-9.]+)/([0-9]+)$")]
async fn compare_matrix_cell_divisor(
    world: &mut MyWorld,
    name: String,
    w: usize,
    h: usize,
    dividend: f64,
    divisor: f64,
) {
    let desired = dividend / divisor;
    let value = match world.matrices.get(&name).unwrap() {
        Matrix::M2x2(m) => m.at(w, h),
        Matrix::M3x3(m) => m.at(w, h),
        Matrix::M4x4(m) => m.at(w, h),
    };
    assert_abs_diff_eq!(desired, value);
}

#[then(regex = r"^(_)\[([0-9]+),([0-9]+)\] = ([-0-9]+)/([0-9]+)$")]
async fn compare_4x4_matrix_cell(
    world: &mut MyWorld,
    name: String,
    w: usize,
    h: usize,
    dividend: f64,
    divisor: f64,
) {
    let desired = dividend / divisor;
    let value = match world.matrices.get(&name).unwrap() {
        Matrix::M4x4(m) => m.at(w, h),
        _ => panic!("matrix needs to be in 4x4 form"),
    };
    assert_abs_diff_eq!(desired, value);
}

#[then(regex = r"^(A) (!)?= (B)$")]
async fn compare_matrices(world: &mut MyWorld, m1: String, negation: String, m2: String) {
    let m1 = world.matrices.get(&m1).unwrap();
    let m2 = world.matrices.get(&m2).unwrap();
    let same = match (m1, m2) {
        (Matrix::M2x2(m1), Matrix::M2x2(m2)) => m1 == m2,
        (Matrix::M3x3(m1), Matrix::M3x3(m2)) => m1 == m2,
        (Matrix::M4x4(m1), Matrix::M4x4(m2)) => m1 == m2,
        _ => panic!("matrix 1 is of different type then matrix 2"),
    };
    match negation.as_str() {
        "!" => assert!(!same),
        _ => assert!(same),
    };
}

#[then(regex = r"^(A) \* (B) is the following 4x4 matrix:$")]
async fn multiply_matrix(world: &mut MyWorld, m1: String, m2: String, step: &Step) {
    let m1 = match world.matrices.get(&m1).unwrap() {
        Matrix::M4x4(m) => m,
        _ => panic!("matrix needs to be in 4x4 form"),
    };
    let m2 = match world.matrices.get(&m2).unwrap() {
        Matrix::M4x4(m) => m,
        _ => panic!("matrix needs to be in 4x4 form"),
    };
    let t = step.table.as_ref().unwrap();
    let desired = form_vec_4x4(&t.rows);
    let computed = m1 * m2;
    assert_eq!(desired, computed);
}

#[then(regex = r"^(B|t) is the following 4x4 matrix:$")]
async fn compare_matrix(world: &mut MyWorld, name: String, step: &Step) {
    let matrix = match world.matrices.get(&name).unwrap() {
        Matrix::M4x4(m) => m,
        _ => panic!("matrix needs to be in 4x4 form"),
    };
    let t = step.table.as_ref().unwrap();
    let desired = form_vec_4x4(&t.rows);
    assert!(eq_matrix4x4_similar(&desired, matrix));
}

#[given(regex = r"^(C) ← (A) \* (B)$")]
async fn assign_mulitplied_matrix(
    world: &mut MyWorld,
    target: String,
    name_1: String,
    name_2: String,
) {
    let m1 = match world.matrices.get(&name_1).unwrap() {
        Matrix::M4x4(m) => m,
        _ => panic!("matrix needs to be in 4x4 form"),
    };
    let m2 = match world.matrices.get(&name_2).unwrap() {
        Matrix::M4x4(m) => m,
        _ => panic!("matrix needs to be in 4x4 form"),
    };
    let computed = Matrix::M4x4(m1 * m2);
    world.matrices.insert(target, computed);
}

#[when(regex = r"^(T) ← (C) \* (B) \* (A)$")]
async fn assign_three_mulitplied_matrix(
    world: &mut MyWorld,
    target: String,
    name_1: String,
    name_2: String,
    name_3: String,
) {
    let m1 = match world.matrices.get(&name_1).unwrap() {
        Matrix::M4x4(m) => m,
        _ => panic!("matrix needs to be in 4x4 form"),
    };
    let m2 = match world.matrices.get(&name_2).unwrap() {
        Matrix::M4x4(m) => m,
        _ => panic!("matrix needs to be in 4x4 form"),
    };
    let m3 = match world.matrices.get(&name_3).unwrap() {
        Matrix::M4x4(m) => m,
        _ => panic!("matrix needs to be in 4x4 form"),
    };
    let computed = Matrix::M4x4(m1 * m2 * m3);
    world.matrices.insert(target, computed);
}

#[then(regex = r"^(A) \* (b) = tuple\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$")]
async fn multiply_matrix_tuple(
    world: &mut MyWorld,
    matrix: String,
    tuple: String,
    x: String,
    y: String,
    z: String,
    w: String,
) {
    let matrix = match world.matrices.get(&matrix).unwrap() {
        Matrix::M4x4(matrix) => matrix,
        _ => panic!("matrix needs to be in 4x4 form"),
    };
    let tuple = world.tuples.get(&tuple).unwrap();
    let computed = matrix * tuple;
    let desired = parse_tuple(&[x, y, z, w]);
    assert_eq!(desired, computed);
}

#[then(regex = r"^(T) \* (p) = point\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$")]
async fn multiply_matrix_point(
    world: &mut MyWorld,
    matrix: String,
    tuple: String,
    x: String,
    y: String,
    z: String,
) {
    let matrix = match world.matrices.get(&matrix).unwrap() {
        Matrix::M4x4(m) => m,
        _ => panic!("matrix needs to be in 4x4 form"),
    };
    let tuple = world.tuples.get(&tuple).unwrap();
    let computed = matrix * tuple;
    let desired = parse_point(&[x, y, z]);
    assert_eq!(desired, computed);
}

#[then(regex = r"^(A) \* identity_matrix = (A)$")]
async fn multiply_matrix_identity_matrix(world: &mut MyWorld, matrix_a: String, matrix_b: String) {
    let matrix = match world.matrices.get(&matrix_a).unwrap() {
        Matrix::M4x4(m) => m,
        _ => panic!("matrix needs to be in 4x4 form"),
    };
    let desired = match world.matrices.get(&matrix_b).unwrap() {
        Matrix::M4x4(m) => m,
        _ => panic!("matrix needs to be in 4x4 form"),
    };
    let computed = matrix * identity_matrix();
    assert_eq!(desired, &computed);
}

#[then(regex = r"^inverse\((A)\) is the following 4x4 matrix:$")]
async fn inverse_matrix(world: &mut MyWorld, matrix: String, step: &Step) {
    let matrix = match world.matrices.get(&matrix).unwrap() {
        Matrix::M4x4(m) => m,
        _ => panic!("matrix needs to be in 4x4 form"),
    };
    let t = step.table.as_ref().unwrap();
    let desired = form_vec_4x4(&t.rows);
    let computed = matrix.inverse().unwrap();
    assert!(eq_matrix4x4_similar(&computed, &desired));
}

#[given(regex = r"^(B) ← inverse\((A)\)$")]
async fn assign_inverse_matrix(world: &mut MyWorld, target: String, matrix: String) {
    let matrix = match world.matrices.get(&matrix).unwrap() {
        Matrix::M4x4(m) => m,
        _ => panic!("matrix needs to be in 4x4 form"),
    };
    let inverse = matrix.inverse().unwrap();
    world.matrices.insert(target, Matrix::M4x4(inverse));
}

#[then(regex = r"^(C) \* inverse\((B)\) = (A)$")]
async fn compare_multiply_with_inverse_matrix(
    world: &mut MyWorld,
    matrix_a: String,
    matrix_b: String,
    desired: String,
) {
    let m1 = match world.matrices.get(&matrix_a).unwrap() {
        Matrix::M4x4(m) => m,
        _ => panic!("matrix needs to be in 4x4 form"),
    };
    let m2 = match world.matrices.get(&matrix_b).unwrap() {
        Matrix::M4x4(m) => m,
        _ => panic!("matrix needs to be in 4x4 form"),
    };
    let computed = m1 * m2.inverse().unwrap();
    let desired = match world.matrices.get(&desired).unwrap() {
        Matrix::M4x4(m) => m,
        _ => panic!("matrix needs to be in 4x4 form"),
    };
    assert!(eq_matrix4x4_similar(&computed, desired));
}

#[then(regex = r"^identity_matrix \* (a) = (a)$")]
async fn multiply_tuple_identity_matrix(world: &mut MyWorld, tuple: String, desired: String) {
    let tuple = world.tuples.get(&tuple).unwrap();
    let desired = world.tuples.get(&desired).unwrap();
    let computed = tuple * identity_matrix();
    assert_eq!(desired, &computed);
}

#[then(regex = r"^transpose\((A)\) is the following matrix:$")]
async fn compare_transpose_matrix(world: &mut MyWorld, matrix: String, step: &Step) {
    let matrix = match world.matrices.get(&matrix).unwrap() {
        Matrix::M4x4(m) => m,
        _ => panic!("matrix needs to be in 4x4 form"),
    };
    let t = step.table.as_ref().unwrap();
    let desired = form_vec_4x4(&t.rows);
    let computed = matrix.transpose();
    assert_eq!(desired, computed);
}

#[given(regex = r"^(A) ← transpose\(identity_matrix\)$")]
async fn assign_transposed_identity_matrix(world: &mut MyWorld, target: String) {
    let computed = Matrix::M4x4(identity_matrix().transpose());
    world.matrices.insert(target, computed);
}

#[given(regex = r"^(B) ← submatrix\((A), ([0-9]+), ([0-9]+)\)$")]
async fn assign_submatrix_matrix(
    world: &mut MyWorld,
    target: String,
    matrix: String,
    w: usize,
    h: usize,
) {
    let matrix = match world.matrices.get(&matrix).unwrap() {
        Matrix::M4x4(m) => Matrix::M3x3(m.submatrix(w, h)),
        Matrix::M3x3(m) => Matrix::M2x2(m.submatrix(w, h)),
        _ => panic!("matrix needs to be in 4x4 or 3x3 form"),
    };
    world.matrices.insert(target, matrix);
}

#[then(regex = r"^(A|t) = identity_matrix$")]
async fn compare_identity_matrix(world: &mut MyWorld, matrix: String) {
    let matrix = world.get4x4(&matrix);
    let desired = identity_matrix();
    assert_eq!(matrix, &desired);
}

#[then(regex = r"^minor\((A), ([0-9]+), ([0-9]+)\) = ([-0-9.]+)$")]
async fn compare_minor(world: &mut MyWorld, matrix: String, w: usize, h: usize, desired: f64) {
    let minor = match world.matrices.get(&matrix).unwrap() {
        Matrix::M3x3(m) => m.minor(w, h),
        _ => panic!("matrix needs to be in 3x3 form"),
    };
    assert_abs_diff_eq!(minor, desired);
}

#[then(regex = r"^cofactor\((A), ([0-9]+), ([0-9]+)\) = ([-0-9.]+)$")]
async fn compare_cofactor(world: &mut MyWorld, matrix: String, w: usize, h: usize, desired: f64) {
    let cofactor = match world.matrices.get(&matrix).unwrap() {
        Matrix::M3x3(m) => m.cofactor(w, h),
        Matrix::M4x4(m) => m.cofactor(w, h),
        _ => panic!("matrix needs to be in 3x3 or 4x4 form"),
    };
    assert_abs_diff_eq!(cofactor, desired);
}

#[then(regex = r"^determinant\((A|B)\) = ([-0-9.]+)$")]
async fn compare_determinant(world: &mut MyWorld, matrix: String, desired: f64) {
    let determinant = match world.matrices.get(&matrix).unwrap() {
        Matrix::M2x2(m) => m.determinant(),
        Matrix::M3x3(m) => m.determinant(),
        Matrix::M4x4(m) => m.determinant(),
    };
    assert_abs_diff_eq!(desired, determinant);
}

#[then(regex = r"^(A) is (not )?invertible$")]
async fn is_invertible(world: &mut MyWorld, matrix: String, invert: String) {
    let invertible = match world.matrices.get(&matrix).unwrap() {
        Matrix::M2x2(m) => m.invertible(),
        Matrix::M3x3(m) => m.invertible(),
        Matrix::M4x4(m) => m.invertible(),
    };
    assert!(match invert.as_str() {
        "not " => !invertible,
        _ => invertible,
    });
}

#[then(regex = r"^submatrix\((A), ([0-9]+), ([0-9]+)\) is the following (2x2|3x3) matrix:$")]
async fn compare_submatrix(
    world: &mut MyWorld,
    matrix: String,
    w: usize,
    h: usize,
    size: String,
    step: &Step,
) {
    let same = match size.as_str() {
        "2x2" => {
            form_vec_2x2(&step.table.as_ref().unwrap().rows)
                == match world.matrices.get(&matrix).unwrap() {
                    Matrix::M3x3(m) => m.submatrix(w, h),
                    _ => panic!("matrix needs to be in 4x4 form"),
                }
        }
        "3x3" => {
            form_vec_3x3(&step.table.as_ref().unwrap().rows)
                == match world.matrices.get(&matrix).unwrap() {
                    Matrix::M4x4(m) => m.submatrix(w, h),
                    _ => panic!("matrix needs to be in 4x4 form"),
                }
        }
        _ => panic!("matrix dimentions not covered"),
    };
    assert!(same);
}

fn form_vec_2x2(v: &[Vec<String>]) -> Matrix2x2 {
    let mut state = [[0.0_f64; 2]; 2];
    for (w, row) in v.iter().enumerate() {
        for (h, e) in row.iter().enumerate() {
            state[w][h] = e.parse::<f64>().unwrap();
        }
    }
    Matrix2x2::new_from(state)
}

fn form_vec_3x3(v: &[Vec<String>]) -> Matrix3x3 {
    let mut state = [[0.0_f64; 3]; 3];
    for (w, row) in v.iter().enumerate() {
        for (h, e) in row.iter().enumerate() {
            state[w][h] = e.parse::<f64>().unwrap();
        }
    }
    Matrix3x3::new_from(state)
}

fn form_vec_4x4(v: &[Vec<String>]) -> Matrix4x4 {
    let mut state = [[0.0_f64; 4]; 4];
    for (w, row) in v.iter().enumerate() {
        for (h, e) in row.iter().enumerate() {
            state[w][h] = e.parse::<f64>().unwrap();
        }
    }
    Matrix4x4::new_from(state)
}

fn eq_matrix4x4_similar(matrix: &Matrix4x4, other: &Matrix4x4) -> bool {
    for w in 0..4 {
        for h in 0..4 {
            if (matrix.at(w, h) - other.at(w, h)).abs() > 0.0001 {
                return false;
            }
        }
    }
    true
}
