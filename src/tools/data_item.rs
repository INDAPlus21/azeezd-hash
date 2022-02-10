/// # `DataType`
/// An enum that represents a cell in the `Table`. 
/// Able to represent a
/// - Unsigned Integer (`u32`): `UInteger`
/// - Signed Integer (`i32`): `Integer`
/// - Floating Point Number (`f32`): `Float`
/// - Boolean (`bool`): `Boolean`
/// - Spaceless String ('String'): `Word`
#[derive(Debug, Clone, PartialEq)]
pub enum DataItem {
    UInteger(u32),
    Integer(i32),
    Float(f32),
    Word(String),
    Boolean(bool),
}

impl DataItem {
    pub fn to_string(&self) -> String {
        match &self {
            DataItem::Boolean(e) => e.to_string(),
            DataItem::UInteger(e) => e.to_string(),
            DataItem::Integer(e) => e.to_string(),
            DataItem::Float(e) => e.to_string(),
            DataItem::Word(e) => e.to_string()
        }
    }
}
