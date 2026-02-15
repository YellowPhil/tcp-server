use std::hash::Hash;

use super::types::{Storage, StorageResult};
use dashmap::DashMap;

pub struct MemoryStorage<K: Eq + Hash, V> {
    data: DashMap<K, V>,
}

impl<K: Eq + Hash, V> MemoryStorage<K, V> {
    pub fn new() -> Self {
        Self {
            data: DashMap::new(),
        }
    }
}

impl<K: Eq + Hash, V: Clone> Storage<K, V> for MemoryStorage<K, V> {
    fn get(&self, key: &K) -> StorageResult<Option<V>> {
        Ok(self.data.get(key).map(|v| v.value().clone()))
    }

    fn set(&self, key: K, value: V) -> StorageResult<()> {
        self.data.insert(key, value);
        Ok(())
    }
}
