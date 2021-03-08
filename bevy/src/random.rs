extern crate rand;
extern crate rand_pcg;

use rand::prelude::*;
//use rand_pcg::Pcg64;

pub fn int_range(a: i32, b: i32) -> i32 {
    let mut rng = thread_rng(); // random seed
                                //let mut rng = Pcg64::seed_from_u64(1); // fixed seed

    //let b: bool = rng.gen();
    //println!("bool {:?}", b);

    //let x: f64 = rng.gen(); // random number in range [0, 1)
    //println!("f64 {:?}", x);

    let n: i32 = rng.gen_range(a..b);
    return n;
}
