use num_bigint::BigUint;

use crate::HashIndex;

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UniqueId {
    biguint: num_bigint::BigUint,
}

impl UniqueId {
    pub fn new() -> Self {
        Self::default()
    }
}

impl HashIndex for UniqueId {
    fn increment(mut self) -> Option<Self> {
        self.biguint += BigUint::from(1u8);
        Some(self)
    }
}
