use crate::{
    sgalic::cmf::{disk_height_cmf_inv, disk_radial_cmf_inv},
    utils::{gen_random_array, to_circular},
};

use super::{cmf::disk_radial_cmf, config::Config};

pub fn set_disk_positions(N: usize, z0: f64, config: Config) -> Vec<(f64, f64, f64)> {
    let Rd = config.Rd;
    let disk_cut = config.disk_cut;

    let disk_cut = disk_radial_cmf(&disk_cut, &Rd);
    println!(
        "{:?}% of disk mass cut by the truncation...",
        (100. * (1. - disk_cut))
    );
    let sample = gen_random_array(N)
        .iter()
        .map(|x| disk_cut * x)
        .collect::<Vec<f64>>();
    let radii = sample
        .iter()
        .map(|r| disk_radial_cmf_inv(r, &Rd))
        .collect::<Vec<f64>>();

    let zs = gen_random_array(N)
        .iter()
        .map(|frac| disk_height_cmf_inv(frac, &z0))
        .collect::<Vec<f64>>();
    let xys = gen_random_array(N)
        .iter()
        .map(|r| to_circular(r))
        .collect::<Vec<(f64, f64)>>();

    xys.iter()
        .zip(zs)
        .map(|((x, y), z)| (*x, *y, z))
        .collect::<Vec<(f64, f64, f64)>>()
}
