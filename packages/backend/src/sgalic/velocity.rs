use crate::sgalic::config::G_CONSTANT;
use rand::{thread_rng, Rng};
use std::f64::consts::PI;

// UTILS
pub fn random() -> f64 {
    thread_rng().gen::<f64>()
}

pub fn gen_random_array(n: usize) -> Vec<f64> {
    (0..n).map(|_| random()).collect::<Vec<f64>>()
}

pub fn to_spherical(r: &f64) -> (f64, f64, f64) {
    let theta = (2. * random() - 1.).acos();
    let phi = 2. * PI * random();

    let x = r * theta.sin() * phi.cos();
    let y = r * theta.sin() * phi.sin();
    let z = r * theta.cos();

    (x, y, z)
}

// Velocity Functions
pub fn plummer(v: &Vec<(f64, f64, f64)>) -> Vec<(f64, f64, f64)> {
    v.iter()
        .map(|&pos| {
            let radius: f64 = [pos.0, pos.1, pos.2].map(|x| x.powi(2)).iter().sum();
            let mut x: f64 = 0.0;
            let mut y: f64 = 0.1;

            while y > x * x * (1. - x * x).powf(3.5) {
                x = random();
                y = random() / 10.;
            }
            let velocity = x * (2.0_f64).sqrt() * (1. + radius * radius).powf(-0.25);
            to_spherical(&velocity)
        })
        .collect::<Vec<(f64, f64, f64)>>()
}

pub fn keplerian(v: &Vec<(f64, f64, f64)>, mass: f64) -> Vec<(f64, f64, f64)> {
    v.iter()
        .map(|&pos| {
            let radius: f64 = [pos.0, pos.1, pos.2].map(|x| x.powi(2)).iter().sum();
            let vel = G_CONSTANT * mass * radius.sqrt().recip();
            (vel * -pos.0.sin(), vel * pos.1.cos(), pos.2)
        })
        .collect::<Vec<(f64, f64, f64)>>()
}
