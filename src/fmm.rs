use std::convert::From;
use std::ops;

#[derive(Debug, Clone)]
pub struct Particle {
    pub p: Vec3,
    pub v: Vec3,
    pub mass: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl ops::AddAssign<(f64, f64, f64)> for Vec3 {
    fn add_assign(&mut self, rhs: (f64, f64, f64)) {
        self.x += rhs.0;
        self.y += rhs.1;
        self.z += rhs.2;
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl From<(f64, f64, f64)> for Vec3 {
    fn from(value: (f64, f64, f64)) -> Self {
        Self {
            x: value.0,
            y: value.1,
            z: value.2,
        }
    }
}
impl Vec3 {
    pub fn new(p_x: f64, p_y: f64, p_z: f64) -> Vec3 {
        Vec3 {
            x: p_x,
            y: p_y,
            z: p_z,
        }
    }
}
impl Vec3 {
    pub fn to_cell(&self, range: f64, level: i64) -> Cell {
        let size = range / ((1 << level) as f64);
        let [x, y, z] = [self.x, self.y, self.z].map(|x| ((x + range / 2.) / size).floor() as i64);
        Cell { x, y, z, level }
    }

    pub fn diff(&self, p: Vec3) -> Vec3 {
        Vec3 {
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

    pub fn center(&self, range: f64) -> Vec3 {
        let size = range / ((1 << self.level) as f64);
        let [x, y, z] = self
            .to_array()
            .map(|x| size * (x as f64 + 0.5) - range / 2.);
        Vec3 { x, y: y, z }
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
