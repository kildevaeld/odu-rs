#![no_std]

mod registry;
mod r#struct;
mod traits;
mod types;

extern crate alloc;

pub use self::{
    r#struct::*,
    registry::{register, type_id, type_info, HasStaticType, TypeId},
    traits::*,
    types::*,
};

pub use once_cell::sync::Lazy;

#[cfg(feature = "derive")]
pub use odu_macros::*;

// use alloc::{
//     boxed::Box,
//     string::{String, ToString},
//     vec::Vec,
// };
// use bytes::{Bytes, BytesMut};

// pub trait HasType {
//     fn typed(&self) -> Type;
// }

// #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", serde(tag = "type"))]
// pub enum Primitive {
//     Bool,
//     U8,
//     I8,
//     U16,
//     I16,
//     U32,
//     I32,
//     U64,
//     I64,
//     F32,
//     F64,
//     String,
//     Bytes,
//     Date,
//     DateTime,
//     Time,
//     Void,
// }

// #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
// #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub struct Struct {
//     pub name: String,
//     pub fields: Vec<Field>,
// }

// impl Struct {
//     pub fn new(name: impl ToString) -> Struct {
//         Struct {
//             name: name.to_string(),
//             fields: Vec::default(),
//         }
//     }
// }

// #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
// #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub struct Field {
//     pub name: String,
//     pub kind: Type,
// }

// #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
// #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", serde(untagged))]
// pub enum Type {
//     Primitive(Primitive),
//     List(List),
//     Map(Map),
//     Struct(Struct),
//     Union(Union),
//     Optional(Optional),
// }

// #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
// #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub struct Optional {
//     pub kind: Box<Type>,
// }

// #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
// #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub struct List {
//     pub item: Box<Type>,
// }

// #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
// #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub struct Union {
//     pub items: Vec<Type>,
// }

// #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
// #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub struct Map {
//     pub key: Box<Type>,
//     pub value: Box<Type>,
// }

// impl Type {
//     pub fn as_primitive(&self) -> Option<&Primitive> {
//         match self {
//             Type::Primitive(primitive) => Some(primitive),
//             _ => None,
//         }
//     }

//     pub fn is_optional(&self) -> bool {
//         matches!(self, Type::Optional(_))
//     }
// }

// impl From<Primitive> for Type {
//     fn from(value: Primitive) -> Self {
//         Type::Primitive(value)
//     }
// }

// pub trait Typed {
//     fn typed() -> Type;
// }

// macro_rules! primitive {
//     ($($ty:ident => $name: ident),*) => {
//         $(
//             impl Typed for $ty {
//                 fn typed() -> Type {
//                     $name
//                 }
//             }

//             impl HasType for $ty {
//                 fn typed(&self) -> Type {
//                     $name
//                 }
//             }

//             pub const $name: Type = Type::Primitive(Primitive::$name);

//         )*
//     };
//     ($($ty:ident => $name: ident => $const: ident),*) => {
//         $(
//             impl Typed for $ty {
//                 fn typed() -> Type {
//                     $const
//                 }
//             }

//             impl HasType for $ty {
//                 fn typed(&self) -> Type {
//                     $const
//                 }
//             }

//             pub const $const: Type = Type::Primitive(Primitive::$name);

//         )*
//     };
// }

// primitive!(
//     bool => Bool => BOOL,
//     String => String => STRING,
//     Bytes => Bytes => BYTES,
//     BytesMut => Bytes => BYTES_MUT
// );

// primitive!(
//     u8 => U8,
//     i8 => I8,
//     u16 => U16,
//     i16 => I16,
//     u32 => U32,
//     i32 => I32,
//     u64 => U64,
//     i64 => I64,
//     f32 => F32,
//     f64 => F64
// );
