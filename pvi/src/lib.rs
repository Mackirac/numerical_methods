pub fn runge_kutta <T: Fn(f64, f64)->f64> (
    f: T, ti: f64, vi: f64, t: f64, h: f64
) -> f64 {
    let n = f64::floor((t - ti)/h) as usize;
    let (mut k1, mut k2, mut k3, mut k4) : (f64, f64, f64, f64);
    let (mut t_a, mut v_a) = (ti, vi);
    for _ in 1..n+1 {
        k1 = f(t_a, v_a);
        k2 = f(t_a + h/2.0, v_a + h*k1/2.0);
        k3 = f(t_a + h/2.0, v_a + h*k2/2.0);
        k4 = f(t_a + h, v_a + h*k3);

        v_a += h * (k1 + 2.0*k2 + 2.0*k3 + k4) / 6.0;
        t_a += h;
    }
    v_a
}

pub fn forward_euler(
    f: impl Fn(f64, f64)->f64, ti: f64, vi: f64, t: f64, e: f64
) -> f64 {
    let mut h = 1.0;
    let mut v_a = forward_euler_inner_loop(&f, ti, vi, t, h);
    h = 0.1;
    loop {
        let v_old = v_a;
        v_a = forward_euler_inner_loop(&f, ti, vi, t, h);
        if (v_a - v_old).abs() <= e { return v_a }
        h /= 10.0;
    }
}

fn forward_euler_inner_loop<T>(f: &T, ti: f64, vi: f64, t: f64, h:f64) -> f64
where T: Fn(f64, f64)->f64 {
    let mut v_a = vi + h*f(ti, vi);
    let mut t_a = ti + h;
    loop {
        v_a = v_a + h*f(t_a, v_a);
        t_a += h;
        if t_a >= t { return v_a }
    }
}

pub fn predictor_corrector(
    f: impl Fn(f64, f64)->f64, ti: f64, vi: f64, t: f64, h: f64
) -> f64 {
    let mut t_a = ti;
    let mut v_a = vi;
    let n = f64::floor((t - ti)/h) as usize;
    for _ in 0..n {
        let v_old = v_a;
        v_a += h*f(t_a, v_a);
        t_a += h;
        v_a = v_old + h*f(t_a, v_a);
    }
    v_a
}
