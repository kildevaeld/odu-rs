use super::types::Type;
use alloc::vec::Vec;
use ustr::Ustr;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Struct {
    pub name: Ustr,
    pub fields: Vec<Field>,
}

impl Struct {
    pub fn new(name: &str, mut fields: Vec<Field>) -> Struct {
        let name = ustr::existing_ustr(name).unwrap_or_else(|| ustr::ustr(name));

        fields.sort_by(|a, b| a.name.cmp(&b.name));

        Struct { name, fields }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Field {
    pub name: Ustr,
    pub kind: Type,
}

impl Field {
    pub fn new(name: &str, kind: Type) -> Field {
        let name = ustr::existing_ustr(name).unwrap_or_else(|| ustr::ustr(name));

        Field { name, kind }
    }
}
