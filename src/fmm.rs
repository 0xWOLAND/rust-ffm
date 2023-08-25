use std::ops;

pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn to_cell(&self, range: f64, level: u64) -> Cell {
        let size = range / ((1 << level) as f64);
        let [x, y, z] = [self.x, self.y, self.z].map(|x| ((x + range / 2.) / size) as u64);
        Cell { x, y, z, level }
    }

    pub fn diff(&self, p: Point) -> Point {
        Point {
            x: self.x - p.x,
            y: self.y - p.y,
            z: self.z - p.z,
        }
    }
}

pub struct Cell {
    x: u64,
    y: u64,
    z: u64,
    level: u64,
}

impl Cell {
    pub fn new(x: u64, y: u64, z: u64, level: u64) -> Self {
        Cell { x, y, z, level }
    }

    fn to_array(&self) -> [u64; 3] {
        [self.x, self.y, self.z]
    }

    pub fn equals(&self, c: &Cell) -> bool {
        [self.x, self.y, self.z, self.level]
            .iter()
            .zip([c.x, c.y, c.z, c.level])
            .map(|(a, b)| *a == b)
            .all(|b| b)
    }

    pub fn center(&self, range: f64) -> Point {
        let size = range / ((1 << self.level) as f64);
        let [x, y, z] = self
            .to_array()
            .map(|x| size * (x as f64 + 0.5) - range / 2.);
        Point { x, y, z }
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

    pub fn children(&self) -> [Cell; 8] {
        let level = self.level + 1;
        let [x, y, z] = self.to_array().map(|x| x * 2);
        [
            Cell { x, y, z, level },
            Cell {
                x: x + 1,
                y,
                z,
                level,
            },
            Cell {
                x,
                y: y + 1,
                z,
                level,
            },
            Cell {
                x,
                y,
                z: z + 1,
                level,
            },
            Cell {
                x: x + 1,
                y: y + 1,
                z,
                level,
            },
            Cell {
                x: x + 1,
                y,
                z: z + 1,
                level,
            },
            Cell {
                x,
                y: y + 1,
                z: z + 1,
                level,
            },
            Cell {
                x: x + 1,
                y: y + 1,
                z: z + 1,
                level,
            },
        ]
    }
}
