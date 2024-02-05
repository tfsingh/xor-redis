use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct Xor {
    seed: u64,
    block_length: u32,
    fingerprints: Vec<u8>,
}

#[derive(Debug)]
struct Hashes {
    h: u64,
    h0: u32,
    h1: u32,
    h2: u32,
}

#[derive(Debug)]
struct KeyIndex {
    hash: u64,
    index: u32,
}

#[derive(Debug)]
struct XorSet {
    xormask: u64,
    count: u32,
}

impl Xor {
    const MAX_ITERATIONS: u32 = 1024;

    pub fn populate(entries: Vec<String>) -> Result<Self, ()> {
        let hashed_entries: Vec<u64> = entries.iter().map(|entry| hash_entry(entry)).collect();

        unimplemented!()
    }

    pub fn contains(&self, entry: &str) -> bool {
        let entry: u64 = hash_entry(&entry);
        let hash = mixsplit(entry, self.seed);
        let f = fingerprint(hash) as u8;
        let r0 = hash as u32;
        let r1 = rotl64(hash, 21) as u32;
        let r2 = rotl64(hash, 42) as u32;
        let h0 = reduce(r0, self.block_length);
        let h1 = reduce(r1, self.block_length) + self.block_length;
        let h2 = reduce(r2, self.block_length) + 2 * self.block_length;
        f == (self.fingerprints[h0 as usize]
            ^ self.fingerprints[h1 as usize]
            ^ self.fingerprints[h2 as usize])
    }

    fn geth0h1h2(&self, k: u64) -> Hashes {
        let hash = mixsplit(k, self.seed);
        let r0 = hash as u32;
        let r1 = rotl64(hash, 21) as u32;
        let r2 = rotl64(hash, 42) as u32;

        Hashes {
            h: hash,
            h0: reduce(r0, self.block_length),
            h1: reduce(r1, self.block_length),
            h2: reduce(r2, self.block_length),
        }
    }

    fn geth0(&self, hash: u64) -> u32 {
        let r0 = hash as u32;
        reduce(r0, self.block_length)
    }

    fn geth1(&self, hash: u64) -> u32 {
        let r1 = rotl64(hash, 21) as u32;
        reduce(r1, self.block_length)
    }

    fn geth2(&self, hash: u64) -> u32 {
        let r2 = rotl64(hash, 42) as u32;
        reduce(r2, self.block_length)
    }
}

fn murmur64(mut h: u64) -> u64 {
    h ^= h >> 33;
    h *= 0xff51afd7ed558ccd;
    h ^= h >> 33;
    h *= 0xc4ceb9fe1a85ec53;
    h ^= h >> 33;
    h
}

fn splitmix64(seed: &mut u64) -> u64 {
    *seed += 0x9E3779B97F4A7C15;
    let mut z = *seed;
    z = (z ^ (z >> 30)) * 0xBF58476D1CE4E5B9;
    z = (z ^ (z >> 27)) * 0x94D049BB133111EB;
    z ^ (z >> 31)
}

fn mixsplit(key: u64, seed: u64) -> u64 {
    murmur64(key + seed)
}

fn rotl64(n: u64, c: i64) -> u64 {
    (n << (c & 63)) | (n >> (-c & 63))
}

fn reduce(hash: u32, n: u32) -> u32 {
    ((hash as u64 * n as u64) >> 32) as u32
}

fn fingerprint(hash: u64) -> u64 {
    hash ^ (hash >> 32)
}

fn hash_entry(entry: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    entry.hash(&mut hasher);
    hasher.finish()
}
