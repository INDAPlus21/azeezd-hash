use std::{num::Wrapping, slice::from_raw_parts};

/// # `default_hash<T>`
/// Takes any struct or data type and returns its hash code as `usize`.
/// The hash is preformed on every byte in the given data.
/// Beware of hashing data holding memory addresses because that will create random hashes
pub fn default_hash<T>(to_hash: &T) -> usize {
    let mut hash: Wrapping<usize> = Wrapping(0);

    // Unsafe.. ooooo... This will create the hash based on the bytes of the key
    unsafe {
        let bytes = from_raw_parts(
            (to_hash as *const T) as *const Wrapping<u8>,
            std::mem::size_of::<T>(),
        );
        
        for byte in bytes {
            hash = ((hash << 5) + hash) + Wrapping(byte.0 as usize);
        }
    }
    return hash.0;
}

/// # `Hashable`
/// A trait used to create a hash code implementation for custom structs or data
pub trait Hashable {
    /// # `hash_code`
    /// Returns the hash code of this data
    fn hash_code(&self) -> usize;
}


impl Hashable for String {
    /// # `hash_code`
    /// Returns the hash code of this `String`. Hashing is based on the characters' bytes.
    fn hash_code(&self) -> usize {
        let mut hash: Wrapping<usize> = Wrapping(0);

        for byte in self.as_bytes() {
            hash = ((hash << 5) + hash) + Wrapping(*byte as usize);
        }

        hash.0
    }
}