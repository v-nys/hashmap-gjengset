use std::{
    collections::hash_map::DefaultHasher,
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

/* Having some trouble with lifetimes here.
 * fact that Hashmap items are references implies that lifetimes are important here.
 *
 */
pub struct Iter<K, V> {
    elem_iterator: dyn Iterator<Item = &'a (K, V)>,
}

impl<'a, K, V> Iter<K, V> {
    fn new(map: &'a HashMap<K, V>) -> Self {
        let x = map.buckets.iter().flat_map(|b| b.iter());
    }
}

impl<K, V> Iterator for Iter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.elem_iterator.next()
    }
}

impl<'a, K, V> IntoIterator for &'a HashMap<K, V> {
    // makes sense that an item cannot outlive the hashmap
    // TODO: can I also do &'a (K,V) ? not sure...
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<&'a K, &'a V>;

    fn into_iter(self) -> Self::IntoIter {
        Iter::new(self)
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
            .find(|(k, _)| k == key) // &(...) in param position basically dereferences, because it matches on the structure
            .map(|(_, v)| v)
        /* I wonder if Jon's way of writing this is due to muscle memory,
         * because he knows a reference is produced and he matches on it immediately...
         * I prefer the style above, though. */
    }

    pub fn len(&self) -> usize {
        self.items
    }

    pub fn is_empty(&self) -> bool {
        self.items == 0
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let bucket = self.bucket(&key);
        let bucket = &mut self.buckets[bucket];
        let index_of_removed = bucket.iter().position(|(k, _)| k == key)?;
        self.items -= 1;
        Some(bucket.swap_remove(index_of_removed).1)
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
        self.items += 1;
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert() {
        let mut map = HashMap::new();
        assert_eq!(map.len(), 0);
        map.insert("foo", 42);
        assert_eq!(map.len(), 1);
        assert_eq!(map.get(&"foo"), Some(&42));
        assert_eq!(map.remove(&"foo"), Some(42));
        assert_eq!(map.len(), 0);
        assert_eq!(map.get(&"foo"), None);
    }
}
