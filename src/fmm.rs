pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    pub fn to_cell(&self, range: f64, level: u64) -> Cell {
        let size = range / ((1 << level) as f64);
        let [x, y, z] = [self.x, self.y, self.z].map(|x| ((x + range / 2.) / size) as u64);
        Cell { x, y, z, level }
    }
}

pub struct Cell {
    x: u64,
    y: u64,
    z: u64,
    level: u64,
}

impl Cell {
    fn to_array(&self) -> [u64; 3] {
        [self.x, self.y, self.z]
    }

    pub fn hash(&self) -> u64 {
        (1 << (3 * self.level))
            * self
                .to_array()
                .iter()
                .enumerate()
                .map(|(i, x)| *x * 1 << ((i + 1) as u64 * self.level))
                .sum::<u64>()
    }

    pub fn in_bounds(&self, level: u64) -> bool {
        self.to_array()
            .map(|x| (1..=(1 << level)).contains(&x))
            .iter()
            .all(|x| *x)
    }
}
