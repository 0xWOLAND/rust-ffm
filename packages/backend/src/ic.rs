use std::sync::{Arc, Mutex};

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    fmm::{Particle, Point, Velocity},
    utils::{random, to_spherical},
};
pub fn nagai_par(n: usize, a: Option<f64>, M: Option<f64>) -> Vec<Particle> {
    let particles = Arc::new(Mutex::new(Vec::<Particle>::new()));

    [0..n].par_iter().for_each(|_i| {
        let a = a.unwrap_or_else(|| 1.);
        let M = M.unwrap_or_else(|| 1.);
        let m = M / (n as f64);
        // sqrt(z^2+b^2) + a
        let radius = (a) / ((M / m).powf(2. / 3.) - 1.).sqrt();
        let (p_x, p_y, p_z) = to_spherical(&radius);

        let mut x: f64 = 0.0;
        let mut y: f64 = 0.1;

        while y > x * x * (1. - x * x).powf(3.5) {
            x = random();
            y = random() / 10.;
        }
        let velocity = x * (2.0_f64).sqrt() * (1. + radius * radius).powf(-0.25);
        let (v_x, v_y, v_z) = to_spherical(&velocity);

        particles.lock().unwrap().push(Particle {
            p: Point { p_x, p_y, p_z },
            v: Velocity { v_x, v_y, v_z },
            mass: m,
        })
    });
    let res: Vec<Particle> = particles.lock().unwrap().to_vec();
    res
}

pub fn nagai(n: usize, a: Option<f64>, M: Option<f64>) -> Vec<Particle> {
    let mut particles = Vec::<Particle>::new();

    for _ in 0..n {
        let mass = M.unwrap_or_else(|| 1.0) / (n as f64);
        // sqrt(z^2+b^2) + a
        let radius = (a.unwrap_or_else(|| 1.))
            / ((M.unwrap_or_else(|| 1.0) / mass).powf(2. / 3.) - 1.).sqrt();
        let (p_x, p_y, p_z) = to_spherical(&radius);

        let mut x: f64 = 0.0;
        let mut y: f64 = 0.1;

        while y > x * x * (1. - x * x).powf(3.5) {
            x = random();
            y = random() / 10.;
        }
        let velocity = x * (2.0_f64).sqrt() * (1. + radius * radius).powf(-0.25);
        let (v_x, v_y, v_z) = to_spherical(&velocity);

        particles.push(Particle {
            p: Point { p_x, p_y, p_z },
            v: Velocity { v_x, v_y, v_z },
            mass,
        })
    }
    particles
}

pub fn plummer(n: usize, a: Option<f64>, M: Option<f64>) -> Vec<Particle> {
    let mut particles = Vec::<Particle>::new();

    for _ in 0..n {
        let mass = M.unwrap_or_else(|| 1.0) / (n as f64);
        let radius = (a.unwrap_or_else(|| 1.))
            / ((M.unwrap_or_else(|| 1.0) / mass).powf(2. / 3.) - 1.).sqrt();
        let (p_x, p_y, p_z) = to_spherical(&radius);

        let mut x: f64 = 0.0;
        let mut y: f64 = 0.1;

        while y > x * x * (1. - x * x).powf(3.5) {
            x = random();
            y = random() / 10.;
        }
        let velocity = x * (2.0_f64).sqrt() * (1. + radius * radius).powf(-0.25);
        let (v_x, v_y, v_z) = to_spherical(&velocity);

        particles.push(Particle {
            p: Point { p_x, p_y, p_z },
            v: Velocity { v_x, v_y, v_z },
            mass,
        })
    }
    particles
}

#[cfg(test)]
mod tests {
    use crate::sgalic::halo::*;
    use crate::{config::AU, ic::plummer};

    use super::{nagai, nagai_par, random};

    #[test]
    fn test_random_number_generator() {
        for _ in 0..10 {
            let a: f64 = random();
            let b: f64 = random();

            assert!(a != b);
            println!("{} {}", a, b);
        }
    }

    #[test]
    fn test_plummer() {
        for _ in 0..10 {
            println!("{:?}", plummer(20, Some(10.), Some(10.)));
        }
    }

    #[test]
    fn test_nagai() {
        nagai(10000, Some(AU), Some(1e24));
    }
    #[test]
    fn test_nagai_par() {
        nagai_par(10000, Some(AU), Some(1e24));
    }
}
