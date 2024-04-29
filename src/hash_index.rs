use core::{fmt::Debug, hash::Hash};

pub trait HashIndex:
    Debug + Default + Sized + Clone + Copy + PartialEq + Eq + PartialOrd + Ord + Hash + 'static
{
    fn increment(self) -> Option<Self>;
}

impl HashIndex for usize {
    fn increment(self) -> Option<Self> {
        self.checked_add(1)
    }
}

impl HashIndex for u8 {
    fn increment(self) -> Option<Self> {
        self.checked_add(1)
    }
}

impl HashIndex for u16 {
    fn increment(self) -> Option<Self> {
        self.checked_add(1)
    }
}

impl HashIndex for u32 {
    fn increment(self) -> Option<Self> {
        self.checked_add(1)
    }
}

impl HashIndex for u64 {
    fn increment(self) -> Option<Self> {
        self.checked_add(1)
    }
}

impl HashIndex for u128 {
    fn increment(self) -> Option<Self> {
        self.checked_add(1)
    }
}
