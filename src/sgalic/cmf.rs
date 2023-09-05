use std::mem::swap;

// Cumulative Mass Functions
fn brentq(f: &dyn Fn(f64) -> f64, bounds: (f64, f64)) -> f64 {
    let (mut a, mut b) = bounds;
    let mut fa = f(a);
    let mut fb = f(b);
    let mut fs = 0.;
    let MAX_ITER = 10000;
    let tolerance = 1e-5;

    if fa.abs() < fb.abs() {
        swap(&mut a, &mut b);
        swap(&mut fa, &mut fb);
    }

    let mut c = a;
    let mut fc = fa;
    let mut mflag = true;
    let mut s = 0.;
    let mut d = 0.;

    for _ in 0..MAX_ITER {
        if (a - b).abs() < tolerance {
            break;
        }
        if (fa != fc && fb != fc) {
            s = (a * fb * fc / ((fa - fb) * (fa - fc)))
                + (b * fa * fc / ((fb - fa) * (fb - fc)))
                + (c * fa * fb / ((fc - fa) * (fc - fb)));
        } else {
            s = b - fb * (b - a) / (fb - fa);
        }
        if ((s < (3. * a + b) * 0.25) || (s > b))
            || (mflag && ((s - b).abs() >= ((b - c).abs() * 0.5)))
            || (!mflag && ((s - b).abs() >= ((c - d).abs() * 0.5)))
            || (mflag && ((b - c).abs() < tolerance))
            || (!mflag && ((c - d).abs() < tolerance))
        {
            s = (a + b) * 0.5;

            mflag = true;
        } else {
            mflag = false;
        }

        fs = f(s);
        d = c;
        c = b;
        fc = fb;

        if (fa * fs < 0.) {
            b = s;
            fb = fs;
        } else {
            a = s;
            fa = fs;
        }

        if (fa.abs() < fb.abs()) {
            swap(&mut a, &mut b);
            swap(&mut fa, &mut fb);
        }
    }

    s
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
    brentq(&f, (-1e10, 1e10))
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
