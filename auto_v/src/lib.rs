#![allow(non_snake_case)]

extern crate matrix;
pub use matrix::{ matrix::Matrix, vector::Vector };

pub fn regular_power(A: Matrix, E: f64, mut x: Vector) -> (f64, Vector) {
    if A.cols() != x.dimensions() { panic!("Unmatching matrix and vector lengths") }

    let mut v_old : f64;
    let mut v_new = x.clone() * A.clone() * x.clone();
    loop {
        v_old = v_new;

        x = (A.clone() * x.clone()).normalize();
        v_new = x.clone() * A.clone() * x.clone();

        if ((v_new - v_old)/v_new).abs() <= E { return (v_new, x) }
    }
}

pub fn inverse_power(A: Matrix, E: f64, x: Vector) -> (f64, Vector) {
    let (av, aV) = regular_power(A.inverse(), E, x);
    (1.0/av, aV)
}

pub fn shift_power(A: Matrix, E: f64, x: Vector, mi: f64) -> (f64, Vector) {
    let A = A.clone() - mi*Matrix::identity(A.lines());
    let (av, aV) = inverse_power(A.unwrap(), E, x);
    (av + mi, aV)
}


pub fn householder(mut A: Matrix) -> Matrix {
    let mut H = Matrix::identity(A.lines());
    for i in 1..A.lines()-1 {
        let H_C = build_house_holder(&mut A, i);
        A = ((H_C.clone() * A).unwrap() * H_C.clone()).unwrap();
        H = (H * H_C).unwrap();
    }
    A
}

fn build_house_holder(A: &mut Matrix, i: usize) -> Matrix {
    let mut v = Vector::zero(A.lines());
    for j in i+1..A.lines()+1 {
        v[j] = A[(j, i)];
    }
    v[i+1] -= v.dimensions() as f64;
    v = v.normalize();
    (Matrix::identity(A.lines()) - v.clone().x(v)*2.0).unwrap()
}
