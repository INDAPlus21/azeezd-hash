use super::*;

/// # `Map`
/// A Hash map storing a key and a value. The key is used for hashing.
#[derive(Debug)]
pub struct Map<Key, Value> {
    buckets: Vec<SlotStatus<Element<Key, Value>>>,
    size: usize,
    keys: Vec<Key>,
    updated: bool
}

impl<Key, Value> Map<Key, Value>
where
    Key: Clone + PartialEq + Hashable,
    Value: Clone + PartialEq,
{
    /// # `new`
    /// Create a new empty Map with the initial size of 31.
    pub fn new() -> Map<Key, Value> {
        Map {
            buckets: vec![SlotStatus::Empty; 31],
            size: 0,
            keys: Vec::with_capacity(31),
            updated: true
        }
    }

    /// # `insert`
    /// Takes a key and a value and tries to insert them into the Map. Returns a `Result<(), &'static str>` where `Err()` is returned if the given key already exists.
    /// Otherwise `Ok(())`
    pub fn insert(&mut self, key: Key, value: Value) -> Result<(), &'static str> {
        let hash = key.hash_code() % self.buckets.len();

        // First occurance of a removed slot. This will be saved to store the element rather than at an empty
        let mut removed_idx: Option<usize> = None;

        // Linear probing starts
        for idx in 0..self.buckets.len() {
            let vec_idx = (hash + idx) % self.buckets.len(); // index inside the bucket vector

            if let Some(slot) = self.buckets.get(vec_idx) {
                match slot {
                    SlotStatus::Empty => {
                        self.size_control()?;
                        // Empty reached, element can be placed
                        // Determine placement of element
                        let idx = if removed_idx.is_none() {
                            vec_idx
                        } else {
                            removed_idx.unwrap()
                        };
                        self.buckets[idx] = SlotStatus::Occupied(Element::new(key.clone(), value));
                        self.keys.push(key);
                        self.size += 1;
                        return Ok(());
                    }
                    SlotStatus::Occupied(item) => {
                        if item.key == key {
                            // Check key's existance in map
                            return Err("Key already exists in Map");
                        }
                    }
                    SlotStatus::Removed => {
                        if removed_idx.is_none() {
                            // Store first occurance for later use
                            removed_idx = Some(vec_idx);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// # `remove`
    /// Removes an item from the Map with the given key.
    /// Returns a `Result<Value, &'static str>` where successful removal returns the value held by the item wrapped in `Ok()`.
    pub fn remove(&mut self, key: Key) -> Result<Value, &'static str> {
        let hash = key.hash_code() % self.buckets.len();

        // Linear probing starts
        for idx in 0..self.buckets.len() {
            // Index inside the bucket vector
            let vec_idx = (hash + idx) & self.buckets.len();

            if let Some(slot) = self.buckets.get(vec_idx) {
                match slot {
                    SlotStatus::Empty => return Err("Key does not exist in Map"), // Reached empty means item was not in or near slot
                    SlotStatus::Occupied(item) => {
                        // Found an item
                        if item.key == key {
                            // Found the item
                            let value = item.value.clone();
                            self.buckets[vec_idx] = SlotStatus::Removed;
                            self.size -= 1;
                            self.updated = false;
                            return Ok(value);
                        }
                    }
                    SlotStatus::Removed => {} // Just skip over removed, nothing to do here
                }
            }
        }

        // This really should never happen but just in case
        Err("Unknown error occured")
    }

    /// # `get`
    /// Returns the value stored at the given key as `Option<Value>`. `None` is returned if the key is not available
    pub fn get(&self, key: Key) -> Option<Value> {
        let hash = key.hash_code() % self.buckets.len();

        // Linear probing starts
        for idx in 0..self.buckets.len() {
            let vec_idx = (hash + idx) % self.buckets.len();

            if let Some(slot) = self.buckets.get(vec_idx) {
                match slot {
                    SlotStatus::Empty => return None,
                    SlotStatus::Occupied(item) => {
                        if item.key == key {
                            return Some(item.value.clone());
                        }
                    }
                    SlotStatus::Removed => {}
                }
            }
        }

        None
    }
    
    /// # `set`
    /// Takes a key and a value and sets the value at that key to the given value. Return `Ok(())` if successful, else `Err()` with the error message
    pub fn set(&mut self, key: Key, value: Value) -> Result<(), &'static str> {
        let hash = key.hash_code() % self.buckets.len();

        // Linear probing starts
        for idx in 0..self.buckets.len() {
            let vec_idx = (hash + idx) % self.buckets.len();

            if let Some(slot) = self.buckets.get_mut(vec_idx) {
                match slot {
                    SlotStatus::Empty => return Err("Such key does not exist"),
                    SlotStatus::Occupied(item) => {
                        if item.key == key {
                            item.value = value;
                            return Ok(());
                        }
                    }
                    SlotStatus::Removed => {}
                }
            }
        }

        Err("Unexpected error occured")
    }

    /// # `resize`
    /// Resizes the Map into the given size as `usize`. Returns `Ok(())` on success.
    /// This is a performance-heavy process.
    pub fn resize(&mut self, size: usize) -> Result<(), &'static str> {
        if self.size > size {
            return Err("Map is bigger than given size");
        }

        let mut new_bucket: Vec<SlotStatus<Element<Key, Value>>> = vec![SlotStatus::Empty; size];

        for slot in self.buckets.iter() {
            if let SlotStatus::Occupied(item) = slot {
                let hash = item.key.hash_code() % size;

                for idx in 0..size {
                    let vec_idx = (hash + idx) % size;

                    if let Some(new_slot) = new_bucket.get(vec_idx) {
                        if let SlotStatus::Empty = new_slot {
                            new_bucket[vec_idx] = SlotStatus::Occupied(Element::new(
                                item.key.clone(),
                                item.value.clone(),
                            ));
                            break;
                        }
                    }
                }
            }
        }

        self.buckets = new_bucket;

        Ok(())
    }

    

    /// # 'size_control`
    /// Checks whether the Map requires resizing and does so if the requirements are met.
    fn size_control(&mut self) -> Result<(), &'static str> {
        // This method might be wack. I've written my reasoning in the README
        // Check if current size is bigger than ~75% max of size. Using a performance light method (I hope) read README for math :D
        let max = self.buckets.len();
        let margin = self.size > (max >> 1) + (max >> 2);

        if margin {
            println!("RESIZE");
            return self.resize(max * 2 - 1);
        }
        Ok(())
    }

    /// # `keys`
    /// Return a vector of the keys currently in the map
    pub fn keys(&mut self) -> &Vec<Key> {
        if self.updated {
            return &self.keys;
        }

        let mut new_keys: Vec<Key> = Vec::with_capacity(self.size);

        for key in self.keys.iter() {
            if let Some(_) = self.get(key.to_owned()) {
                new_keys.push(key.to_owned());
            }
        }

        self.keys = new_keys;
        self.updated = true;
        return &self.keys;
    }
}
