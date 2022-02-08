/// # `DataType`
/// An enum that represents a cell in the `Table`. 
/// Able to represent a
/// - Unsigned Integer (`u32`): `UInteger`
/// - Signed Integer (`i32`): `Integer`
/// - Floating Point Number (`f32`): `Float`
/// - Boolean (`bool`): `Boolean`
#[derive(Debug, Clone, PartialEq)]
pub enum DataItem {
    UInteger(u32),
    Integer(i32),
    Float(f32),
    String(String),
    Boolean(bool),
}

impl DataItem {
    /// # `same_type_as`
    /// Returns true if the given `DataItem` has the same type (Rust's Variant) as this `DataItem`
    pub fn same_type_as(&self, other: &DataItem) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}
