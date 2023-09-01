use crate::octree::Grid;

use super::config::Config;

pub fn fill_potential_grid(coords_stars: Vec<(f64, f64, f64)>, config: Config) {
    let grav_tree = Grid::new(1., (config.a_halo * 400) as f64);
}
