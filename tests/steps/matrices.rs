use crate::steps::tuples::{parse_point, parse_tuple};
use crate::Matrix;
use crate::MyWorld;
use approx::assert_abs_diff_eq;
use cucumber_rust::Steps;
use lab_raytracing_rs::matrices::{identity_matrix, Matrix2x2, Matrix3x3, Matrix4x4};

pub fn steps() -> Steps<MyWorld> {
    let mut steps: Steps<MyWorld> = Steps::new();

    steps.given_regex(
        r#"^the following (2x2|3x3|4x4) matrix (A|B|M):$"#,
        |mut world, ctx| {
            let t = ctx.step.table.as_ref().unwrap();
            world.matrices.insert(
                ctx.matches[2].clone(),
                match ctx.matches[1].as_str() {
                    "2x2" => Matrix::M2x2(form_vec_2x2(&t.rows)),
                    "3x3" => Matrix::M3x3(form_vec_3x3(&t.rows)),
                    "4x4" => Matrix::M4x4(form_vec_4x4(&t.rows)),
                    _ => panic!("matrix size not covered"),
                },
            );

            world
        },
    );

    steps.given_regex(r#"^the following matrix (A|B):$"#, |mut world, ctx| {
        let t = ctx.step.table.as_ref().unwrap();
        let m = form_vec_4x4(&t.rows);
        world
            .matrices
            .insert(ctx.matches[1].clone(), Matrix::M4x4(m));

        world
    });

    steps.then_regex(
        r#"^(M|B)\[([0-9]+),([0-9]+)\] = ([-0-9.]+)/?([0-9]+)?$"#,
        |world, ctx| {
            let w = ctx.matches[2].parse::<usize>().unwrap();
            let h = ctx.matches[3].parse::<usize>().unwrap();
            let mut desired = ctx.matches[4].parse::<f64>().unwrap();
            if !ctx.matches[5].is_empty() {
                desired /= ctx.matches[5].parse::<f64>().unwrap();
            }
            let value = match world.matrices.get(&ctx.matches[1]).unwrap() {
                Matrix::M2x2(m) => m.at(w, h),
                Matrix::M3x3(m) => m.at(w, h),
                Matrix::M4x4(m) => m.at(w, h),
            };
            assert_abs_diff_eq!(desired, value);
            world
        },
    );
    steps.then_regex(
        r#"^(_)\[([0-9]+),([0-9]+)\] = ([-0-9]+)/([0-9]+)$"#,
        |world, ctx| {
            let w = ctx.matches[2].parse::<usize>().unwrap();
            let h = ctx.matches[3].parse::<usize>().unwrap();
            let dividend = ctx.matches[4].parse::<f64>().unwrap();
            let divisor = ctx.matches[5].parse::<f64>().unwrap();
            let matrix = match world.matrices.get(&ctx.matches[1]).unwrap() {
                Matrix::M4x4(m) => m,
                _ => panic!("matrix needs to be in 4x4 form"),
            };
            let desired = dividend / divisor;
            let value = matrix.at(w, h);

            assert_abs_diff_eq!(desired, value);

            world
        },
    );

    steps.then_regex(r#"^(A) (!)?= (B)$"#, |world, ctx| {
        let m1 = world.matrices.get(&ctx.matches[1]).unwrap();
        let m2 = world.matrices.get(&ctx.matches[3]).unwrap();
        let same = match (m1, m2) {
            (Matrix::M2x2(m1), Matrix::M2x2(m2)) => m1 == m2,
            (Matrix::M3x3(m1), Matrix::M3x3(m2)) => m1 == m2,
            (Matrix::M4x4(m1), Matrix::M4x4(m2)) => m1 == m2,
            _ => panic!("matrix 1 is of different type then matrix 2"),
        };
        match ctx.matches[2].as_str() {
            "!" => assert!(!same),
            _ => assert!(same),
        };
        world
    });

    steps.then_regex(
        r#"^(A) \* (B) is the following 4x4 matrix:$"#,
        |world, ctx| {
            let m1 = match world.matrices.get(&ctx.matches[1]).unwrap() {
                Matrix::M4x4(m) => m,
                _ => panic!("matrix needs to be in 4x4 form"),
            };
            let m2 = match world.matrices.get(&ctx.matches[2]).unwrap() {
                Matrix::M4x4(m) => m,
                _ => panic!("matrix needs to be in 4x4 form"),
            };
            let t = ctx.step.table.as_ref().unwrap();
            let desired = form_vec_4x4(&t.rows);
            let computed = m1 * m2;
            assert_eq!(desired, computed);
            world
        },
    );

    steps.then_regex(r#"^(B|t) is the following 4x4 matrix:$"#, |world, ctx| {
        let m1 = match world.matrices.get(&ctx.matches[1]).unwrap() {
            Matrix::M4x4(m) => m,
            _ => panic!("matrix needs to be in 4x4 form"),
        };

        let t = ctx.step.table.as_ref().unwrap();
        let m = form_vec_4x4(&t.rows);

        assert_eq!(&m, m1);

        world
    });

    steps.given_regex(r#"^(C) ← (A) \* (B)$"#, |mut world, ctx| {
        let m1 = match world.matrices.get(&ctx.matches[2]).unwrap() {
            Matrix::M4x4(m) => m,
            _ => panic!("matrix needs to be in 4x4 form"),
        };
        let m2 = match world.matrices.get(&ctx.matches[3]).unwrap() {
            Matrix::M4x4(m) => m,
            _ => panic!("matrix needs to be in 4x4 form"),
        };
        let computed = Matrix::M4x4(m1 * m2);
        world.matrices.insert(ctx.matches[1].clone(), computed);
        world
    });

    steps.when_regex(r#"^(T) ← (C) \* (B) \* (A)$"#, |mut world, ctx| {
        let m1 = match world.matrices.get(&ctx.matches[2]).unwrap() {
            Matrix::M4x4(m) => m,
            _ => panic!("matrix needs to be in 4x4 form"),
        };
        let m2 = match world.matrices.get(&ctx.matches[3]).unwrap() {
            Matrix::M4x4(m) => m,
            _ => panic!("matrix needs to be in 4x4 form"),
        };
        let m3 = match world.matrices.get(&ctx.matches[4]).unwrap() {
            Matrix::M4x4(m) => m,
            _ => panic!("matrix needs to be in 4x4 form"),
        };
        let matrix = Matrix::M4x4(m1 * m2 * m3);
        world.matrices.insert(ctx.matches[1].clone(), matrix);
        world
    });

    steps.then_regex(
        r#"^(A) \* (b) = tuple\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |world, ctx| {
            let matrix = match world.matrices.get(&ctx.matches[1]).unwrap() {
                Matrix::M4x4(m) => m,
                _ => panic!("matrix needs to be in 4x4 form"),
            };
            let tuple = world.tuples.get(&ctx.matches[2]).unwrap();
            let computed = matrix * tuple;
            let desired = parse_tuple(&ctx.matches[3..=6]);
            assert_eq!(desired, computed);
            world
        },
    );

    steps.then_regex(
        r#"^(T) \* (p) = point\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$"#,
        |world, ctx| {
            let matrix = match world.matrices.get(&ctx.matches[1]).unwrap() {
                Matrix::M4x4(m) => m,
                _ => panic!("matrix needs to be in 4x4 form"),
            };
            let tuple = world.tuples.get(&ctx.matches[2]).unwrap();
            let computed = matrix * tuple;
            let desired = parse_point(&ctx.matches[3..=5]);
            assert_eq!(desired, computed);
            world
        },
    );

    steps.then_regex(r#"^(A) \* identity_matrix = (A)$"#, |world, ctx| {
        let matrix = match world.matrices.get(&ctx.matches[1]).unwrap() {
            Matrix::M4x4(m) => m,
            _ => panic!("matrix needs to be in 4x4 form"),
        };
        let desired_matrix = match world.matrices.get(&ctx.matches[2]).unwrap() {
            Matrix::M4x4(m) => m,
            _ => panic!("matrix needs to be in 4x4 form"),
        };
        let computed = matrix * identity_matrix();
        assert_eq!(desired_matrix, &computed);
        world
    });

    steps.then_regex(
        r#"^inverse\((A)\) is the following 4x4 matrix:$"#,
        |world, ctx| {
            let matrix = match world.matrices.get(&ctx.matches[1]).unwrap() {
                Matrix::M4x4(m) => m,
                _ => panic!("matrix needs to be in 4x4 form"),
            };
            let inverted_matrix = matrix.inverse().unwrap();
            let t = ctx.step.table.as_ref().unwrap();
            let desired_matrix = form_vec_4x4(&t.rows);
            assert_eq!(desired_matrix, inverted_matrix);
            world
        },
    );

    steps.given_regex(r#"^(B) ← inverse\((A)\)$"#, |mut world, ctx| {
        let matrix = match world.matrices.get(&ctx.matches[2]).unwrap() {
            Matrix::M4x4(m) => m,
            _ => panic!("matrix needs to be in 4x4 form"),
        };
        let inverse = matrix.inverse().unwrap();
        world
            .matrices
            .insert(ctx.matches[1].clone(), Matrix::M4x4(inverse));

        world
    });

    steps.then_regex(r#"^(C) \* inverse\((B)\) = (A)$"#, |world, ctx| {
        let m1 = match world.matrices.get(&ctx.matches[1]).unwrap() {
            Matrix::M4x4(m) => m,
            _ => panic!("matrix needs to be in 4x4 form"),
        };
        let m2 = match world.matrices.get(&ctx.matches[2]).unwrap() {
            Matrix::M4x4(m) => m,
            _ => panic!("matrix needs to be in 4x4 form"),
        };
        let computed = m1 * m2.inverse().unwrap();
        let desired_matrix = match world.matrices.get(&ctx.matches[3]).unwrap() {
            Matrix::M4x4(m) => m,
            _ => panic!("matrix needs to be in 4x4 form"),
        };
        assert_eq!(desired_matrix, &computed);
        world
    });

    steps.then_regex(r#"^identity_matrix \* (a) = (a)$"#, |world, ctx| {
        let tuple = world.tuples.get(&ctx.matches[1]).unwrap();
        let desired_tuple = world.tuples.get(&ctx.matches[2]).unwrap();
        let computed = tuple * identity_matrix();
        assert_eq!(desired_tuple, &computed);
        world
    });

    steps.then_regex(
        r#"^transpose\((A)\) is the following matrix:$"#,
        |world, ctx| {
            let matrix = match world.matrices.get(&ctx.matches[1]).unwrap() {
                Matrix::M4x4(m) => m,
                _ => panic!("matrix needs to be in 4x4 form"),
            };
            let computed = matrix.transpose();
            let t = ctx.step.table.as_ref().unwrap();
            let desired_matrix = form_vec_4x4(&t.rows);
            assert_eq!(desired_matrix, computed);
            world
        },
    );

    steps.given_regex(
        r#"^(A) ← transpose\(identity_matrix\)$"#,
        |mut world, ctx| {
            let computed = Matrix::M4x4(identity_matrix().transpose());
            world.matrices.insert(ctx.matches[1].clone(), computed);
            world
        },
    );

    steps.given_regex(
        r#"^(B) ← submatrix\((A), ([0-9]+), ([0-9]+)\)$"#,
        |mut world, ctx| {
            let w = ctx.matches[3].parse::<usize>().unwrap();
            let h = ctx.matches[4].parse::<usize>().unwrap();
            let matrix = match world.matrices.get(&ctx.matches[2]).unwrap() {
                Matrix::M4x4(m) => Matrix::M3x3(m.submatrix(w, h)),
                Matrix::M3x3(m) => Matrix::M2x2(m.submatrix(w, h)),
                _ => panic!("matrix needs to be in 4x4 or 3x3 form"),
            };
            world.matrices.insert(ctx.matches[1].clone(), matrix);
            world
        },
    );

    steps.then_regex(r#"^(A|t) = identity_matrix$"#, |world, ctx| {
        let matrix = match world.matrices.get(&ctx.matches[1]).unwrap() {
            Matrix::M4x4(m) => m,
            _ => panic!("matrix needs to be in 4x4 form"),
        };
        assert_eq!(matrix, &identity_matrix());
        world
    });

    steps.then_regex(
        r#"^minor\((A), ([0-9]+), ([0-9]+)\) = ([-0-9.]+)$"#,
        |world, ctx| {
            let w = ctx.matches[2].parse::<usize>().unwrap();
            let h = ctx.matches[3].parse::<usize>().unwrap();
            let desired = ctx.matches[4].parse::<f64>().unwrap();
            let computed = match world.matrices.get(&ctx.matches[1]).unwrap() {
                Matrix::M3x3(m) => m.minor(w, h),
                _ => panic!("matrix needs to be in 3x3 form"),
            };
            assert_abs_diff_eq!(desired, computed);
            world
        },
    );

    steps.then_regex(
        r#"^cofactor\((A), ([0-9]+), ([0-9]+)\) = ([-0-9.]+)$"#,
        |world, ctx| {
            let w = ctx.matches[2].parse::<usize>().unwrap();
            let h = ctx.matches[3].parse::<usize>().unwrap();
            let desired = ctx.matches[4].parse::<f64>().unwrap();
            let cofactor = match world.matrices.get(&ctx.matches[1]).unwrap() {
                Matrix::M3x3(m) => m.cofactor(w, h),
                Matrix::M4x4(m) => m.cofactor(w, h),
                _ => panic!("matrix needs to be in 3x3 or 4x4 form"),
            };
            assert_abs_diff_eq!(desired, cofactor);
            world
        },
    );

    steps.then_regex(r#"^determinant\((A|B)\) = ([-0-9.]+)$"#, |world, ctx| {
        let determinant = match world.matrices.get(&ctx.matches[1]).unwrap() {
            Matrix::M2x2(m) => m.determinant(),
            Matrix::M3x3(m) => m.determinant(),
            Matrix::M4x4(m) => m.determinant(),
        };
        let desired = ctx.matches[2].parse::<f64>().unwrap();
        assert_abs_diff_eq!(desired, determinant);
        world
    });

    steps.then_regex(r#"^(A) is (not )?invertible$"#, |world, ctx| {
        let invertible = match world.matrices.get(&ctx.matches[1]).unwrap() {
            Matrix::M2x2(m) => m.invertible(),
            Matrix::M3x3(m) => m.invertible(),
            Matrix::M4x4(m) => m.invertible(),
        };
        assert!(match ctx.matches[2].as_str() {
            "not " => !invertible,
            _ => invertible,
        });
        world
    });

    steps.then_regex(
        r#"^submatrix\((A), ([0-9]+), ([0-9]+)\) is the following (2x2|3x3) matrix:$"#,
        |world, ctx| {
            let w = ctx.matches[2].parse::<usize>().unwrap();
            let h = ctx.matches[3].parse::<usize>().unwrap();
            let same = match ctx.matches[4].as_str() {
                "2x2" => {
                    form_vec_2x2(&ctx.step.table.as_ref().unwrap().rows)
                        == match world.matrices.get(&ctx.matches[1]).unwrap() {
                            Matrix::M3x3(m) => m.submatrix(w, h),
                            _ => panic!("matrix needs to be in 4x4 form"),
                        }
                }
                "3x3" => {
                    form_vec_3x3(&ctx.step.table.as_ref().unwrap().rows)
                        == match world.matrices.get(&ctx.matches[1]).unwrap() {
                            Matrix::M4x4(m) => m.submatrix(w, h),
                            _ => panic!("matrix needs to be in 4x4 form"),
                        }
                }
                _ => panic!("matrix dimentions not covered"),
            };
            assert!(same);
            world
        },
    );

    steps
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
