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
    let n = A.lines();
    for k in 1..n-2+1 {
        let alfa = if A[(k+1, k)] < 0.0 { -1.0 } else { 1.0 } * {
            let mut alfa = 0.0;
            for j in k+1..n+1 {
                alfa += A[(j, k)] * A[(j, k)];
            }
            f64::sqrt(alfa)
        };
        let r = f64::sqrt((alfa*alfa - A[(k+1, k)]*alfa) / 2.0);
        let mut v = Vector::zero(n);
        v[k+1] = (A[(k+1, k)] - alfa) / (2.0*r);
        for j in k+2..n+1 {
            v[j] = A[(j, k)] / (2.0*r);
        }
        let P = (Matrix::identity(n) - 2.0*v.clone().x(v)).unwrap();
        A = ((P.clone() * A).unwrap() * P).unwrap();
    }
    A
}
