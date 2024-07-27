use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

struct SimpleHashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    capacity: usize,
}

impl<K: Eq + Hash, V> SimpleHashMap<K, V> {
    fn new(capacity: usize) -> Self {
        let buckets = (0..capacity).map(|_| Vec::new()).collect();
        SimpleHashMap { buckets, capacity }
    }

    fn insert(&mut self, key: K, value: V) {
        let index = self.get_index(&key);
        let bucket = &mut self.buckets[index];
        for &mut (ref k, ref mut v) in bucket.iter_mut() {
            if *k == key {
                *v = value;
                return;
            }
        }
        bucket.push((key, value));
    }

    fn get(&self, key: &K) -> Option<&V> {
        let index = self.get_index(key);
        let bucket = &self.buckets[index];
        for &(ref k, ref v) in bucket.iter() {
            if *k == *key {
                return Some(v);
            }
        }
        None
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        let index = self.get_index(key);
        let bucket = &mut self.buckets[index];
        let pos = bucket.iter().position(|&(ref k, _)| *k == *key)?;
        Some(bucket.swap_remove(pos).1)
    }

    fn get_index(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.capacity
    }
}

fn main() {
    let mut map = SimpleHashMap::new(10);
    map.insert("foo", 42);
    map.insert("bar", 7);

    if let Some(value) = map.get(&"foo") {
        println!("foo: {}", value);
    } else {
        println!("foo not found");
    }

    map.remove(&"foo");

    if let Some(value) = map.get(&"foo") {
        println!("foo: {}", value);
    } else {
        println!("foo not found");
    }
}
