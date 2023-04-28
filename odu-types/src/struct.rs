use super::types::Type;
use alloc::{borrow::Cow, vec::Vec};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Struct {
    pub name: Cow<'static, str>,
    pub fields: Vec<Field>,
}

impl Struct {
    pub fn new(name: impl Into<Cow<'static, str>>) -> Struct {
        Struct {
            name: name.into(),
            fields: Vec::default(),
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Field {
    pub name: Cow<'static, str>,
    pub kind: Type,
}
