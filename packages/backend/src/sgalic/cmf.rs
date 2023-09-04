use roots::find_root_brent;
use roots::SimpleConvergency;

// Cumulative Mass Functions

fn brentq(f: &dyn Fn(f64) -> f64) -> f64 {
    let mut convergency = SimpleConvergency {
        eps: 1e-15f64,
        max_iter: 30,
    };
    find_root_brent(0., 1e10, &f, &mut convergency).unwrap()
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
            brentq(&f)
        })
        .collect::<Vec<f64>>()
}

// Disk CMFs
pub fn disk_radial_cmf(r: &f64, Rd: &f64) -> f64 {
    (Rd.powi(2) - (Rd.powi(2) + r * Rd) * (-r / Rd).exp()) / Rd.powi(2)
}

pub fn disk_radial_cmf_inv(frac: &f64, Rd: &f64) -> f64 {
    let f = |r: f64| disk_radial_cmf(&r, Rd) - frac;
    brentq(&f)
}

pub fn disk_height_cmf_inv(frac: &f64, z0: &f64) -> f64 {
    0.5 * z0 * (frac / (1. - frac)).ln()
}
