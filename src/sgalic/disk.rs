use crate::{
    sgalic::{
        cmf::{disk_height_cmf_inv, disk_radial_cmf_inv},
        velocity::random,
    },
    utils::{gen_random_array, to_circular},
};
#[cfg(feature = "parallel")]
use rayon::prelude::*;

#[cfg(feature = "parallel")]
pub use wasm_bindgen_rayon::init_thread_pool;

use super::{cmf::disk_radial_cmf, config::Config};

#[cfg(not(feature = "rayon"))]
pub fn set_disk_positions(config: &Config) -> (Vec<(f64, f64, f64)>, f64) {
    let N = config.disk.N_disk as usize;
    let z0 = config.disk.z0;
    let Rd = config.disk.Rd;
    let disk_cut = config.disk.disk_cut_r;

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
        .map(|r| random() * disk_radial_cmf_inv(r, &Rd))
        .collect::<Vec<f64>>();

    let zs = gen_random_array(N)
        .iter()
        .map(|frac| disk_height_cmf_inv(frac, &z0))
        .collect::<Vec<f64>>();
    let xys = radii
        .iter()
        .map(|r| to_circular(r))
        .collect::<Vec<(f64, f64)>>();

    (
        xys.iter()
            .zip(zs)
            .map(|((x, y), z)| (*x, *y, z))
            .collect::<Vec<(f64, f64, f64)>>(),
        disk_cut,
    )
}

#[cfg(feature = "rayon")]
pub fn set_disk_positions(config: &Config) -> (Vec<(f64, f64, f64)>, f64) {
    let N = config.disk.N_disk as usize;
    let z0 = config.disk.z0;
    let Rd = config.disk.Rd;
    let disk_cut = config.disk.disk_cut_r;

    let disk_cut = disk_radial_cmf(&disk_cut, &Rd);
    println!(
        "{:?}% of disk mass cut by the truncation...",
        (100. * (1. - disk_cut))
    );
    let sample = gen_random_array(N)
        .par_iter()
        .map(|x| disk_cut * x)
        .collect::<Vec<f64>>();
    let radii = sample
        .par_iter()
        .map(|r| random() * disk_radial_cmf_inv(r, &Rd))
        .collect::<Vec<f64>>();

    let zs = gen_random_array(N)
        .par_iter()
        .map(|frac| disk_height_cmf_inv(frac, &z0))
        .collect::<Vec<f64>>();
    let xys = radii
        .par_iter()
        .map(|r| to_circular(r))
        .collect::<Vec<(f64, f64)>>();

    (
        xys.par_iter()
            .zip(zs)
            .map(|((x, y), z)| (*x, *y, z))
            .collect::<Vec<(f64, f64, f64)>>(),
        disk_cut,
    )
}
