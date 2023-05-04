use crate::{
    registry::{self, HasStaticType},
    types::{PrimitiveType, Type},
};
use alloc::{string::String, vec::Vec};
use bytes::{Bytes, BytesMut};
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, TimeZone};

pub trait Typed {
    fn typed(&self) -> Type;
}

pub trait StaticTyped {
    fn typed() -> Type;
}

impl<T> StaticTyped for T
where
    T: HasStaticType + 'static,
{
    fn typed() -> Type {
        let tid = registry::type_id::<T>();
        Type::Complex(tid)
    }
}

macro_rules! primitive {
    ($($ty:ident => $name: ident),*) => {
        $(
            impl StaticTyped for $ty {
                fn typed() -> Type {
                    $name
                }
            }

            impl Typed for $ty {
                fn typed(&self) -> Type {
                    $name
                }
            }

            pub const $name: Type = Type::Primitive(PrimitiveType::$name);

        )*
    };
    ($($ty:ident => $name: ident => $const: ident),*) => {
        $(
            impl StaticTyped for $ty {
                fn typed() -> Type {
                    $const
                }
            }

            impl Typed for $ty {
                fn typed(&self) -> Type {
                    $const
                }
            }

            pub const $const: Type = Type::Primitive(PrimitiveType::$name);


        )*
    };
}

primitive!(
    bool => Bool => BOOL,
    String => String => STRING,
    Bytes => Bytes => BYTES,
    BytesMut => Bytes => BYTES_MUT,
    NaiveDate => Date => DATE,
    NaiveDateTime => DateTime => DATE_TIME,
    NaiveTime => Time => TIME
);

primitive!(
    u8 => U8,
    i8 => I8,
    u16 => U16,
    i16 => I16,
    u32 => U32,
    i32 => I32,
    u64 => U64,
    i64 => I64,
    f32 => F32,
    f64 => F64
);

impl<T: TimeZone> Typed for DateTime<T> {
    fn typed(&self) -> Type {
        DATE_TIME
    }
}

impl<T: TimeZone> StaticTyped for DateTime<T> {
    fn typed() -> Type {
        DATE_TIME
    }
}

impl StaticTyped for Vec<u8> {
    fn typed() -> Type {
        Type::Primitive(PrimitiveType::Bytes)
    }
}

impl Typed for Vec<u8> {
    fn typed(&self) -> Type {
        Type::Primitive(PrimitiveType::Bytes)
    }
}

impl<'a> StaticTyped for &'a [u8] {
    fn typed() -> Type {
        Type::Primitive(PrimitiveType::Bytes)
    }
}

impl<'a> Typed for &'a [u8] {
    fn typed(&self) -> Type {
        Type::Primitive(PrimitiveType::Bytes)
    }
}

impl<'a> StaticTyped for &'a str {
    fn typed() -> Type {
        Type::Primitive(PrimitiveType::String)
    }
}

impl<'a> Typed for &'a str {
    fn typed(&self) -> Type {
        Type::Primitive(PrimitiveType::String)
    }
}
