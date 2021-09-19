use crate::Matrix;
use approx::assert_abs_diff_eq;
use cucumber_rust::Steps;
use lab_raytracing_rs::matrices::{identity_matrix, Matrix2x2, Matrix3x3, Matrix4x4};
use lab_raytracing_rs::tuples::Tuple;

use crate::MyWorld;

pub fn steps() -> Steps<MyWorld> {
    let mut steps: Steps<MyWorld> = Steps::new();

    steps.given_regex(
        r#"^the following 2x2 matrix ([A-Z]+):$"#,
        |mut world, ctx| {
            let name = ctx.matches[1].clone();
            let t = ctx.step.table.as_ref().unwrap();
            let m = form_vec_2x2(&t.rows);
            world.matrices.insert(name, Matrix::M2x2(m));

            world
        },
    );

    steps.given_regex(
        r#"^the following 3x3 matrix ([A-Z]+):$"#,
        |mut world, ctx| {
            let name = ctx.matches[1].clone();
            let t = ctx.step.table.as_ref().unwrap();
            let m = form_vec_3x3(&t.rows);
            world.matrices.insert(name, Matrix::M3x3(m));

            world
        },
    );

    steps.given_regex(
        r#"^the following 4x4 matrix ([A-Z]+):$"#,
        |mut world, ctx| {
            let name = ctx.matches[1].clone();
            let t = ctx.step.table.as_ref().unwrap();
            let m = form_vec_4x4(&t.rows);
            world.matrices.insert(name, Matrix::M4x4(m));

            world
        },
    );

    steps.given_regex(r#"^the following matrix ([A-Z]+):$"#, |mut world, ctx| {
        let name = ctx.matches[1].clone();
        let t = ctx.step.table.as_ref().unwrap();
        let m = form_vec_4x4(&t.rows);
        world.matrices.insert(name, Matrix::M4x4(m));

        world
    });

    steps.then_regex(
        r#"^([A-Z]+)\[([0-9]+),([0-9]+)\] = ([-0-9.]+)$"#,
        |world, ctx| {
            let name = ctx.matches[1].clone();
            let w = ctx.matches[2].parse::<usize>().unwrap();
            let h = ctx.matches[3].parse::<usize>().unwrap();
            let desired = ctx.matches[4].parse::<f64>().unwrap();
            let m = world.matrices.get(&name).unwrap();
            let v = match m {
                Matrix::M2x2(m) => m.at(w, h),
                Matrix::M3x3(m) => m.at(w, h),
                Matrix::M4x4(m) => m.at(w, h),
            };

            assert_abs_diff_eq!(desired, v);

            world
        },
    );

    steps.then_regex(r#"^([A-Z]+) = ([A-Z]+)$"#, |world, ctx| {
        let name1 = ctx.matches[1].clone();
        let name2 = ctx.matches[2].clone();
        let m1 = world.matrices.get(&name1).unwrap();
        let m2 = world.matrices.get(&name2).unwrap();
        match (m1, m2) {
            (Matrix::M2x2(m1), Matrix::M2x2(m2)) => assert!(m1 == m2),
            (Matrix::M3x3(m1), Matrix::M3x3(m2)) => assert!(m1 == m2),
            (Matrix::M4x4(m1), Matrix::M4x4(m2)) => assert!(m1 == m2),
            _ => panic!("matrix 1 of different type then matrix 2"),
        };

        world
    });

    steps.then_regex(r#"^([A-Z]+) != ([A-Z]+)$"#, |world, ctx| {
        let name1 = ctx.matches[1].clone();
        let name2 = ctx.matches[2].clone();
        let m1 = world.matrices.get(&name1).unwrap();
        let m2 = world.matrices.get(&name2).unwrap();
        match (m1, m2) {
            (Matrix::M2x2(m1), Matrix::M2x2(m2)) => assert!(m1 != m2),
            (Matrix::M3x3(m1), Matrix::M3x3(m2)) => assert!(m1 != m2),
            (Matrix::M4x4(m1), Matrix::M4x4(m2)) => assert!(m1 != m2),
            _ => panic!("matrix 1 of different type then matrix 2"),
        };

        world
    });

    steps.then_regex(
        r#"^([A-Z]+) \* ([A-Z]+) is the following 4x4 matrix:$"#,
        |world, ctx| {
            let name1 = ctx.matches[1].clone();
            let name2 = ctx.matches[2].clone();
            let m1 = match world.matrices.get(&name1).unwrap() {
                Matrix::M4x4(m) => m,
                _ => panic!("matrix needs to be in 4x4 form"),
            };
            let m2 = match world.matrices.get(&name2).unwrap() {
                Matrix::M4x4(m) => m,
                _ => panic!("matrix needs to be in 4x4 form"),
            };

            let t = ctx.step.table.as_ref().unwrap();
            let m = form_vec_4x4(&t.rows);

            assert_eq!(m, m1 * m2);

            world
        },
    );

    steps.then_regex(
        r#"^([A-Z]+) is the following 4x4 matrix:$"#,
        |world, ctx| {
            let name1 = ctx.matches[1].clone();
            let m1 = match world.matrices.get(&name1).unwrap() {
                Matrix::M4x4(m) => m,
                _ => panic!("matrix needs to be in 4x4 form"),
            };

            let t = ctx.step.table.as_ref().unwrap();
            let m = form_vec_4x4(&t.rows);

            assert_eq!(&m, m1);

            world
        },
    );

    steps.given_regex(
        r#"^([A-Z]+) ← ([A-Z]+) \* ([A-Z]+)$"#,
        |mut world, ctx| {
            let name1 = ctx.matches[2].clone();
            let name2 = ctx.matches[3].clone();
            let name3 = ctx.matches[1].clone();
            let m1 = match world.matrices.get(&name1).unwrap() {
                Matrix::M4x4(m) => m,
                _ => panic!("matrix needs to be in 4x4 form"),
            };
            let m2 = match world.matrices.get(&name2).unwrap() {
                Matrix::M4x4(m) => m,
                _ => panic!("matrix needs to be in 4x4 form"),
            };

            let matrix = m1 * m2;

            world.matrices.insert(name3, Matrix::M4x4(matrix));

            world
        },
    );

    steps.then_regex(
        r#"^([A-Z]+) \* ([a-z]+) = tuple\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |world, ctx| {
            let matrix_name = ctx.matches[1].clone();
            let matrix = match world.matrices.get(&matrix_name).unwrap() {
                Matrix::M4x4(m) => m,
                _ => panic!("matrix needs to be in 4x4 form"),
            };

            let tuple_name = ctx.matches[2].clone();
            let tuple = world.tuples.get(&tuple_name).unwrap();

            let x = ctx.matches[3].parse::<f64>().unwrap();
            let y = ctx.matches[4].parse::<f64>().unwrap();
            let z = ctx.matches[5].parse::<f64>().unwrap();
            let w = ctx.matches[6].parse::<f64>().unwrap();
            let desired = Tuple::new(x, y, z, w);

            assert_eq!(desired, matrix * tuple);

            world
        },
    );

    steps.then_regex(
        r#"^([A-Z]+) \* identity_matrix = ([A-Z]+)$"#,
        |world, ctx| {
            let matrix_name = ctx.matches[1].clone();
            let matrix = match world.matrices.get(&matrix_name).unwrap() {
                Matrix::M4x4(m) => m,
                _ => panic!("matrix needs to be in 4x4 form"),
            };

            let desired_name = ctx.matches[2].clone();
            let desired_matrix = match world.matrices.get(&desired_name).unwrap() {
                Matrix::M4x4(m) => m,
                _ => panic!("matrix needs to be in 4x4 form"),
            };

            assert_eq!(desired_matrix, &(matrix * identity_matrix()));

            world
        },
    );

    steps.then_regex(
        r#"^inverse\(([A-Z]+)\) is the following 4x4 matrix:$"#,
        |world, ctx| {
            let name1 = ctx.matches[1].clone();
            let matrix = match world.matrices.get(&name1).unwrap() {
                Matrix::M4x4(m) => m,
                _ => panic!("matrix needs to be in 4x4 form"),
            };
            let t = ctx.step.table.as_ref().unwrap();
            let desired_matrix = form_vec_4x4(&t.rows);

            let inverted_matrix = matrix.invert().unwrap();

            assert_eq!(desired_matrix, inverted_matrix);

            world
        },
    );

    steps.given_regex(r#"^([A-Z]+) ← inverse\(([A-Z]+)\)$"#, |mut world, ctx| {
        let name1 = ctx.matches[1].clone();
        let name2 = ctx.matches[2].clone();
        let matrix = match world.matrices.get(&name2).unwrap() {
            Matrix::M4x4(m) => m.invert().unwrap(),
            _ => panic!("matrix needs to be in 4x4 form"),
        };

        world.matrices.insert(name1, Matrix::M4x4(matrix));

        world
    });

    steps.then_regex(
        r#"^([A-Z]+)\[([0-9]+),([0-9]+)\] = ([-0-9]+)/([0-9]+)$"#,
        |world, ctx| {
            let name1 = ctx.matches[1].clone();
            let w = ctx.matches[2].parse::<usize>().unwrap();
            let h = ctx.matches[3].parse::<usize>().unwrap();
            let dividend = ctx.matches[4].parse::<f64>().unwrap();
            let divisor = ctx.matches[5].parse::<f64>().unwrap();
            let matrix = match world.matrices.get(&name1).unwrap() {
                Matrix::M4x4(m) => m,
                _ => panic!("matrix needs to be in 4x4 form"),
            };
            let desired = dividend / divisor;
            let value = matrix.at(w, h);

            assert_abs_diff_eq!(desired, value);

            world
        },
    );

    steps.then_regex(
        r#"^([A-Z]+) \* inverse\(([A-Z]+)\) = ([A-Z]+)$"#,
        |world, ctx| {
            let name1 = ctx.matches[1].clone();
            let name2 = ctx.matches[2].clone();
            let name3 = ctx.matches[3].clone();
            let m1 = match world.matrices.get(&name1).unwrap() {
                Matrix::M4x4(m) => m,
                _ => panic!("matrix needs to be in 4x4 form"),
            };
            let m2 = match world.matrices.get(&name2).unwrap() {
                Matrix::M4x4(m) => m,
                _ => panic!("matrix needs to be in 4x4 form"),
            };
            let desired_matrix = match world.matrices.get(&name3).unwrap() {
                Matrix::M4x4(m) => m,
                _ => panic!("matrix needs to be in 4x4 form"),
            };

            let matrix = m1 * m2.invert().unwrap();

            assert_eq!(desired_matrix, &matrix);

            world
        },
    );

    steps.then_regex(
        r#"^identity_matrix \* ([a-z]+) = ([a-z]+)$"#,
        |world, ctx| {
            let tuple_name = ctx.matches[1].clone();
            let tuple = world.tuples.get(&tuple_name).unwrap();

            let desired_name = ctx.matches[2].clone();
            let desired_tuple = world.tuples.get(&desired_name).unwrap();

            assert_eq!(desired_tuple, &(tuple * identity_matrix()));

            world
        },
    );

    steps.then_regex(
        r#"^transpose\(([A-Z]+)\) is the following matrix:$"#,
        |world, ctx| {
            let matrix_name = ctx.matches[1].clone();
            let matrix = match world.matrices.get(&matrix_name).unwrap() {
                Matrix::M4x4(m) => m,
                _ => panic!("matrix needs to be in 4x4 form"),
            };

            let t = ctx.step.table.as_ref().unwrap();
            let desired_matrix = form_vec_4x4(&t.rows);

            assert_eq!(desired_matrix, matrix.transpose());

            world
        },
    );

    steps.given_regex(
        r#"^([A-Z]+) ← transpose\(identity_matrix\)$"#,
        |mut world, ctx| {
            let name = ctx.matches[1].clone();
            let m = identity_matrix().transpose();
            world.matrices.insert(name, Matrix::M4x4(m));

            world
        },
    );

    steps.given_regex(
        r#"^([A-Z]+) ← submatrix\(([A-Z]+), ([0-9]+), ([0-9]+)\)$"#,
        |mut world, ctx| {
            let target_name = ctx.matches[1].clone();
            let matrix_name = ctx.matches[2].clone();
            let w = ctx.matches[3].parse::<usize>().unwrap();
            let h = ctx.matches[4].parse::<usize>().unwrap();
            let matrix = match world.matrices.get(&matrix_name).unwrap() {
                Matrix::M4x4(m) => Matrix::M3x3(m.submatrix(w, h)),
                Matrix::M3x3(m) => Matrix::M2x2(m.submatrix(w, h)),
                _ => panic!("matrix needs to be in 4x4 or 3x3 form"),
            };
            world.matrices.insert(target_name, matrix);

            world
        },
    );

    steps.then_regex(r#"^([A-Z]+) = identity_matrix$"#, |world, ctx| {
        let matrix_name = ctx.matches[1].clone();
        let matrix = match world.matrices.get(&matrix_name).unwrap() {
            Matrix::M4x4(m) => m,
            _ => panic!("matrix needs to be in 4x4 form"),
        };

        assert_eq!(matrix, &identity_matrix());

        world
    });

    steps.then_regex(
        r#"^minor\(([A-Z]+), ([0-9]+), ([0-9]+)\) = ([-0-9.]+)$"#,
        |world, ctx| {
            let matrix_name = ctx.matches[1].clone();
            let w = ctx.matches[2].parse::<usize>().unwrap();
            let h = ctx.matches[3].parse::<usize>().unwrap();
            let desired_minor = ctx.matches[4].parse::<f64>().unwrap();
            let minor = match world.matrices.get(&matrix_name).unwrap() {
                Matrix::M3x3(m) => m.minor(w, h),
                _ => panic!("matrix needs to be in 3x3 form"),
            };

            assert_abs_diff_eq!(desired_minor, minor);

            world
        },
    );

    steps.then_regex(
        r#"^cofactor\(([A-Z]+), ([0-9]+), ([0-9]+)\) = ([-0-9.]+)$"#,
        |world, ctx| {
            let matrix_name = ctx.matches[1].clone();
            let w = ctx.matches[2].parse::<usize>().unwrap();
            let h = ctx.matches[3].parse::<usize>().unwrap();
            let desired_minor = ctx.matches[4].parse::<f64>().unwrap();
            let minor = match world.matrices.get(&matrix_name).unwrap() {
                Matrix::M3x3(m) => m.cofactor(w, h),
                Matrix::M4x4(m) => m.cofactor(w, h),
                _ => panic!("matrix needs to be in 3x3 or 4x4 form"),
            };

            assert_abs_diff_eq!(desired_minor, minor);

            world
        },
    );

    steps.then_regex(r#"^determinant\(([A-Z]+)\) = ([-0-9.]+)$"#, |world, ctx| {
        let matrix_name = ctx.matches[1].clone();
        let determinant = match world.matrices.get(&matrix_name).unwrap() {
            Matrix::M2x2(m) => m.determinant(),
            Matrix::M3x3(m) => m.determinant(),
            Matrix::M4x4(m) => m.determinant(),
        };
        let desired_determinant = ctx.matches[2].parse::<f64>().unwrap();

        assert_abs_diff_eq!(desired_determinant, determinant);

        world
    });

    steps.then_regex(r#"^([A-Z]+) is not invertible$"#, |world, ctx| {
        let matrix_name = ctx.matches[1].clone();
        let invertible = match world.matrices.get(&matrix_name).unwrap() {
            Matrix::M2x2(m) => m.invertible(),
            Matrix::M3x3(m) => m.invertible(),
            Matrix::M4x4(m) => m.invertible(),
        };

        assert!(!invertible);

        world
    });

    steps.then_regex(r#"^([A-Z]+) is invertible$"#, |world, ctx| {
        let matrix_name = ctx.matches[1].clone();
        let invertible = match world.matrices.get(&matrix_name).unwrap() {
            Matrix::M2x2(m) => m.invertible(),
            Matrix::M3x3(m) => m.invertible(),
            Matrix::M4x4(m) => m.invertible(),
        };

        assert!(invertible);

        world
    });

    steps.then_regex(
        r#"^submatrix\(([A-Z]+), ([0-9]+), ([0-9]+)\) is the following 2x2 matrix:$"#,
        |world, ctx| {
            let matrix_name = ctx.matches[1].clone();
            let matrix = match world.matrices.get(&matrix_name).unwrap() {
                Matrix::M3x3(m) => m,
                _ => panic!("matrix needs to be in 3x3 form"),
            };
            let w = ctx.matches[2].parse::<usize>().unwrap();
            let h = ctx.matches[3].parse::<usize>().unwrap();
            let submatrix = matrix.submatrix(w, h);

            let t = ctx.step.table.as_ref().unwrap();
            let desired_matrix = form_vec_2x2(&t.rows);

            assert_eq!(desired_matrix, submatrix);

            world
        },
    );

    steps.then_regex(
        r#"^submatrix\(([A-Z]+), ([0-9]+), ([0-9]+)\) is the following 3x3 matrix:$"#,
        |world, ctx| {
            let matrix_name = ctx.matches[1].clone();
            let matrix = match world.matrices.get(&matrix_name).unwrap() {
                Matrix::M4x4(m) => m,
                _ => panic!("matrix needs to be in 4x4 form"),
            };
            let w = ctx.matches[2].parse::<usize>().unwrap();
            let h = ctx.matches[3].parse::<usize>().unwrap();
            let submatrix = matrix.submatrix(w, h);

            let t = ctx.step.table.as_ref().unwrap();
            let desired_matrix = form_vec_3x3(&t.rows);

            assert_eq!(desired_matrix, submatrix);

            world
        },
    );

    steps
}

pub fn form_vec_2x2(v: &[Vec<String>]) -> Matrix2x2 {
    let mut state = [[0.0_f64; 2]; 2];
    for (w, row) in v.iter().enumerate() {
        for (h, e) in row.iter().enumerate() {
            state[w][h] = e.parse::<f64>().unwrap();
        }
    }
    Matrix2x2::new_from(state)
}

pub fn form_vec_3x3(v: &[Vec<String>]) -> Matrix3x3 {
    let mut state = [[0.0_f64; 3]; 3];
    for (w, row) in v.iter().enumerate() {
        for (h, e) in row.iter().enumerate() {
            state[w][h] = e.parse::<f64>().unwrap();
        }
    }
    Matrix3x3::new_from(state)
}

pub fn form_vec_4x4(v: &[Vec<String>]) -> Matrix4x4 {
    let mut state = [[0.0_f64; 4]; 4];
    for (w, row) in v.iter().enumerate() {
        for (h, e) in row.iter().enumerate() {
            state[w][h] = e.parse::<f64>().unwrap();
        }
    }
    Matrix4x4::new_from(state)
}
