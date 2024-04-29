use std::{collections::HashMap, ops::Deref};

use crate::{hash_index::HashIndex, index_tracker::IndexTracker};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HashMapAlloc<K: HashIndex, V> {
    hash_map: HashMap<K, V>,
    index_tracker: IndexTracker<K>,
}
impl<K: HashIndex, V> HashMapAlloc<K, V> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<K: HashIndex, V> Default for HashMapAlloc<K, V> {
    fn default() -> Self {
        Self {
            hash_map: HashMap::default(),
            index_tracker: IndexTracker::default(),
        }
    }
}
impl<K: HashIndex, V> Deref for HashMapAlloc<K, V> {
    type Target = HashMap<K, V>;

    fn deref(&self) -> &Self::Target {
        &self.hash_map
    }
}
impl<K: HashIndex, V> HashMapAlloc<K, V> {}
