use crate::{config::AU, fmm::Particle};

pub fn to_texture(a: &Vec<Particle>, width: usize, height: usize) -> js_sys::Uint8Array {
    let mut image: Vec<u8> = vec![0; width * height];

    a.iter().for_each(|particle| {
        let p = &particle.p;
        let pixel = 3 * ((p.p_x * (width as f64 / AU) + (p.p_y * (height as f64 / AU))) as usize);
        image[pixel..pixel + 3].fill(255);
    });

    js_sys::Uint8Array::from(&image[..])
}

pub fn flatten(v: Vec<(f64, f64, f64)>) -> Vec<f64> {
    v.iter()
        .flat_map(|tup| [tup.0, tup.1, tup.2].iter().cloned().collect::<Vec<f64>>())
        .collect::<Vec<f64>>()
}
