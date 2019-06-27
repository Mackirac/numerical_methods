mod integral;

use integral::newton_cotes::{ integrate, closed::Closed::G4 as G };

fn main() {
    let f = |x: f64| -> f64 { f64::sin(1.5*x) };
    let i = integrate(0.0, std::f64::consts::PI, 0.0000001, G, f);
    println!("{:?}", i);
}
