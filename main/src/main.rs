extern crate integral;
extern crate auto_v;

// use std::f64::consts::PI;

// use integral::newton_cotes::{
//     integrate,
//     closed::Closed::G4 as G
// };
// use integral::gauss::{
//     legendre::g4 as gl,
//     hermite::g4 as gh,
//     laguerre::g2 as glr,
//     chebyshev::gauss_chebyshev as gc
// };
// use auto_v::*;
// use pvi::*;

fn main() {
    println!();
    /*
    let f = |x: f64| -> f64 { x.sin() };
    let g = |x: f64| -> f64 { x.cos() };
    let h = |x: f64| -> f64 { x.powi(2)+2.0 };

    let i = integrate(0.0, PI, 1e-10, G, &f);
    println!("Newton-Cotes Result: {:.5?}\nNewton-Cotes error: {:.10?}\n", i, (i - 2.0).abs());

    let i = gl(0.0, PI, &f);
    println!("Gauss-Legendre Result: {:.5?}\nGauss-Legendre error: {:.10?}\n", i, (i - 2.0).abs());

    let i = gh(&h);
    println!("Gauss-Hermite Result: {:?}\n", i);

    let i = glr(&h);
    println!("Gauss-Laguerre Result: {:?}\n", i);

    let i = gc(10, &g);
    println!("Gauss-Chebyshev Result: {:?}\n", i);

    let m = Matrix::from_vec(3, 3, vec!(
        1, 0, 1,
        2, 7, 1,
        3, 3, 3
    ));

    #[allow(non_snake_case)]
    let (mv, mV) = regular_power(m.clone(), 1e-10, Vector::from_vec(vec!(1, 1, 1)));
    println!("Autovalor: {:?}\nAutovetor: {:?}", mv, mV);
    println!("{:?}", m.clone()*mV.clone());
    println!("{:?}", mv.clone()*mV.clone());
    println!();

    #[allow(non_snake_case)]
    let (mv, mV) = shift_power(m.clone(), 1e-10, Vector::from_vec(vec!(1, 1, 1)), 4.0);
    println!("Autovalor: {:?}\nAutovetor: {:?}", mv, mV);
    println!("{:?}", m.clone()*mV.clone());
    println!("{:?}", mv.clone()*mV.clone());
    println!();

    #[allow(non_snake_case)]
    let (mv, mV) = inverse_power(m.clone(), 1e-10, Vector::from_vec(vec!(1, 1, 1)));
    println!("Autovalor: {:?}\nAutovetor: {:?}", mv, mV);
    println!("{:?}", m.clone()*mV.clone());
    println!("{:?}", mv.clone()*mV.clone());
    println!();

    let m = Matrix::from_vec(5, 5, vec!(
        42, 03, 07, 07, 07,
        03, 20, 03, 09, 10,
        07, 03, 64, 14, 14,
        07, 09, 14, 37, 10,
        07, 10, 14, 10, 53
    ));
    println!("{:?}\n", householder(m));

    let v = forward_euler(|_, v| { 2.0*v }, 0.0, 1.0, 1.0, 10e-2);
    println!("{:?}\n", v);

    let v = runge_kutta(|_, v| { 2.0*v }, 0.0, 1.0, 1.0, 10e-5);
    println!("{:?}\n", v);

    let v = predictor_corrector(|_, v| { 2.0*v }, 0.0, 1.0, 1.0, 10e-5);
    println!("{:?}\n", v);
    */
}
