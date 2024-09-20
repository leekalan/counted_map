use std::{
    collections::HashMap,
    ops::{Index, IndexMut},
};

use crate::{hash_index::HashIndex, HashMapFull, RemovableCounter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReassignableCountedMap<K: HashIndex, V> {
    hash_map: HashMap<K, V>,
    counter: RemovableCounter<K>,
}
impl<K: HashIndex, V> ReassignableCountedMap<K, V> {
    pub fn new() -> Self {
        Self::default()
    }

    pub unsafe fn new_unsafe(hash_map: HashMap<K, V>, counter: RemovableCounter<K>) -> Self {
        Self { hash_map, counter }
    }

    pub fn with_hasher(hash_builder: std::hash::RandomState) -> Self {
        Self {
            hash_map: HashMap::with_hasher(hash_builder),
            ..Default::default()
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            hash_map: HashMap::with_capacity(capacity),
            ..Default::default()
        }
    }

    pub fn with_capacity_and_hasher(capacity: usize, hash_builder: std::hash::RandomState) -> Self {
        Self {
            hash_map: HashMap::with_capacity_and_hasher(capacity, hash_builder),
            ..Default::default()
        }
    }

    pub fn len(&self) -> usize {
        self.hash_map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.hash_map.is_empty()
    }

    pub fn capacity(&self) -> usize {
        self.hash_map.capacity()
    }

    pub fn clear(&mut self) {
        self.hash_map.clear();
        self.counter = RemovableCounter::default();
    }

    pub fn reserve(&mut self, additional: usize) {
        self.hash_map.reserve(additional);
    }

    pub fn try_reserve(
        &mut self,
        additional: usize,
    ) -> Result<(), std::collections::TryReserveError> {
        self.hash_map.try_reserve(additional)
    }

    pub fn shrink_to_fit(&mut self) {
        self.hash_map.shrink_to_fit();
    }

    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.hash_map.shrink_to(min_capacity);
    }

    pub fn drain(&mut self) -> std::collections::hash_map::Drain<'_, K, V> {
        self.counter = RemovableCounter::default();
        self.hash_map.drain()
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, K, V> {
        self.hash_map.iter()
    }

    pub fn iter_mut(&mut self) -> std::collections::hash_map::IterMut<'_, K, V> {
        self.hash_map.iter_mut()
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.hash_map.contains_key(key)
    }

    pub fn keys(&self) -> std::collections::hash_map::Keys<'_, K, V> {
        self.hash_map.keys()
    }

    pub fn values(&self) -> std::collections::hash_map::Values<'_, K, V> {
        self.hash_map.values()
    }

    pub fn values_mut(&mut self) -> std::collections::hash_map::ValuesMut<'_, K, V> {
        self.hash_map.values_mut()
    }

    pub fn get_hash_map(&self) -> &HashMap<K, V> {
        &self.hash_map
    }

    pub unsafe fn get_hash_map_mut(&mut self) -> &mut HashMap<K, V> {
        &mut self.hash_map
    }

    pub fn extract_hash_map(self) -> HashMap<K, V> {
        self.hash_map
    }

    pub fn get_counter(&self) -> &RemovableCounter<K> {
        &self.counter
    }

    pub unsafe fn get_counter_mut(&mut self) -> &mut RemovableCounter<K> {
        &mut self.counter
    }

    pub fn extract_counter(self) -> RemovableCounter<K> {
        self.counter
    }

    pub fn extract(self) -> (HashMap<K, V>, RemovableCounter<K>) {
        (self.hash_map, self.counter)
    }

    pub fn push(&mut self, value: V) -> Result<K, HashMapFull<V>> {
        let Some(index) = self.counter.next() else {
            return Err(HashMapFull(value))
        };

        if self.hash_map.insert(index.clone(), value).is_some() {
            panic!("value already exists, this should not happen!")
        }

        Ok(index)
    }

    pub fn get(&self, index: &K) -> Option<&V> {
        self.hash_map.get(index)
    }

    pub fn get_mut(&mut self, index: &K) -> Option<&mut V> {
        self.hash_map.get_mut(index)
    }

    pub fn remove(&mut self, index: K) -> Option<V> {
        let result = self.hash_map.remove(&index)?;
        self.counter
            .free(index)
            .expect("if it was removed from the hashmap, then it should be in the counter");
        Some(result)
    }

    pub fn remove_entry(&mut self, index: K) -> Option<(K, V)> {
        let result = self.hash_map.remove_entry(&index)?;
        self.counter
            .free(index)
            .expect("if it was removed from the hashmap, then it should be in the counter");
        Some(result)
    }
}
impl<K: HashIndex, V> Default for ReassignableCountedMap<K, V> {
    fn default() -> Self {
        Self {
            hash_map: HashMap::default(),
            counter: RemovableCounter::default(),
        }
    }
}
impl<K: HashIndex, V> AsRef<HashMap<K, V>> for ReassignableCountedMap<K, V> {
    fn as_ref(&self) -> &HashMap<K, V> {
        self.get_hash_map()
    }
}
impl<K: HashIndex, V> AsRef<RemovableCounter<K>> for ReassignableCountedMap<K, V> {
    fn as_ref(&self) -> &RemovableCounter<K> {
        self.get_counter()
    }
}
impl<K: HashIndex, V> Index<&K> for ReassignableCountedMap<K, V> {
    type Output = V;

    fn index(&self, index: &K) -> &Self::Output {
        self.get(index).unwrap()
    }
}
impl<K: HashIndex, V> IndexMut<&K> for ReassignableCountedMap<K, V> {
    fn index_mut(&mut self, index: &K) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}
impl<K: HashIndex, V> IntoIterator for ReassignableCountedMap<K, V> {
    type Item = (K, V);
    type IntoIter = std::collections::hash_map::IntoIter<K, V>;
    fn into_iter(self) -> Self::IntoIter {
        self.hash_map.into_iter()
    }
}
impl<'a, K: HashIndex, V> IntoIterator for &'a ReassignableCountedMap<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = std::collections::hash_map::Iter<'a, K, V>;
    fn into_iter(self) -> Self::IntoIter {
        self.hash_map.iter()
    }
}
impl<'a, K: HashIndex, V> IntoIterator for &'a mut ReassignableCountedMap<K, V> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = std::collections::hash_map::IterMut<'a, K, V>;
    fn into_iter(self) -> Self::IntoIter {
        self.hash_map.iter_mut()
    }
}
