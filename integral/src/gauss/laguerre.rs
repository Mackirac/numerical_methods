fn gauss_laguerre(points: &[(f64, f64)], f: impl Fn(f64)->f64) -> f64 {
    let mut output : f64 = 0.0;
    for point in points {
        output += f(point.0) * point.1;
    }
    output
}

#[allow(dead_code)]
pub fn g2(f: impl Fn(f64)->f64) -> f64 {
    let hermite = [
        (
            0.58578643762690495119831127579,
            0.853553390593273762200422181052
        ),
        (
            3.41421356237309504880168872421,
            0.146446609406726237799577818948
        )
    ];
    gauss_laguerre(&hermite, f)
}

#[allow(dead_code)]
pub fn g3(f: impl Fn(f64)->f64) -> f64 {
    let hermite = [
        (
            0.415774556783479083311533873128,
            0.711093009929173015449590191143
        ),
        (
            2.29428036027904171982205036136,
            0.278517733569240848801444888457
        ),
        (
            6.28994508293747919686641576551,
            0.0103892565015861357489649204007
        )
    ];
    gauss_laguerre(&hermite, f)
}

#[allow(dead_code)]
pub fn g4(f: impl Fn(f64)->f64) -> f64 {
    let hermite = [
        (
            0.322547689619392311800361459104,
            0.603154104341633601635966023818
        ),
        (
            1.74576110115834657568681671252,
            0.35741869243779968664149201746
        ),
        (
            4.53662029692112798327928538496,
            0.0388879085150053842724381681562
        ),
        (
            9.39507091230113312923353644342,
            5.3929470556132745010379056762E-4
        )
    ];
    gauss_laguerre(&hermite, f)
}
