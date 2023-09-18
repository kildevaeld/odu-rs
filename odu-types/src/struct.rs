use super::types::Type;
use alloc::{vec::Vec, borrow::ToOwned};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Struct {
    pub name: alloc::string::String,
    pub fields: Vec<Field>,
}

impl Struct {
    pub fn new(name: &str, mut fields: Vec<Field>) -> Struct {
        fields.sort_by(|a, b| a.name.cmp(&b.name));

        Struct { name: name.to_owned(), fields }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Field {
    pub name: alloc::string::String,
    pub kind: Type,
}

impl Field {
    pub fn new(name: &str, kind: Type) -> Field {
        Field { name: name.to_owned(), kind }
    }
}
