use crate::{List, Map, Number, Time, Value};
use alloc::string::String;
use bytes::Bytes;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

#[derive(Debug, Clone)]
pub enum FromValueErr<'a> {
    Value(Value),
    Ref(&'a Value),
}

impl<'a> FromValueErr<'a> {
    pub fn value(&self) -> &Value {
        match self {
            FromValueErr::Ref(ret) => ret,
            FromValueErr::Value(v) => v,
        }
    }

    pub fn to_owned(self) -> FromValueErr<'static> {
        match self {
            FromValueErr::Ref(v) => FromValueErr::Value(v.clone()),
            FromValueErr::Value(v) => FromValueErr::Value(v),
        }
    }
}

impl<'a> core::fmt::Display for FromValueErr<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "from error: {:?}", self.value())
    }
}

#[cfg(feature = "std")]
impl<'a> std::error::Error for FromValueErr<'a> {}

macro_rules! from_impl {
    ($type: ty, $method: ident, $as: ident, $as_mut: ident) => {
        impl TryFrom<Value> for $type {
            type Error = FromValueErr<'static>;
            fn try_from(from: Value) -> Result<Self, Self::Error> {
                match from.$method() {
                    Ok(s) => Ok(s),
                    Err(err) => Err(FromValueErr::Value(err)),
                }
            }
        }

        impl<'a> TryFrom<&'a Value> for &'a $type {
            type Error = FromValueErr<'a>;
            fn try_from(from: &'a Value) -> Result<Self, Self::Error> {
                match from.$as() {
                    Some(s) => Ok(s),
                    None => Err(FromValueErr::Ref(from)),
                }
            }
        }

        impl<'a> TryFrom<&'a Value> for $type {
            type Error = FromValueErr<'a>;
            fn try_from(from: &'a Value) -> Result<Self, Self::Error> {
                match from.$as() {
                    Some(s) => Ok(s.clone()),
                    None => Err(FromValueErr::Ref(from)),
                }
            }
        }
    };
    (@number $($type: ty => $variant: ident),*) => {
        $(
            impl TryFrom<Value> for $type {
                type Error = FromValueErr<'static>;
                fn try_from(from: Value) -> Result<Self, Self::Error> {
                    match from.into_number() {
                        Ok(n) => match n {
                            Number::$variant(n) => Ok(n),
                            n => Err(FromValueErr::Value(Value::Number(n)))
                        },
                        Err(err) => Err(FromValueErr::Value(err)),
                    }
                }
            }

            impl<'a> TryFrom<&'a Value> for $type {
                type Error = FromValueErr<'a>;
                fn try_from(from: &'a Value) -> Result<Self, Self::Error> {
                    match from.as_number() {
                        Some(Number::$variant(n)) => Ok(*n),
                        _ => Err(FromValueErr::Ref(from)),
                    }
                }
            }
        )*
    };
    (@time $($type: ty => $method: ident),*) => {
        $(
            impl TryFrom<Value> for $type {
                type Error = FromValueErr<'static>;
                fn try_from(from: Value) -> Result<Self, Self::Error> {
                    match from.into_time() {
                        Ok(n) => n.$method().ok_or_else(|| FromValueErr::Value(Value::Time(n))),
                        Err(err) => Err(FromValueErr::Value(err)),
                    }
                }
            }

            impl<'a> TryFrom<&'a Value> for $type {
                type Error = FromValueErr<'a>;
                fn try_from(from: &'a Value) -> Result<Self, Self::Error> {
                    match from.as_time() {
                        Some(n) => n.$method().ok_or_else(|| FromValueErr::Value(Value::Time(*n))),
                        None => Err(FromValueErr::Ref(from)),
                    }
                }
            }
        )*
    };
}

from_impl!(String, into_string, as_string, as_string_mut);
from_impl!(Bytes, into_bytes, as_bytes, as_bytes_mut);
from_impl!(bool, into_bool, as_bool, as_bool_mut);
from_impl!(Number, into_number, as_number, as_number_mut);
from_impl!(Map, into_map, as_map, as_map_mut);
from_impl!(List, into_list, as_list, as_list_mut);
from_impl!(Time, into_time, as_time, as_time_mut);

from_impl!(
    @number
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

from_impl!(
    @time
    NaiveDate => as_date,
    NaiveDateTime => as_datetime,
    NaiveTime => as_time
);

impl<'a> TryFrom<&'a Value> for &'a str {
    type Error = FromValueErr<'a>;
    fn try_from(from: &'a Value) -> Result<Self, Self::Error> {
        match from.as_string() {
            Some(s) => Ok(s),
            None => Err(FromValueErr::Ref(from)),
        }
    }
}

impl<'a> TryFrom<&'a Value> for &'a [u8] {
    type Error = FromValueErr<'a>;
    fn try_from(from: &'a Value) -> Result<Self, Self::Error> {
        match from.as_bytes() {
            Some(s) => Ok(s),
            None => Err(FromValueErr::Ref(from)),
        }
    }
}
