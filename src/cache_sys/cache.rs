use std::collections::HashMap;
use std::hash::Hash;

///Generic cache system which can be utilized wherever heavy computation needs to be reduced
pub struct Cache<K, V> {
    entries: HashMap<K, V>,
}

impl<K: Eq + Hash, V> Cache<K, V> {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.entries.insert(key, value);
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.entries.get(key)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.entries.remove(key)
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl<K: Eq + Hash, V> Default for Cache<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
