use rand::prelude::*;
use rand_pcg::{Lcg128Xsl64, Pcg64};

pub enum RandGen {
    Random(ThreadRng),
    Seeded(Lcg128Xsl64),
}

pub fn setup_random_seed() -> RandGen {
    RandGen::Random(thread_rng())
}

pub fn setup_fixed_seed() -> RandGen {
    RandGen::Seeded(Pcg64::seed_from_u64(1))
}

pub fn get_usize(rng: &mut RandGen, n: usize) -> usize {
    match rng {
        RandGen::Random(r) => r.gen_range(0..n),
        RandGen::Seeded(r) => r.gen_range(0..n),
    }
}
