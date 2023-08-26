use std::f64::consts::PI;

use rand::{thread_rng, Rng};
use rand_distr::Normal;

use crate::fmm::{Particle, Point, Velocity};

fn random() -> f64 {
    thread_rng().gen::<f64>()
}

fn to_spherical(r: f64) -> (f64, f64, f64) {
    let theta = (2. * random() - 1.).acos();
    let phi = 2. * PI * random();

    let x = r * theta.sin() * phi.cos();
    let y = r * theta.sin() * phi.sin();
    let z = r * theta.cos();

    (x, y, z)
}

pub fn plummer(n: usize, a: Option<f64>, M: Option<f64>) -> Vec<Particle> {
    let mut particles = Vec::<Particle>::new();

    for _ in 0..n {
        let mass = M.unwrap_or_else(|| 1.0) / (n as f64);
        let radius =
            a.unwrap_or_else(|| 1.) / ((M.unwrap_or_else(|| 1.0) / mass).powf(2. / 3.) - 1.).sqrt();
        let (p_x, p_y, p_z) = to_spherical(radius);

        let mut x: f64 = 0.0;
        let mut y: f64 = 0.1;

        while y > x * x * (1. - x * x).powf(3.5) {
            x = random();
            y = random() / 10.;
        }
        let velocity = x * (2.0_f64).sqrt() * (1. + radius * radius).powf(-0.25);
        let (v_x, v_y, v_z) = to_spherical(velocity);

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
    use crate::ic::plummer;

    use super::random;

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
}
