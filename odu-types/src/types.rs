use crate::{
    r#struct::Struct,
    registry::{self, TypeId},
};
use alloc::{sync::Arc, vec, vec::Vec};
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Type {
    Primitive(PrimitiveType),
    Complex(TypeId),
}

#[cfg(feature = "serde")]
impl serde::Serialize for Type {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Type::Primitive(p) => p.serialize(serializer),
            Type::Complex(c) => c.data().serialize(serializer),
        }
    }
}

#[cfg(feature = "serde")]
mod de {
    use crate::{ComplexType, PrimitiveType};

    use super::Type;
    use core::fmt;
    use serde::{
        de::{self, MapAccess},
        Deserialize,
    };

    pub struct TypeVisitor;

    impl<'de> de::Visitor<'de> for TypeVisitor {
        type Value = Type;

        fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            fmt.write_str("a type description")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            PrimitiveType::deserialize(de::value::StrDeserializer::new(v)).map(Type::Primitive)
        }

        fn visit_string<E>(self, v: alloc::string::String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.visit_str(&v)
        }

        fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            let complex_type =
                ComplexType::deserialize(de::value::MapAccessDeserializer::new(map))?;

            let type_id = crate::register_dynamic(|_| complex_type);

            Ok(Type::Complex(type_id))
        }
    }

    impl<'de> serde::de::Deserialize<'de> for Type {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            deserializer.deserialize_any(TypeVisitor)
        }
    }
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

    let id = registry::register::<(i8, u8, i16, u16, i32, u32, i64, u64, f32, f64), _>(|_id| {
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
