pub mod open;
pub mod closed;

pub trait NewtonCotes {
    fn get(self) -> fn(f64, f64, fn(f64)->f64)->f64;
}

#[allow(dead_code)]
pub fn integrate(
    a: f64, b: f64, precision: f64, method: impl NewtonCotes, f: fn(f64)->f64
) -> f64 {
    let method : fn(f64, f64, fn(f64)->f64) -> f64 = method.get();
    let mut _previous_i : f64 = 0.0;
    let mut actual_i : f64 = method(a, b, f);
    let mut partitions : usize = 1;

    loop {
        partitions *= 2;
        _previous_i = actual_i;
        actual_i = 0.0;
        let step = (b - a)/(partitions as f64);
        for i in 0..partitions {
            actual_i += method(a+(i as f64)*step, a+((i+1) as f64)*step, f);
        }

        if (actual_i - _previous_i).abs() <= precision { return actual_i }
    }
}
