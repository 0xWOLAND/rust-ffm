extern crate console_error_panic_hook;
use std::panic;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    config::AU,
    fmm::{Particle, Point},
    ic::plummer,
    octree::Grid,
    utils::to_texture,
};

#[wasm_bindgen]
pub struct CosmoSim {
    n: usize,
    a: f64,
    M: f64,
    width: usize,
    height: usize,
    particles: Vec<Particle>,
    g: Grid,
}

#[wasm_bindgen]
impl CosmoSim {
    #[wasm_bindgen(constructor)]
    pub fn new(n: usize, a: f64, M: f64, width: usize, height: usize) -> CosmoSim {
        panic::set_hook(Box::new(console_error_panic_hook::hook));

        let particles = plummer(n, Some(a), Some(M));

        let mut g = Grid::new(AU / 10., AU);
        for particle in &particles {
            g.insert_particle(&particle.p, particle.mass);
        }

        CosmoSim {
            n,
            a,
            M,
            width,
            height,
            particles,
            g,
        }
    }

    #[wasm_bindgen]
    pub fn simulate(&mut self, dt: f64) -> js_sys::Uint8Array {
        let mul_tuple = |tup: (f64, f64, f64), x: f64| (tup.0 * x, tup.1 * x, tup.2 * x);
        self.particles.iter_mut().for_each(|p| {
            let a = self.g.get_acceleration(&p.p);
            p.v += mul_tuple(a, dt);
            p.p += mul_tuple((p.v.v_x, p.v.v_y, p.v.v_z), dt);
        });
        to_texture(&self.particles, self.width, self.height)
    }
}
#[wasm_bindgen]
pub fn get_scale_length() -> f64 {
    AU
}

#[cfg(test)]
mod tests {

    use crate::{config::AU, fmm::Point, octree::Grid};

    #[test]
    fn test_two_particle() {
        let mut g = Grid::new(2., AU);
        let mass = 10.;

        let p_1 = Point::new(0., 0., 0.);
        let p_2 = Point::new(2., 0., 0.);

        g.insert_particle(&p_1, mass);
        g.insert_particle(&p_2, mass);

        let a_1 = g.get_acceleration(&p_1);
        let a_2 = g.get_acceleration(&p_2);

        println!("a_1: {:?}", a_1);
        println!("a_2: {:?}", a_2);
    }
}
