#[allow(dead_code)]
pub enum Closed { G1, G2, G3, G4 }

impl Closed {
    #[allow(dead_code)]
    pub fn function(&self) -> fn(f64, f64, fn(f64)->f64)->f64 {
        match self {
            &Closed::G1 => nc1,
            &Closed::G2 => nc2,
            &Closed::G3 => nc3,
            &Closed::G4 => nc4
        }
    }
}

#[allow(dead_code)]
fn nc1(a: f64, b: f64, f: impl Fn(f64)->f64) -> f64 {
    (f(a)+f(b)) * (b - a)/2.0
}

#[allow(dead_code)]
fn nc2(a: f64, b: f64, f: impl Fn(f64)->f64) -> f64 {
    let f0 = f(a);
    let f1 = f(a + (b - a)/2.0);
    let f2 = f(b);

    (f0 + 4.0*f1 + f2) * (b - a)/6.0
}

#[allow(dead_code)]
fn nc3(a: f64, b: f64, f: impl Fn(f64)->f64) -> f64 {
    let f0 = f(a);
    let f1 = f(a + (b - a)/3.0);
    let f2 = f(a + (b - a)*2.0/3.0);
    let f3 = f(b);

    (f0 + 3.0*f1 + 3.0*f2 + f3) * (b - a)/8.0
}

#[allow(dead_code)]
fn nc4(a: f64, b: f64, f: impl Fn(f64)->f64) -> f64 {
    let f0 = f(a);
    let f1 = f(a + (b - a)/4.0);
    let f2 = f(a + (b - a)*2.0/4.0);
    let f3 = f(a + (b - a)*3.0/4.0);
    let f4 = f(b);

    (7.0*f0 + 32.0*f1 + 12.0*f2 + 32.0*f3 + 7.0*f4) * (b - a)/90.0
}
