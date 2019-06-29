#![allow(non_snake_case)]

extern crate matrix;
pub use matrix::{ matrix::Matrix, vector::Vector };

pub fn regular_power(A: Matrix, E: f64, x: Vector) -> (f64, Vector) {
    if A.cols() != x.dimensions() { panic!("Unmatching matrix and vector lengths") }
    let mut v_new = 0_f64;
    let mut x_old = x.normalize();

    loop {
        let v_old = v_new;
        let y = A.clone() * x_old.clone();
        let x_new = y.clone().normalize();
        v_new = x_old * y;
        x_old = x_new.clone();
        if ((v_new - v_old)/v_new).abs() <= E {
            return (v_new, x_new);
        }
    }
}
