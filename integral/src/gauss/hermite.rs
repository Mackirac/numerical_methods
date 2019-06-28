use std::f64::consts::PI;
static SQRT: fn(f64)->f64 = f64::sqrt;

fn gauss_hermite(points: &[(f64, f64)], f: impl Fn(f64)->f64) -> f64 {
    let mut output : f64 = 0.0;
    for point in points {
        output += f(point.0) * point.1;
    }
    output
}

#[allow(dead_code)]
pub fn g2(f: impl Fn(f64)->f64) -> f64 {
    let hermite = [
        (-SQRT(2.0)/2.0, SQRT(PI)/2.0),
        ( SQRT(2.0)/2.0, SQRT(PI)/2.0)
    ];
    gauss_hermite(&hermite, f)
}

#[allow(dead_code)]
pub fn g3(f: impl Fn(f64)->f64) -> f64 {
    let hermite = [
        (-SQRT(6.0)/2.0, SQRT(PI)/6.0),
        (0.0, 2.0*SQRT(PI)/3.0),
        ( SQRT(6.0)/2.0, SQRT(PI)/6.0)
    ];
    gauss_hermite(&hermite, f)
}

#[allow(dead_code)]
pub fn g4(f: impl Fn(f64)->f64) -> f64 {
    let hermite = [
        (-SQRT((3.0 + SQRT(6.0))/2.0), SQRT(PI)/(4.0 * (3.0 + SQRT(6.0)))),
        (-SQRT((3.0 - SQRT(6.0))/2.0), SQRT(PI)/(4.0 * (3.0 - SQRT(6.0)))),
        ( SQRT((3.0 - SQRT(6.0))/2.0), SQRT(PI)/(4.0 * (3.0 - SQRT(6.0)))),
        ( SQRT((3.0 + SQRT(6.0))/2.0), SQRT(PI)/(4.0 * (3.0 + SQRT(6.0))))
    ];
    gauss_hermite(&hermite, f)
}
