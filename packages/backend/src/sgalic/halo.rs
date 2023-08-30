use crate::{
    ic::to_spherical,
    sgalic::utils::dehnen_cmf_inverse,
    utils::{gen_random_array, random},
};

use super::{config::Config, utils::dehnen_cmf};

pub fn set_halo_positions(config: Config) -> Vec<(f64, f64, f64)> {
    let halo_cut_r = config.halo_cut_r;
    let M_halo = config.M_halo;
    let N_halo = config.N_halo as usize;
    let a_halo = config.a_halo;
    let gamma_halo = config.gamma_halo;

    let halo_cut_M = dehnen_cmf(halo_cut_r, M_halo, a_halo, gamma_halo);

    println!(
        "{:?}% of halo mass cut by the truncation...",
        (100. * (1. - halo_cut_M) / M_halo)
    );
    let Mc = gen_random_array(N_halo)
        .iter()
        .map(|x| halo_cut_M * (*x))
        .collect::<Vec<f64>>();

    let radii = dehnen_cmf_inverse(Mc, M_halo, a_halo, gamma_halo);
    let coords: Vec<(f64, f64, f64)> = radii.iter().map(|r| to_spherical(*r)).collect();
    coords
}
