use crate::{
    r#struct::Struct,
    registry::{self, TypeId},
};
use alloc::{boxed::Box, sync::Arc, vec, vec::Vec};
use once_cell::sync::Lazy;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ComplexType {
    Struct(Arc<Struct>),
    List(List),
    Map(Map),
    Union(Union),
    Optional(Optional),
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum PrimitiveType {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum Type {
    Primitive(PrimitiveType),
    Complex(TypeId),
}

impl Type {
    pub fn as_primitive(&self) -> Option<&PrimitiveType> {
        match self {
            Type::Primitive(primitive) => Some(primitive),
            _ => None,
        }
    }

    pub fn as_complex(&self) -> Option<ComplexType> {
        match self {
            Type::Complex(ty) => Some(ty.data()),
            _ => None,
        }
    }

    pub fn is_optional(&self) -> bool {
        let Type::Complex(complex) = self else {
            return false
        };

        matches!(registry::Registry::get(complex), ComplexType::Optional(_))
    }
}

impl From<PrimitiveType> for Type {
    fn from(value: PrimitiveType) -> Self {
        Type::Primitive(value)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Optional {
    pub kind: Type,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct List {
    pub item: Type,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Union {
    pub items: Vec<Type>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Map {
    pub key: Type,
    pub value: Type,
}

pub static NUMBERS: Lazy<Type> = Lazy::new(|| {
    use crate::StaticTyped;

    let id = registry::register::<(i8, u8, i16, u16, i32, u32, i64, u64, f32, f64), _>(|id| {
        let union = Union {
            items: vec![
                i8::typed(),
                u8::typed(),
                i16::typed(),
                u16::typed(),
                i32::typed(),
                u32::typed(),
                i64::typed(),
                u64::typed(),
                f32::typed(),
                f64::typed(),
            ],
        };

        ComplexType::Union(union)
    });

    Type::Complex(id)
});
