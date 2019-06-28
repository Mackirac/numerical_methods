extern crate integral;

use std::f64::consts::PI;

use integral::newton_cotes::{
    integrate,
    closed::Closed::G4 as G
};
use integral::gauss::{
    legendre::g4 as gl,
    hermite::g4 as gh,
    laguerre::g2 as glr,
    chebyshev::gauss_chebyshev as gc
};

fn main() {
    println!();

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
}
