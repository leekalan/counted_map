use crate::{counter::Counter, hash_index::HashIndex};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct IndexTracker<T: HashIndex> {
    released: Vec<T>,
    counter: Counter<T>,
}
impl<T: HashIndex> IndexTracker<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn free(&mut self, index: T) {
        self.released.push(index)
    }
}
impl<T: HashIndex> Iterator for IndexTracker<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(released) = self.released.pop() {
            Some(released)
        } else {
            self.counter.next()
        }
    }
}
