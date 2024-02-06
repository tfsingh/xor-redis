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

#[derive(Debug, Default, Clone)]
struct KeyIndex {
    hash: u64,
    index: u32,
}

#[derive(Debug, Default, Clone)]
struct XorSet {
    xormask: u64,
    count: u32,
}

impl Xor {
    const MAX_ITERATIONS: u32 = 1024;

    pub fn populate(entries: Vec<String>) -> Result<Self, String> {
        let mut keys: Vec<u64> = entries.iter().map(|entry| hash_entry(entry)).collect();

        let mut size = keys.len();
        if size == 0 {
            return Err(String::from("Filter requires at least one entry"));
        }

        let mut capacity = 32 + ((1.23 * size as f64).ceil() as usize);
        capacity = capacity / 3 * 3;

        let mut rngcounter: u64 = 1;
        let mut filter = Xor {
            seed: splitmix64(&mut rngcounter),
            block_length: (capacity as u32 / 3),
            fingerprints: vec![0; capacity as usize],
        };
        let block_length = (capacity / 3) as usize;

        let mut stack: Vec<KeyIndex> = vec![KeyIndex::default(); size];
        let mut q0: Vec<KeyIndex> = vec![KeyIndex::default(); block_length];
        let mut q1: Vec<KeyIndex> = vec![KeyIndex::default(); block_length];
        let mut q2: Vec<KeyIndex> = vec![KeyIndex::default(); block_length];

        let mut sets0: Vec<XorSet> = vec![XorSet::default(); block_length];
        let mut sets1: Vec<XorSet> = vec![XorSet::default(); block_length];
        let mut sets2: Vec<XorSet> = vec![XorSet::default(); block_length];

        let mut iterations: u32 = 0;

        loop {
            iterations += 1;
            if iterations > Self::MAX_ITERATIONS {
                return Err(String::from("Too many iterations"));
            }

            let mut i = 0;
            while i < size {
                let key = &keys[i];
                let hs = filter.geth0h1h2(*key);
                sets0[hs.h0 as usize].xormask ^= hs.h;
                sets0[hs.h0 as usize].count += 1;
                sets1[hs.h1 as usize].xormask ^= hs.h;
                sets1[hs.h1 as usize].count += 1;
                sets2[hs.h2 as usize].xormask ^= hs.h;
                sets2[hs.h2 as usize].count += 1;
                i += 1;
            }

            let mut q0_size = scan_count(&mut q0, &sets0) as usize;
            let mut q1_size = scan_count(&mut q1, &sets1) as usize;
            let mut q2_size = scan_count(&mut q2, &sets2) as usize;

            let mut stack_size: usize = 0;

            while q0_size + q1_size + q2_size > 0 {
                while q0_size > 0 {
                    q0_size -= 1;
                    let keyindexvar = &q0[q0_size];
                    let index = keyindexvar.index;
                    if sets0[index as usize].count == 0 {
                        continue;
                    }
                    let hash = keyindexvar.hash;
                    let h1 = filter.geth1(hash) as usize;
                    let h2 = filter.geth2(hash) as usize;
                    stack[stack_size] = keyindexvar.clone();
                    stack_size += 1;
                    sets1[h1].xormask ^= hash;

                    sets1[h1].count -= 1;
                    if sets1[h1].count == 1 {
                        q1[q1_size].index = h1 as u32;
                        q1[q1_size].hash = sets1[h1].xormask;
                        q1_size += 1;
                    }

                    sets2[h2].xormask ^= hash;
                    sets2[h2].count -= 1;
                    if sets2[h2].count == 1 {
                        q2[q2_size].index = h2 as u32;
                        q2[q2_size].hash = sets2[h2].xormask;
                        q2_size += 1;
                    }
                }
                while q1_size > 0 {
                    q1_size -= 1;
                    let keyindexvar = &mut q1[q1_size];
                    let index = keyindexvar.index as usize;
                    if sets1[index].count == 0 {
                        continue;
                    }
                    let hash = keyindexvar.hash;
                    let h0 = filter.geth0(hash) as usize;
                    let h2 = filter.geth2(hash) as usize;
                    keyindexvar.index += filter.block_length as u32;
                    stack[stack_size] = keyindexvar.clone();
                    stack_size += 1;
                    sets0[h0].xormask ^= hash;
                    sets0[h0].count -= 1;

                    if sets0[h0].count == 1 {
                        q0[q0_size].index = h0 as u32;
                        q0[q0_size].hash = sets0[h0].xormask;
                        q0_size += 1;
                    }

                    sets2[h2].xormask ^= hash;
                    sets2[h2].count -= 1;
                    if sets2[h2].count == 1 {
                        q2[q2_size].index = h2 as u32;
                        q2[q2_size].hash = sets2[h2].xormask;
                        q2_size += 1;
                    }
                }
                while q2_size > 0 {
                    q2_size -= 1;
                    let keyindexvar = &mut q2[q2_size];
                    let index = keyindexvar.index as usize;
                    if sets2[index].count == 0 {
                        continue;
                    }
                    let hash = keyindexvar.hash;
                    let h0 = filter.geth0(hash) as usize;
                    let h1 = filter.geth1(hash) as usize;
                    keyindexvar.index += 2 * filter.block_length as u32;
                    stack[stack_size] = keyindexvar.clone();
                    stack_size += 1;
                    sets0[h0].xormask ^= hash;
                    sets0[h0].count -= 1;

                    if sets0[h0].count == 1 {
                        q0[q0_size].index = h0 as u32;
                        q0[q0_size].hash = sets0[h0].xormask;
                        q0_size += 1;
                    }

                    sets1[h1].xormask ^= hash;
                    sets1[h1].count -= 1;
                    if sets1[h1].count == 1 {
                        q1[q1_size].index = h1 as u32;
                        q1[q1_size].hash = sets1[h1].xormask;
                        q1_size += 1;
                    }
                }
            }

            if stack_size == size {
                break;
            }

            if iterations == 10 {
                keys = prune_duplicates(keys);
                size = keys.len();
            }

            reset_sets(&mut sets0);
            reset_sets(&mut sets1);
            reset_sets(&mut sets2);

            filter.seed = splitmix64(&mut rngcounter)
        }

        let mut stack_size = size;

        while stack_size > 0 {
            stack_size -= 1;
            let ki = &stack[stack_size as usize];
            let mut val = fingerprint(ki.hash) as u8;
            if ki.index < filter.block_length {
                val ^= filter.fingerprints[(filter.geth1(ki.hash) + filter.block_length) as usize]
                    ^ filter.fingerprints
                        [(filter.geth2(ki.hash) + 2 * filter.block_length) as usize];
            } else if ki.index < 2 * filter.block_length {
                val ^= filter.fingerprints[filter.geth0(ki.hash) as usize]
                    ^ filter.fingerprints
                        [(filter.geth2(ki.hash) + 2 * filter.block_length) as usize];
            } else {
                val ^= filter.fingerprints[filter.geth0(ki.hash) as usize]
                    ^ filter.fingerprints[(filter.geth1(ki.hash) + filter.block_length) as usize];
            }
            filter.fingerprints[ki.index as usize] = val;
        }
        Ok(filter)
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

fn prune_duplicates(mut array: Vec<u64>) -> Vec<u64> {
    array.sort_unstable();
    let mut pos = 0;
    for i in 1..array.len() {
        if array[i] != array[pos] {
            pos += 1;
            array[pos] = array[i];
        }
    }
    array.truncate(pos + 1);
    array
}

fn murmur64(mut h: u64) -> u64 {
    h ^= h >> 33;
    h = h.wrapping_mul(0xff51afd7ed558ccd);
    h ^= h >> 33;
    h = h.wrapping_mul(0xc4ceb9fe1a85ec53);
    h ^= h >> 33;
    h
}

fn splitmix64(seed: &mut u64) -> u64 {
    *seed = (*seed).wrapping_add(0x9E3779B97F4A7C15);
    let mut z: u64 = seed.clone();
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}

fn mixsplit(key: u64, seed: u64) -> u64 {
    murmur64(key.wrapping_add(seed))
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

fn scan_count(qi: &mut Vec<KeyIndex>, setsi: &[XorSet]) -> u64 {
    let mut qi_size: u64 = 0;

    for (i, set) in setsi.iter().enumerate() {
        if set.count == 1 {
            if qi_size < qi.len() as u64 {
                qi[qi_size as usize].index = i as u32;
                qi[qi_size as usize].hash = set.xormask;
            } else {
                qi.push(KeyIndex {
                    index: i as u32,
                    hash: set.xormask,
                });
            }
            qi_size += 1;
        }
    }

    qi_size
}

fn reset_sets(setsi: &mut [XorSet]) {
    for set in setsi.iter_mut() {
        *set = XorSet {
            count: 0,
            xormask: 0,
        };
    }
}
