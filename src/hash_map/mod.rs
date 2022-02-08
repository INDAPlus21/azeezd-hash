mod hashable;
mod map;

#[derive(Clone, Copy, Debug)]
/// # `SlotStatus<T>`
/// Slot status stores 2 states which describe how a Hash Table slot is. If it holds a value then it is `Occupied(T)` and holds the value in that enum.
/// If The status is Removed, then that slot used to hold a value that now is removed. This helps the linear probing and ensures searching is correct.
/// If the status is Empty then that slot i
pub enum SlotStatus<T> {
    Occupied(T),
    Removed,
    Empty,
}

/// # `Element`
/// A struct that holds a key-value pair
#[derive(Clone, Copy, Debug)]
pub struct Element<Key, Value> {
    pub value: Value,
    pub key: Key,
}

impl<Key, Value> Element<Key, Value>
where
    Key: Clone + PartialEq + Hashable,
    Value: Clone,
{
    /// # `new`
    /// Creates a new Element using the given `Key` and `Value`
    pub fn new(key: Key, value: Value) -> Element<Key, Value> {
        Element { key, value }
    }
}

pub use {hashable::Hashable, map::Map};
