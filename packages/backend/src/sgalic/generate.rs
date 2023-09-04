use super::{
    bulge::set_bulge_positions, config::generate_config, disk::set_disk_positions,
    halo::set_halo_positions,
};

pub fn generate_galaxy() {
    let config = generate_config();
    let (coord_stars, disk_cut) = set_disk_positions(&config);
    let coords_bulge = set_bulge_positions(&config);
    let (coords_halo, halo_cut_M) = set_halo_positions(&config);
}
#[cfg(test)]
mod tests {
    use super::generate_galaxy;

    #[test]
    fn test() {
        generate_galaxy();
    }
}
