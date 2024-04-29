use std::mem;

use crate::hash_index::HashIndex;

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Counter<T: HashIndex> {
    count: T,
}
impl<T: HashIndex> Counter<T> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn starting_at(starting_at: T) -> Self {
        Self { count: starting_at }
    }
    pub fn get_count(&self) -> &T {
        &self.count
    }
    pub unsafe fn get_count_mut(&mut self) -> &mut T {
        &mut self.count
    }
    pub fn extract(self) -> T {
        self.count
    }
}
impl<T: HashIndex> Iterator for Counter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let new = self.count.increment()?;
        Some(mem::replace(&mut self.count, new))
    }
}
