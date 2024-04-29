use std::mem;

use crate::hash_index::HashIndex;

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Counter<T: HashIndex> {
    count: T,
}
impl<T: HashIndex> Iterator for Counter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let new = self.count.increment();
        Some(mem::replace(&mut self.count, new))
    }
}