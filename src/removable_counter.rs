use std::collections::HashSet;

use crate::{counter::Counter, hash_index::HashIndex};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct RemovableCounter<T: HashIndex> {
    released_set: HashSet<T>,
    released_vec: Vec<T>,
    counter: Counter<T>,
}
impl<T: HashIndex> RemovableCounter<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub unsafe fn new_unsafe(
        released_set: HashSet<T>,
        released_vec: Vec<T>,
        counter: Counter<T>,
    ) -> Self {
        Self {
            released_set,
            released_vec,
            counter,
        }
    }

    pub fn get_count(&self) -> &T {
        self.counter.get_count()
    }

    pub fn released_set(&self) -> &HashSet<T> {
        &self.released_set
    }

    pub fn released_vec(&self) -> &Vec<T> {
        &self.released_vec
    }

    pub fn free(&mut self, index: T) -> Result<(), T> {
        if index < *self.counter.get_count() && self.released_set.insert(index) {
            self.released_vec.push(index);
            Ok(())
        } else {
            Err(index)
        }
    }

    pub unsafe fn free_unchecked(&mut self, index: T) -> Result<(), T> {
        if self.released_set.insert(index) {
            self.released_vec.push(index);
            Ok(())
        } else {
            Err(index)
        }
    }
}
impl<T: HashIndex> Iterator for RemovableCounter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(released) = self.released_vec.pop() {
            self.released_set.remove(&released);
            Some(released)
        } else {
            self.counter.next()
        }
    }
}
