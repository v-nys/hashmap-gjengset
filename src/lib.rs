use std::{
    collections::hash_map::{DefaultHasher, RandomState},
    hash::{Hash, Hasher},
    mem,
};

const INITIAL_NBUCKETS: usize = 1; // for easier testing

// S is a a way to build a hasher
pub struct HashMap<K, V> {
    // I guess you can have multiple values in the same bucket
    // misunderstood the video as saying that wouldn't be the case, but then we'd be using Option rather than Vec
    buckets: Vec<Vec<(K, V)>>,
    items: usize,
}

impl<K, V> HashMap<K, V> {
    pub fn new() -> Self {
        HashMap {
            buckets: vec![], // start empty to avoid allocating when it is not necessary
            items: 0,
        }
    }
}

impl<K, V> HashMap<K, V>
where
    K: Hash + Eq,
{
    fn bucket(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() % (self.buckets.len() as u64)) as usize
    }

    fn resize(&mut self) {
        let target_size = match self.buckets.len() {
            0 => INITIAL_NBUCKETS,
            n => 2 * n,
        };
        let mut new_buckets = Vec::with_capacity(target_size);
        new_buckets.extend((1..=target_size).map(|_| Vec::new()));
        // drain gives every bucket in turn
        // we want to drain from every bucket
        for (key, value) in self.buckets.iter_mut().flat_map(|bucket| bucket.drain(..)) {
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);
            let bucket: usize = (hasher.finish() % (new_buckets.len() as u64)) as usize;
            new_buckets[bucket].push((key, value));
        }
        // in-place replace
        mem::replace(&mut self.buckets, new_buckets);
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let bucket = self.bucket(&key);
        self.buckets[bucket]
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v)
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.buckets.is_empty() || self.items > 3 * self.buckets.len() / 4 {
            self.resize();
        }
        // expect never to have so many buckets u64 is insufficient
        // as usize means there is a limit imposed by architecture...
        let bucket: usize = self.bucket(&key);
        let bucket = &mut self.buckets[bucket];
        for (existing_key, existing_value) in bucket.iter_mut() {
            if *existing_key == key {
                // mem::replace requires a &mut T and a T
                // if value is of type V, existing_value needs to be of type &mut V
                return Some(mem::replace(existing_value, value));
            }
        }
        // note earlier return
        bucket.push((key, value));
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert() {
        let mut map = HashMap::new();
        map.insert("foo", 42);
    }
}
