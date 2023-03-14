use std::{collections::hash_map::RandomState, hash::BuildHasher};

const INITIAL_NBUCKETS: usize = 1; // for easier testing

struct Bucket<K, V> {
    items: Vec<(K, V)>,
}

// S is a a way to build a hasher
pub struct HashMap<K, V, S = RandomState> {
    buckets: Vec<Bucket<K, V>>,
    build_hasher: S, // would've thought hasher itself...
}

impl<K, V> HashMap<K, V, RandomState> {
    pub fn new() -> Self {
        HashMap {
            buckets: vec![], // start empty to avoid allocating when it is not necessary
            build_hasher: RandomState::new(),
        }
    }
}

impl<K, V, S> HashMap<K, V, S>
where
    S: BuildHasher,
{
    fn resize(&mut self) {
        let target_size = match self.buckets.len() {
            0 => INITIAL_NBUCKETS,
            n => 2 * n,
        };
        // TODO
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        key.hash() % self.buckets.len()
    }
}
