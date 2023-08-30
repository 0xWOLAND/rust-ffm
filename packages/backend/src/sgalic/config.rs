pub struct Config {
    pub halo_cut_M: f64,
    pub halo_cut_r: f64,
    pub M_halo: f64,
    pub N_halo: i32,
    pub a_halo: i32,
    pub gamma_halo: i32,
    // Disk Params
    pub disk_cut: f64,
    pub Rd: f64,
    // Bulge Params
    pub bulge_cut_M: f64,
    pub bulge_cut_r: f64,
    pub M_bulge: f64,
    pub N_bulge: i32,
    pub a_bulge: i32,
    pub gamma_bulge: i32,
}
