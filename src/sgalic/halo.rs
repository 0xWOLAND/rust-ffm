use crate::sgalic::{
    cmf::{dehnen_cmf, dehnen_cmf_inv},
    velocity::{gen_random_array, to_spherical},
};

use super::config::Config;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

#[cfg(feature = "parallel")]
pub use wasm_bindgen_rayon::init_thread_pool;

#[cfg(not(feature = "rayon"))]
pub fn set_halo_positions(config: &Config) -> (Vec<(f64, f64, f64)>, f64) {
    let halo_cut_r = config.halo.halo_cut_r;
    let M_halo = config.halo.M_halo;
    let N_halo = config.halo.N_halo as usize;
    let a_halo = config.halo.a_halo;
    let gamma_halo = config.halo.gamma_halo;

    let halo_cut_M = dehnen_cmf(halo_cut_r, M_halo, a_halo, gamma_halo);

    println!(
        "{:?}% of halo mass cut by the truncation...",
        (100. * (1. - halo_cut_M) / M_halo)
    );
    let Mc = gen_random_array(N_halo)
        .iter()
        .map(|x| halo_cut_M * (*x))
        .collect::<Vec<f64>>();

    let radii = dehnen_cmf_inv(Mc, M_halo, a_halo, gamma_halo);

    (
        radii
            .iter()
            .map(|r| to_spherical(r))
            .collect::<Vec<(f64, f64, f64)>>(),
        halo_cut_M,
    )
}

#[cfg(feature = "rayon")]
pub fn set_halo_positions(config: &Config) -> (Vec<(f64, f64, f64)>, f64) {
    let halo_cut_r = config.halo.halo_cut_r;
    let M_halo = config.halo.M_halo;
    let N_halo = config.halo.N_halo as usize;
    let a_halo = config.halo.a_halo;
    let gamma_halo = config.halo.gamma_halo;

    let halo_cut_M = dehnen_cmf(halo_cut_r, M_halo, a_halo, gamma_halo);

    println!(
        "{:?}% of halo mass cut by the truncation...",
        (100. * (1. - halo_cut_M) / M_halo)
    );
    let Mc = gen_random_array(N_halo)
        .par_iter()
        .map(|x| halo_cut_M * (*x))
        .collect::<Vec<f64>>();

    let radii = dehnen_cmf_inv(Mc, M_halo, a_halo, gamma_halo);

    (
        radii
            .par_iter()
            .map(|r| to_spherical(r))
            .collect::<Vec<(f64, f64, f64)>>(),
        halo_cut_M,
    )
}
