use crate::fmm::{Cell, Point};

pub struct Grid {
    x: Vec<u64>,
    y: Vec<u64>,
    z: Vec<u64>,

    max_depth: usize,
    max_hash_val: u64,
}

impl Grid {
    pub fn new(resolution: f64, range: f64) -> Grid {
        // exclusive
        let max_depth = ((range / resolution).log2().ceil() + 1.) as usize;
        let max_hash_val = 3 * 1 << ((range / resolution) as u64);

        let x = vec![0; max_hash_val];
        let y = vec![0; max_hash_val];
        let z = vec![0; max_hash_val];

        Grid {
            x,
            y,
            z,
            max_depth,
            max_hash_val: max_hash_val as u64,
        }
    }

    fn neighbors(p: Point) -> Vec<Cell> {
        todo!()
    }

    pub fn insert(p: Point, v: f64) {}
}
