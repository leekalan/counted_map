#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HashMapFull<V>(pub V);

impl<V> HashMapFull<V> {
    pub fn into(self) -> V {
        self.0
    }
}
