use std::sync::{Arc, Mutex};

#[cfg(feature = "parallel")]
use rayon::prelude::*;

#[cfg(feature = "parallel")]
pub use wasm_bindgen_rayon::init_thread_pool;

use crate::{
    fmm::{Particle, Vec3},
    sgalic::generate::generate_galaxy,
    utils::{random, to_spherical},
};

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
            p: Vec3 {
                x: p_x,
                y: p_y,
                z: p_z,
            },
            v: Vec3 {
                x: v_x,
                y: v_y,
                z: v_z,
            },
            mass,
        })
    }
    particles
}

#[cfg(not(feature = "rayon"))]
pub fn spiral_galaxy() -> Vec<Particle> {
    let (pos, vels, masses) = generate_galaxy();
    pos.iter()
        .zip(vels)
        .zip(masses)
        .map(|((&pos, vel), mass)| Particle {
            p: Vec3::from(pos),
            v: Vec3::from(vel),
            mass,
        })
        .collect()
}

#[cfg(feature = "rayon")]
pub fn spiral_galaxy() -> Vec<Particle> {
    let (pos, vels, masses) = generate_galaxy();
    pos.par_iter()
        .zip(vels)
        .zip(masses)
        .map(|((&pos, vel), mass)| Particle {
            p: Vec3::from(pos),
            v: Vec3::from(vel),
            mass,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::ic::spiral_galaxy;
    use crate::sgalic::halo::*;
    use crate::utils::random;
    use crate::{config::AU, ic::plummer};

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
    fn test_spiral_galaxy() {
        println!("{:?}", spiral_galaxy());
    }
}
