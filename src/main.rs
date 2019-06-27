mod integral;

use integral::newton_cotes::{
    integrate,
    closed::Closed::G4 as G
};
use integral::gauss::legendre::g4 as gl;

fn main() {
    let f = |x: f64| -> f64 { (x).sin() };

    let i = integrate(0.0, std::f64::consts::PI, 1e-10, G, f);
    println!("Newton-Cotes error: {:.10?}", (i - 2.0).abs());

    let i = gl(0.0, std::f64::consts::PI, f);
    println!("Gauss-Legendre error: {:.10?}", (i - 2.0).abs());
}
