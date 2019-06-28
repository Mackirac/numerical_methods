static SQRT: fn(f64)->f64 = f64::sqrt;

fn _x_(a:f64, b: f64, x: f64) -> f64 { (a+b+(b-a)*x)/2.0 }

fn gauss_legendre(a: f64, b: f64, points: &[(f64, f64)], f: impl Fn(f64)->f64) -> f64 {
    let mut output : f64 = 0.0;
    for point in points {
        output += f(_x_(a, b, point.0)) * point.1;
    }
    output * (b - a) / 2.0
}

#[allow(dead_code)]
pub fn g2(a: f64, b: f64, f: impl Fn(f64)->f64) -> f64 {
    let legendre = [
        (-SQRT(3.0)/3.0, 1_f64),
        ( SQRT(3.0)/3.0, 1_f64)
    ];
    gauss_legendre(a, b, &legendre, f)
}

#[allow(dead_code)]
pub fn g3(a: f64, b: f64, f: impl Fn(f64)->f64) -> f64 {
    let legendre = [
        (-SQRT(3.0/5.0), 5_f64/9.0),
        (0_f64, 8_f64/9.0),
        ( SQRT(3.0/5.0), 5_f64/9.0)
    ];
    gauss_legendre(a, b, &legendre, f)
}

#[allow(dead_code)]
pub fn g4(a: f64, b: f64, f: impl Fn(f64)->f64) -> f64 {
    let legendre = [
        (-SQRT((3.0 - 2.0*SQRT(6.0/5.0))/7.0), (18.0 + SQRT(30.0))/36.0),
        (-SQRT((3.0 + 2.0*SQRT(6.0/5.0))/7.0), (18.0 - SQRT(30.0))/36.0),
        ( SQRT((3.0 + 2.0*SQRT(6.0/5.0))/7.0), (18.0 - SQRT(30.0))/36.0),
        ( SQRT((3.0 - 2.0*SQRT(6.0/5.0))/7.0), (18.0 + SQRT(30.0))/36.0),
    ];
    gauss_legendre(a, b, &legendre, f)
}
