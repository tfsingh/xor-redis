use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct Xor {
    seed: u64,
    block_length: usize,
    fingerprints: Vec<u8>,
}

impl Xor {
    const MAX_ITERATIONS: u32 = 1024;

    pub fn populate(&self, entries: Vec<String>) -> Result<(), ()> {
        unimplemented!()
    }

    pub fn contains(&self, entry: String) -> bool {
        true
    }

    fn fingerprint(hash: u64) -> u64 {
        unimplemented!()
    }

    fn hash_entry(entry: String) -> u64 {
        let mut hasher = DefaultHasher::new();
        entry.hash(&mut hasher);
        hasher.finish()
    }
}
