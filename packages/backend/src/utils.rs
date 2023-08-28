use crate::{
    config::{AU, IMAGE_DIM},
    fmm::Particle,
};

pub fn to_texture(a: &Vec<Particle>, width: usize, height: usize) -> js_sys::Uint8Array {
    let mut image: Vec<u8> = vec![0; width * height];

    a.iter().for_each(|particle| {
        let p = &particle.p;
        let pixel = 3 * ((p.p_x * (width as f64 / AU) + (p.p_y * (height as f64 / AU))) as usize);
        image[pixel..pixel + 3].fill(255);
    });

    js_sys::Uint8Array::from(&image[..])
}
