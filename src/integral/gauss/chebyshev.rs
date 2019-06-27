use std::f64::consts::PI;
static COS: fn(f64)->f64 = f64::cos;

fn _x_(k: usize, n: usize) -> f64 {
    COS((PI * (2*k - 1) as f64) / (2*n) as f64)
}

#[allow(dead_code)]
pub fn gauss_chebyshev(n: usize, f: impl Fn(f64)->f64) -> f64 {
    let mut output : f64 = 0.0;
    for i in 0..n {
        output += f(_x_(i+1, n)) * PI / n as f64;
    }
    output
}
