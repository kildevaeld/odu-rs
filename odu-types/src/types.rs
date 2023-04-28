use crate::{r#struct::Struct, registry::TypeId};
use alloc::{boxed::Box, sync::Arc, vec::Vec};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum StaticType {
    Struct(Arc<Struct>),
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum Primitive {
    Bool,
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    F32,
    F64,
    String,
    Bytes,
    Date,
    DateTime,
    Time,
    Void,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum Type {
    Primitive(Primitive),
    List(List),
    Map(Map),
    Union(Union),
    Optional(Optional),
    Static(TypeId),
}

impl Type {
    pub fn as_primitive(&self) -> Option<&Primitive> {
        match self {
            Type::Primitive(primitive) => Some(primitive),
            _ => None,
        }
    }

    pub fn is_optional(&self) -> bool {
        matches!(self, Type::Optional(_))
    }
}

impl From<Primitive> for Type {
    fn from(value: Primitive) -> Self {
        Type::Primitive(value)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Optional {
    pub kind: Box<Type>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct List {
    pub item: Box<Type>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Union {
    pub items: Vec<Type>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Map {
    pub key: Box<Type>,
    pub value: Box<Type>,
}
