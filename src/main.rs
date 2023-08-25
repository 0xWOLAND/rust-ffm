use rust_ffm::{config::AU, fmm::Point, octree::Grid};

fn main() {
    let mut g = Grid::new(2., AU);
    let mass = 10.;

    let p_1 = Point::new(0., 0., 0.);
    let p_2 = Point::new(2., 0., 0.);

    g.insert_particle(&p_1, mass);
    g.insert_particle(&p_2, mass);

    //   1    1   1      1
    //   2    2   2      2
    //   4    4   4      3
    //   8    8   8      4
    //  16   16  16      5
    //  32   32  32      6
    //  64   64  64      7
    // 128  128 128      8
    // 256  256 256      9
    let a_1 = g.get_acceleration(&p_1);
    let a_2 = g.get_acceleration(&p_2);

    println!("a_1: {:?}", a_1);
    println!("a_2: {:?}", a_2);
}
