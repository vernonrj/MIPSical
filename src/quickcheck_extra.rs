#[cfg(test)]
use rand;
#[cfg(test)]
use quickcheck::{QuickCheck, StdGen};

pub fn masked_quickcheck(size: usize) -> QuickCheck<StdGen<rand::ThreadRng>> {
    QuickCheck::new().gen(StdGen::new(rand::thread_rng(), size))
}
