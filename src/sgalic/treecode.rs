#[derive(Clone, Copy)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn ZERO() -> Vec3 {
        Vec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }

    pub fn to_array(&self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }
}

pub struct Octree {
    side: Vec3,
    center: Vec3,
    CoM: Vec3,
    mass: f64,
    branches: Option<Vec<Octree>>,
    childless: bool,
}

const DELTAS: [[i32; 3]; 8] = [
    [1, 1, 1],
    [-1, 1, 1],
    [1, -1, 1],
    [1, 1, -1],
    [-1, -1, 1],
    [-1, 1, -1],
    [1, -1, -1],
    [-1, -1, -1],
];

impl Octree {
    pub fn new(side: Vec3, center: Option<Vec3>, CoM: Option<Vec3>, mass: Option<f64>) -> Octree {
        let center = center.unwrap_or_else(|| Vec3::ZERO());
        let CoM = CoM.unwrap_or_else(|| Vec3::ZERO());
        let mass = mass.unwrap_or_else(|| 0.);
        let branches = None;
        let childless = true;

        Octree {
            side,
            center,
            CoM,
            mass,
            branches,
            childless,
        }
    }

    pub fn find_place(&self, pos: Vec3) -> i32 {
        let signs = pos
            .to_array()
            .iter()
            .zip(self.center.to_array())
            .map(|(&a, b)| (a >= b) as i32)
            .collect::<Vec<i32>>();

        let array_equal = |a: Vec<i32>, b: Vec<i32>| {
            a.iter()
                .zip(b)
                .map(|(&a, b)| a == b)
                .collect::<Vec<bool>>()
                .iter()
                .all(|&x| x) as bool
        };

        let mut index: i32 = 0;
        while index < 7 && !(array_equal(signs.to_owned(), Vec::from(DELTAS[index as usize]))) {
            index += 1;
        }
        index
    }

    pub fn insert(&mut self, pos: Vec3, mass: f64) {
        let mut insertion_pos_list: Vec<Vec3> = vec![pos];
        let mut insertion_mass_list: Vec<f64> = vec![mass];
        if self.childless && self.mass > 0. {
            insertion_pos_list.push(self.CoM);
            insertion_mass_list.push(self.mass);
        }
        self.childless = false;

        for (&p, m) in insertion_pos_list.iter().zip(insertion_mass_list) {
            let index = self.find_place(p);
        }
    }
}
