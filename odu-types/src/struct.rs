use super::types::Type;
use alloc::vec::Vec;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Struct<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub name: &'a str,
    pub fields: Vec<Field<'a>>,
}

impl<'a> Struct<'a> {
    pub const fn new(name: &'a str, fields: Vec<Field<'a>>) -> Struct<'a> {
        Struct { name, fields }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Field<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub name: &'a str,
    pub kind: Type,
}

impl<'a> Field<'a> {
    pub const fn new(name: &'a str, kind: Type) -> Field {
        Field { name, kind }
    }
}
