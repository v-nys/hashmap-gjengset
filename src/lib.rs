use std::{
    collections::hash_map::{DefaultHasher, RandomState},
    hash::{Hash, Hasher}
};

const INITIAL_NBUCKETS: usize = 1; // for easier testing

struct Bucket<K, V> {
    items: Vec<(K, V)>,
}

// S is a a way to build a hasher
pub struct HashMap<K, V> {
    buckets: Vec<Bucket<K, V>>,
}

impl<K, V> HashMap<K, V> {
    pub fn new() -> Self {
        HashMap {
            buckets: vec![], // start empty to avoid allocating when it is not necessary
        }
    }
}

impl<K, V> HashMap<K, V> where K: Hash {
    fn resize(&mut self) {
        let target_size = match self.buckets.len() {
            0 => INITIAL_NBUCKETS,
            n => 2 * n,
        };
        // TODO
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        // expect never to have so many buckets u64 is insufficient
        // as usize means there is a limit imposed by architecture...
        let bucket: usize = (hasher.finish() % (self.buckets.len() as u64)) as usize;
        let bucket = &mut self.buckets[bucket];
        bucket.items.push((key, value)); // conceptually, but would mean collisions are possible
    }
}
