use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const INITIAL_CAPACITY: usize = 16;

struct HashTable<K, V> {
    buckets: Vec<Option<(K, V)>>,
    size: usize,
}

impl<K: Hash + Eq + Clone, V: Clone> HashTable<K, V> {
    fn new() -> Self {
        Self {
            buckets: vec![None; INITIAL_CAPACITY],
            size: 0,
        }
    }

    fn hash(&self, key: &K) -> usize {
        let mut hasher: DefaultHasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.buckets.len()
    }

    fn insert(&mut self, key: K, value: V) {
        let mut index: usize = self.hash(&key);
        while let Some((existing_key, _)) = &self.buckets[index] {
            if existing_key == &key {
                self.buckets[index] = Some((key, value));
                return;
            }
            index = (index + 1) % self.buckets.len();
        }
        self.buckets[index] = Some((key, value));
        self.size += 1;
    }

    fn get(&self, key: &K) -> Option<&V> {
        let mut index: usize = self.hash(key);
        while let Some((existing_key, value)) = &self.buckets[index] {
            if existing_key == key {
                return Some(value);
            }
            index = (index + 1) % self.buckets.len();
        }
        None
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        let index: usize = self.hash(key);
        while let Some((existing_key, value)) = &self.buckets[index] {
            if existing_key == key {
                let entry = self.buckets[index].take();
                self.size -= 1;
                return entry.map(|(_, v)| v);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_table() {
        let mut table: HashTable<&str, &str> = HashTable::new();
        table.insert("name", "Alice");
        table.insert("age", "30");

        assert_eq!(table.get(&"name"), Some(&"Alice"));
        assert_eq!(table.get(&"age"), Some(&"30"));

        table.remove(&"age");
        assert_eq!(table.get(&"age"), None);
    }
}
