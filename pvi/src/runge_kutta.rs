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
