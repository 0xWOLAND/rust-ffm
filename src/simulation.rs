extern crate console_error_panic_hook;
use std::panic;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    config::AU,
    fmm::Particle,
    ic::{plummer, spiral_galaxy},
    octree::Grid,
};

#[wasm_bindgen]
pub struct CosmoSim {
    particles: Vec<Particle>,
    g: Grid,
}

mod Helpers {
    use std::iter::once;

    use crate::{
        config::G_CONSTANT,
        fmm::{Particle, Vec3},
        octree::Grid,
    };

    pub fn insert_particle(g: &mut Grid, p: Particle) {
        g.insert_particle(&p.p, p.mass, &|v1: Vec3, v2: Vec3, mass: f64| {
            let mut d = v1.diff(v2);
            let r = [d.x, d.y, d.z]
                .map(|x| x.powi(2))
                .iter()
                .sum::<f64>()
                .sqrt();

            let a = G_CONSTANT * mass / r.powi(2);

            d.x /= r;
            d.y /= r;
            d.z /= r;

            let x = a * d.x;
            let y = a * d.y;
            let z = a * d.z;

            Vec3 { x, y, z }
        });
    }

    pub fn flatten(v: Vec<(f64, f64, f64)>) -> Vec<f64> {
        v.iter()
            .flat_map(|&(a, b, c)| once(a).chain(once(b)).chain(once(c)))
            .collect()
    }
}

#[wasm_bindgen]
impl CosmoSim {
    #[wasm_bindgen(constructor)]
    pub fn new(n: usize, a: f64, M: f64) -> CosmoSim {
        panic::set_hook(Box::new(console_error_panic_hook::hook));

        // let particles = plummer(n, Some(a), Some(M));
        // let mut particles = plummer(0, Some(a), Some(M));
        // particles.append(&mut spiral_galaxy());

        let mut particles = spiral_galaxy();
        // particles.append(&mut plummer(n, Some(a), Some(M)));

        let scale = AU;

        let mut g = Grid::new(scale / 2., scale);
        for particle in &particles {
            Helpers::insert_particle(&mut g, particle.clone());
        }

        CosmoSim { particles, g }
    }

    #[wasm_bindgen]
    pub fn simulate(&mut self, dt: f64) {
        self.g = Grid::new(AU / 10., AU);
        for particle in &self.particles {
            Helpers::insert_particle(&mut self.g, particle.clone());
        }

        let mul_tuple = |tup: (f64, f64, f64), x: f64| (tup.0 * x, tup.1 * x, tup.2 * x);
        self.particles.iter_mut().for_each(|p| {
            let a = self.g.get_acceleration(&p.p);
            p.v += mul_tuple(a, dt);
            p.p += mul_tuple((p.v.x, p.v.y, p.v.z), dt);
        });
    }

    #[wasm_bindgen]
    pub fn get_position(&self) -> js_sys::Float32Array {
        let x: Vec<f32> = Helpers::flatten(
            self.particles
                .iter()
                .map(|particle| (particle.p.x, particle.p.y, particle.p.z))
                .collect::<Vec<(f64, f64, f64)>>(),
        )
        .iter()
        .map(|x| *x as f32)
        .collect();
        js_sys::Float32Array::from(&x[..])
    }

    #[wasm_bindgen]
    pub fn get_velocity(&self) -> js_sys::Float32Array {
        let v: Vec<f32> = Helpers::flatten(
            self.particles
                .iter()
                .map(|p| (p.v.x, p.v.y, p.v.z))
                .collect::<Vec<(f64, f64, f64)>>(),
        )
        .iter()
        .map(|x| *x as f32)
        .collect();
        js_sys::Float32Array::from(&v[..])
    }
}
#[wasm_bindgen]
pub fn get_scale_length() -> f64 {
    AU
}
