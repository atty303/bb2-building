use std::collections::BTreeSet;
use std::hash::{BuildHasher, Hasher};
use std::hash::Hash;

pub struct IdHash {
    pub seed: u64,
    bits: usize,
}

impl IdHash {
    pub fn new(seed: u64, bits: usize) -> Self {
        Self {
            seed,
            bits,
        }
    }

    pub fn id_hash<T: Hash>(&self, v: &T) -> u64 {
        let mut hasher = ahash::RandomState::with_seeds(self.seed, 0, 0, 0).build_hasher();
        v.hash(&mut hasher);
        hasher.finish() >> (64 - self.bits)
    }

    pub fn search_seed<T: Hash, I: IntoIterator<Item = T> + Copy>(&mut self, values: I)
    {
        let max = 10000;
        'seed: for s in 0..max {
            self.seed = s;
            let mut used = BTreeSet::<u64>::new();;
            for v in values {
                if !used.insert(self.id_hash(&v)) {
                    continue 'seed;
                }
            }
            break 'seed;
        }
        if self.seed == max - 1 {
            panic!("failed to find seed");
        }
    }
}
