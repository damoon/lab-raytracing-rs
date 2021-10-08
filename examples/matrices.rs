use lab_raytracing_rs::matrices::{identity_matrix, Matrix4x4};
use lab_raytracing_rs::tuples::Tuple;

fn main() {
    println!("identity_matrix\n{}", identity_matrix());
    println!(
        "inverted identity_matrix\n{}",
        identity_matrix().inverse().unwrap()
    );

    println!();
    let mut state = [[0.0_f64; 4]; 4];
    let mut i = 20.0;
    for (_, row) in state.iter_mut().enumerate() {
        for (_, e) in row.iter_mut().enumerate() {
            *e = i;
            i += 5.1;
        }
    }
    let matrix = Matrix4x4::new_from(state);
    println!("matrix\n{}", matrix);
    println!("inverted matrix\n{}", matrix.inverse().unwrap());
    println!(
        "inverted matrix * matrix\n{}",
        &matrix * &matrix.inverse().unwrap()
    );

    println!();
    println!(
        "inverse of the transpose of matrix\n{}",
        matrix.transpose().inverse().unwrap()
    );
    println!(
        "transpose of the inverse of matrix\n{}",
        matrix.inverse().unwrap().transpose()
    );

    println!();
    let tuple = Tuple::new(2.0, 3.0, 4.0, 5.0);
    println!("tuple\n{}", &tuple);
    println!("tuple * identity_matrix\n{}", &tuple * identity_matrix());
    let mut state = [[0.0_f64; 4]; 4];
    state[0][0] = 1.0;
    state[1][1] = 1.0;
    state[2][2] = 1.0;
    state[3][3] = 1.0;
    state[2][1] = 1.0;
    let matrix = Matrix4x4::new_from(state);
    println!("matrix\n{}", matrix);
    println!("tuple * matrix\n{}", &tuple * matrix);
}
