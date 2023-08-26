use std::f64::consts::PI;

use rand::{thread_rng, Rng};
use rand_distr::Normal;

use crate::fmm::{Particle, Point, Velocity};

fn random() -> f64 {
    thread_rng().gen::<f64>()
}

pub fn plummer(n: usize) -> Vec<Particle> {
    let mut particles = Vec::<Particle>::new();

    for _ in 0..n {
        let mass = 1.0 / (n as f64);
        let radius = (random().powf(-2. / 3.) - 1.).sqrt().recip();
        let theta = (2. * random() - 1.).acos();
        let phi = 2. * PI * random();

        let p_x = radius * theta.sin() * phi.cos();
        let p_y = radius * theta.sin() * phi.sin();
        let p_z = radius * theta.cos();

        let mut x: f64 = 0.0;
        let mut y: f64 = 0.1;

        while y > x * x * (1. - x * x).powf(3.5) {
            x = random();
            y = random() / 10.;
        }
        let velocity = x * (2.0_f64).sqrt() * (1. + radius * radius).powf(-0.25);
        let theta = (2. * random() - 1.).acos();
        let phi = 2. * PI * random();

        let v_x = velocity * theta.sin() * phi.cos();
        let v_y = velocity * theta.sin() * phi.sin();
        let v_z = velocity * theta.cos();

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
}
