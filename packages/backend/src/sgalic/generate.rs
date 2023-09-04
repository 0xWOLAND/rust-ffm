use super::{
    bulge::set_bulge_positions,
    config::generate_config,
    disk::set_disk_positions,
    halo::set_halo_positions,
    velocity::{keplerian, plummer},
};

pub fn generate_galaxy() -> (Vec<(f64, f64, f64)>, Vec<(f64, f64, f64)>, Vec<f64>) {
    let config = generate_config();
    let (coords_disk, disk_cut) = set_disk_positions(&config);
    let (coords_bulge, bulge_cut_M) = set_bulge_positions(&config);
    let (coords_halo, halo_cut_M) = set_halo_positions(&config);

    let n_disk = config.disk.N_disk as usize;
    let n_bulge = config.bulge.N_bulge as usize;
    let n_halo = config.halo.N_halo as usize;

    assert!(coords_disk.len() == n_disk);
    assert!(coords_bulge.len() == n_bulge);
    assert!(coords_halo.len() == n_halo);

    let mass = config.disk.M_disk / 10.;

    let vels = [
        keplerian(&coords_disk, mass),
        plummer(&coords_bulge),
        plummer(&coords_halo),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<(f64, f64, f64)>>();

    let pos = [coords_disk, coords_bulge, coords_halo]
        .into_iter()
        .flatten()
        .collect::<Vec<(f64, f64, f64)>>();

    let masses = vec![
        vec![disk_cut; n_disk],
        vec![bulge_cut_M; n_bulge],
        vec![halo_cut_M; n_halo],
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<f64>>();

    (pos, vels, masses)
}
#[cfg(test)]
mod tests {
    use super::generate_galaxy;

    #[test]
    fn test() {
        generate_galaxy();
    }
}
