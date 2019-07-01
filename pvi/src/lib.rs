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
