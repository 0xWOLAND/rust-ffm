use crate::{
    sgalic::cmf::dehnen_cmf_inv,
    utils::{gen_random_array, to_spherical},
};

use super::{cmf::dehnen_cmf, config::Config};

pub fn set_bulge_positions(config: &mut Config) -> Vec<(f64, f64, f64)> {
    let bulge_cut_r = config.bulge_cut_r;
    let M_bulge = config.M_bulge;
    let N_bulge = config.N_bulge as usize;
    let a_bulge = config.a_bulge;
    let gamma_bulge = config.gamma_bulge;

    let bulge_cut_M = dehnen_cmf(bulge_cut_r, M_bulge, a_bulge, gamma_bulge);

    println!(
        "{:?}% of bulge mass cut by the truncation...",
        (100. * (1. - bulge_cut_M) / M_bulge)
    );
    let Mc = gen_random_array(N_bulge)
        .iter()
        .map(|x| bulge_cut_M * (*x))
        .collect::<Vec<f64>>();

    let radii = dehnen_cmf_inv(Mc, M_bulge, a_bulge, gamma_bulge);

    radii
        .iter()
        .map(|r| to_spherical(r))
        .collect::<Vec<(f64, f64, f64)>>()
}
