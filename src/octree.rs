use crate::{
    config::G_CONSTANT,
    fmm::{Cell, Point},
};

pub struct Grid {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub z: Vec<f64>,

    range: f64,
    max_depth: usize,
    max_hash_val: u64,
}

impl Grid {
    pub fn new(resolution: f64, range: f64) -> Grid {
        // exclusive
        let max_depth = ((range / resolution).log2().ceil() + 1.) as usize;
        let max_hash_val = 3 * 1 << ((range / resolution) as u64);

        let x = vec![0.; max_hash_val];
        let y = vec![0.; max_hash_val];
        let z = vec![0.; max_hash_val];

        Grid {
            x,
            y,
            z,
            range,
            max_depth,
            max_hash_val: max_hash_val as u64,
        }
    }

    fn neighbors(&self, p: &Point) -> Vec<Cell> {
        let cells = self.get_nested_cells(p);
        let mut _cell = &Cell::new(0, 0, 0, 0);

        let mut neighbors = Vec::<Cell>::new();

        for level in 0..cells.len() {
            let children = _cell.children();
            let cell = cells.get(level).unwrap();
            for child in children {
                if !child.equals(cell) && child.in_bounds(level as u64) {
                    neighbors.push(child);
                }
            }
            _cell = cell;
        }
        neighbors
    }

    fn get_nested_cells(&self, p: &Point) -> Vec<Cell> {
        let mut nested_cells = Vec::<Cell>::new();
        for level in (1..=self.max_depth).map(|x| x as u64) {
            let cell = p.to_cell(self.range, level);
            if cell.in_bounds(level) {
                nested_cells.push(cell);
            }
        }
        nested_cells
    }

    pub fn insert_particle(&mut self, p: &Point, mass: f64) {
        for cell in self.neighbors(&p) {
            let hash = cell.hash() as usize;
            let mut d = p.diff(cell.center(self.range));
            let r = [d.x, d.y, d.z]
                .map(|x| x.powi(2))
                .iter()
                .sum::<f64>()
                .sqrt();

            let a = G_CONSTANT * mass / r.powi(2);

            d.x /= r;
            d.y /= r;
            d.z /= r;

            let a_x = a * d.x;
            let a_y = a * d.y;
            let a_z = a * d.z;

            self.x[hash] += a_x;
            self.y[hash] += a_y;
            self.z[hash] += a_z;
        }
    }

    pub fn get_acceleration(&self, p: &Point) -> (f64, f64, f64) {
        let _cells = self.get_nested_cells(p);

        let mut x = 0.;
        let mut y = 0.;
        let mut z = 0.;

        for _cell in _cells {
            let hash = _cell.hash() as usize;
            x += self.x[hash];
            y += self.y[hash];
            z += self.z[hash];
        }

        (x, y, z)
    }
}
