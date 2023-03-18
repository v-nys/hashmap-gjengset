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
    // I guess you can have multiple values in the same bucket
    // misunderstood the video as saying that wouldn't be the case, but then we'd be using Option rather than Vec
    buckets: Vec<Bucket<K, V>>,
}

impl<K, V> HashMap<K, V> {
    pub fn new() -> Self {
        HashMap {
            buckets: vec![], // start empty to avoid allocating when it is not necessary
        }
    }
}

impl<K, V> HashMap<K, V> where K: Hash + Eq {
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
        for (existing_key, existing_value) in bucket.items.iter_mut() {
            if *existing_key == key {
                use std::mem;
                // mem::replace requires a &mut T and a T
                // if value is of type V, existing_value needs to be of type &mut V
                return Some(mem::replace(existing_value, value));
            }
        }
        // note earlier return
        bucket.items.push((key, value));
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert() {
        let map = HashMap::new();
        map.insert("foo", 42);
    }
}