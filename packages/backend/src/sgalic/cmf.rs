// Cumulative Mass Functions
fn brentq(f: &dyn Fn(f64) -> f64, bounds: (f64, f64)) -> f64 {
    let (a, b) = bounds;
    let xtol: f64 = 1e-15;
    let ytol: f64 = 1e-8;

    let mut xb = [a, b];
    let mut yb = [f(a), f(b)];

    assert!(yb.map(|x| x.signum()).iter().sum::<f64>() == 0.);

    if yb[0].abs() < yb[1].abs() {
        xb = [b, a];
        yb = [yb[1], yb[0]];
    }

    let mut c = xb[0];
    let mut d = 0.;
    let mut yc = yb[0];
    let mut usedbisec = true;

    let mut x = 0.;
    let mut y = 0.;

    for _ in 0..100 {
        let mut s = {
            if yb[0] != yc && yb[1] != yc {
                xb[0] * yb[1] * yc / ((yb[0] - yb[1]) * (yb[0] - yc))
                    + xb[1] * yb[0] * yc / ((yb[1] - yb[0]) * (yb[1] - yc))
                    + c * yb[0] * yb[1] / ((yc - yb[0]) * (yc - yb[1]))
            } else {
                (yb[0] * xb[1] - yb[1] * xb[0]) / (yb[0] - yb[1])
            }
        };
        if (s - (3. * xb[0] + xb[1]) / 4.) * (s - xb[1]) >= 0.
            || (usedbisec && (s - xb[1]).abs() >= (xb[1] - c).abs() / 2.)
            || (!usedbisec && (s - xb[1]).abs() >= (c - d).abs() / 2.)
            || (usedbisec && (xb[1] - c).abs() < (xtol).abs())
            || (!usedbisec && (c - d).abs() < (xtol).abs())
        {
            s = (xb[0] + xb[1]) / 2.;
            usedbisec = true;
        } else {
            usedbisec = false
        }
        let ys = f(s);
        d = c;
        c = xb[1];
        yc = yb[1];
        if yb[0] * ys < 0. {
            xb[1] = s;
            yb[1] = ys;
        } else {
            xb[0] = s;
            yb[0] = ys;
        }
        if yb[0].abs() < yb[1].abs() {
            xb = [xb[1], xb[0]];
            yb = [yb[1], yb[0]];
        }
        if yb[1] == 0. {
            x = xb[1];
            y = yb[1];
            break;
        }
        if ys == 0. {
            x = s;
            y = ys;
            break;
        }
        if (xb[1] - xb[0]).abs() < xtol {
            x = s;
            y = ys;
            break;
        }
    }
    x
}

// Halo CMFs
pub fn dehnen_cmf(r: f64, M: f64, a: f64, gamma: i32) -> f64 {
    M * (r / (r + a)).powi(3 - gamma)
}

pub fn dehnen_cmf_inv(Mc: Vec<f64>, M: f64, a: f64, gamma: i32) -> Vec<f64> {
    Mc.iter()
        .map(|i| {
            assert!(0. <= *i && *i <= M);
            let f = |r| dehnen_cmf(r, M, a, gamma) - i;
            brentq(&f, (0., 1e10))
        })
        .collect::<Vec<f64>>()
}

// Disk CMFs
pub fn disk_radial_cmf(r: &f64, Rd: &f64) -> f64 {
    (Rd.powi(2) - (Rd.powi(2) + r * Rd) * (-r / Rd).exp()) / Rd.powi(2)
}

pub fn disk_radial_cmf_inv(frac: &f64, Rd: &f64) -> f64 {
    let f = |r: f64| disk_radial_cmf(&r, Rd) - frac;
    brentq(&f, (0., 1e10))
}

pub fn disk_height_cmf_inv(frac: &f64, z0: &f64) -> f64 {
    0.5 * z0 * (frac / (1. - frac)).ln()
}

#[cfg(test)]
mod tests {
    use crate::sgalic::cmf::disk_radial_cmf_inv;

    use super::brentq;

    #[test]
    fn test_cubic() {
        let f = |x: f64| x.powi(3) - 13. * x.powi(2) + 20. * x + 100.;
        let x = brentq(&f, (-1e10, 1e10));
        println!("solutions: {} {}", x, f(x));
    }

    #[test]
    fn test_disk_radial_cmv_inf() {
        println!("{:?}", disk_radial_cmf_inv(&0., &0.));
    }
}
