use roots::find_root_brent;
use roots::SimpleConvergency;

pub fn dehnen_cmf(r: f64, M: f64, a: i32, gamma: i32) -> f64 {
    M * (r / (r + (a as f64))).powi(3 - gamma)
}

pub fn dehnen_cmf_inverse(Mc: Vec<f64>, M: f64, a: i32, gamma: i32) -> Vec<f64> {
    Mc.iter()
        .map(|i| {
            assert!(0. <= *i && *i <= M);
            let f = |r| dehnen_cmf(r, M, a, gamma) - i;
            let mut convergency = SimpleConvergency {
                eps: 1e-15f64,
                max_iter: 30,
            };
            find_root_brent(0., 1e10, &f, &mut convergency).unwrap()
        })
        .collect::<Vec<f64>>()
}
