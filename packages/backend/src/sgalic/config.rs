use serde_derive::Deserialize;
use std::fs;
use std::process::exit;
use toml;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub halo: HaloConfig,
    pub disk: DiskConfig,
    pub bulge: BulgeConfig,
    pub gas: GasConfig,
    pub global: GlobalConfig,
}

pub const G_CONSTANT: f64 = 6.67e-11;

#[derive(Deserialize, Debug)]
pub struct HaloConfig {
    pub M_halo: f64,
    pub N_halo: i32,
    pub a_halo: f64,
    pub halo_cut_r: f64,
    pub gamma_halo: i32,
}

#[derive(Deserialize, Debug)]
pub struct DiskConfig {
    pub M_disk: f64,
    pub N_disk: i32,
    pub Rd: f64,
    pub z0: f64,
    pub factor: f64,
    pub disk_cut_r: f64,
}

#[derive(Deserialize, Debug)]
pub struct BulgeConfig {
    pub M_bulge: f64,
    pub N_bulge: i32,
    pub a_bulge: f64,
    pub bulge_cut_r: f64,
    pub gamma_bulge: i32,
}

#[derive(Deserialize, Debug)]
pub struct GasConfig {
    pub M_gas: f64,
    pub N_gas: f64,
    pub z0_gas: f64,
    pub Z: f64,
}

#[derive(Deserialize, Debug)]
pub struct GlobalConfig {
    pub N_rho: f64,
    pub rho_max: f64,
    pub Nz: f64,
    pub z_max: f64,
}

pub fn generate_config() -> Config {
    let filename = "src/sgalic/params.toml";
    let contents = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not read file");
            exit(1);
        }
    };
    match toml::from_str(&contents) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Unable to load data");
            exit(1);
        }
    }
}
