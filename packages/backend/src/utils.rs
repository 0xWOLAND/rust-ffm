use crate::{
    config::{AU, IMAGE_DIM},
    fmm::Particle,
};

pub fn to_texture(a: Vec<Particle>) -> js_sys::Uint8Array {
    let mut image: [u8; IMAGE_DIM] = [0; IMAGE_DIM];

    a.iter().for_each(|particle| {
        let p = &particle.p;
        let pixel = 3 * (p.p_x + p.p_y * AU + p.p_z * AU * AU) as usize;
        image[pixel..pixel + 3].fill(255);
    });

    js_sys::Uint8Array::from(&image[..])
}
