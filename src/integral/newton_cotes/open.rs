use super::NewtonCotes;

#[allow(dead_code)]
pub enum Open { G2, G3, G4 }

impl NewtonCotes for Open {
    fn get <T: Fn(f64)->f64> (self) -> fn(f64, f64, &T)->f64 {
        match self {
            Open::G2 => nc2,
            Open::G3 => nc3,
            Open::G4 => nc4
        }
    }
}

#[allow(dead_code)]
fn nc2 <T: Fn(f64)->f64> (a: f64, b: f64, f: &T) -> f64 {
    let f1 = f(a + (b - a)/2.0);

    f1 * (b - a)
}

#[allow(dead_code)]
fn nc3 <T: Fn(f64)->f64> (a: f64, b: f64, f: &T) -> f64 {
    let f1 = f(a + (b - a)/3.0);
    let f2 = f(a + (b - a)*2.0/3.0);

    (f1 + f2) * (b - a)/2.0
}

#[allow(dead_code)]
fn nc4 <T: Fn(f64)->f64> (a: f64, b: f64, f: &T) -> f64 {
    let f1 = f(a + (b - a)/4.0);
    let f2 = f(a + (b - a)*2.0/4.0);
    let f3 = f(a + (b - a)*3.0/4.0);

    (2.0*f1 - f2 + 2.0*f3) * (b - a)/3.0
}
