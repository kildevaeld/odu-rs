use super::types::Type;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Struct {
    pub name: &'static str,
    pub fields: &'static [Field],
}

impl Struct {
    pub const fn new(name: &'static str, fields: &'static [Field]) -> Struct {
        Struct { name, fields }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Field {
    pub name: &'static str,
    pub kind: Type,
}

impl Field {
    pub const fn new(name: &'static str, kind: Type) -> Field {
        Field { name, kind }
    }
}
