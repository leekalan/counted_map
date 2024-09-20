pub mod counted_map;
pub mod counter;
pub mod hash_index;
pub mod hash_map_full;
pub mod reassignable_counted_map;
pub mod removable_counter;
pub mod unique_id;

pub use hash_index::HashIndex;
pub use unique_id::UniqueId;

pub use counter::Counter;
pub use removable_counter::RemovableCounter;

pub use hash_map_full::HashMapFull;

pub use counted_map::CountedMap;
pub use reassignable_counted_map::ReassignableCountedMap;

#[cfg(test)]
mod tests {
    use crate::ReassignableCountedMap;

    #[test]
    fn add_and_remove() {
        let mut map: ReassignableCountedMap<usize, i32> = ReassignableCountedMap::new();

        let a = map.push(10).unwrap();
        let b = map.push(20).unwrap();
        map.remove(b);
        let c = map.push(30).unwrap();
        map.remove(a);
        map.remove(c);
    }
}
