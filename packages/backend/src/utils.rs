use std::f64::consts::PI;

use rand::{thread_rng, Rng};

use crate::{config::AU, fmm::Particle};

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

pub fn to_circular(r: &f64) -> (f64, f64) {
    let phi = 2. * PI * random();
    let x = r * phi.cos();
    let y = r * phi.sin();

    (x, y)
}
