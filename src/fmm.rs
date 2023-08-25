use std::ops;

pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point { x, y, z }
    }
    pub fn to_cell(&self, range: f64, level: i64) -> Cell {
        let size = range / ((1 << level) as f64);
        let [x, y, z] = [self.x, self.y, self.z].map(|x| ((x + range / 2.) / size).floor() as i64);
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

#[derive(Debug)]
pub struct Cell {
    x: i64,
    y: i64,
    z: i64,
    level: i64,
}

impl Cell {
    pub fn new(x: i64, y: i64, z: i64, level: i64) -> Self {
        Cell { x, y, z, level }
    }

    pub fn to_array(&self) -> [i64; 3] {
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

    pub fn hash(&self) -> i64 {
        (1 << (3 * self.level))
            + self
                .to_array()
                .iter()
                .enumerate()
                .map(|(i, x)| *x * 1 << (i as i64 * self.level))
                .sum::<i64>()
    }

    pub fn in_bounds(&self) -> bool {
        self.to_array()
            .map(|x| (0..(1 << self.level)).contains(&x))
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

#[cfg(test)]
mod tests {
    use super::Cell;

    #[test]
    fn should_be_in_bounds() {
        let c = Cell::new(0, 1, 1, 1);
        println!("{:?}", c.in_bounds());
    }
}
