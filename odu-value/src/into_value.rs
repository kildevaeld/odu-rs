use crate::{List, Map, MapImpl, Value};
use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use bytes::Bytes;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

macro_rules! into_value {
    ($($ty: ty => $val: ident),*) => {
        $(
            impl From<$ty> for Value {
                fn from(from: $ty) -> Value {
                    Value::$val(from.into())
                }
            }

            impl<'a> From<&'a $ty> for Value {
                fn from(from: &'a $ty) -> Value {
                    Value::$val(from.clone().into())
                }
            }
        )*
    };
    (@number $($ty: ty),*) => {
        $(
            impl From<$ty> for Value {
                fn from(from: $ty) -> Value {
                    Value::Number(from.into())
                }
            }

            impl<'a> From<&'a $ty> for Value {
                fn from(from: &'a $ty) -> Value {
                    Value::Number((*from).into())
                }
            }
        )*
    };
    (@time $($ty: ty),*) => {
        $(
            impl From<$ty> for Value {
                fn from(from: $ty) -> Value {
                    Value::Time(from.into())
                }
            }

            impl<'a> From<&'a $ty> for Value {
                fn from(from: &'a $ty) -> Value {
                    Value::Time((*from).into())
                }
            }
        )*
    };
}

into_value!(
    String => String,
    bool => Bool,
    Vec<Value> => List,
    List => List,
    Map => Map,
    MapImpl => Map
);

into_value!(@number i8, u8, i16, u16, i32, u32, i64, u64, f32, f64);

into_value!(@time NaiveDate, NaiveDateTime, NaiveTime);

impl<'a> From<&'a str> for Value {
    fn from(from: &'a str) -> Value {
        Value::String(from.to_string())
    }
}

impl From<Bytes> for Value {
    fn from(value: Bytes) -> Self {
        Value::Bytes(value)
    }
}

impl From<Vec<u8>> for Value {
    fn from(value: Vec<u8>) -> Self {
        Value::Bytes(value.into())
    }
}

impl<'a> From<&'a [u8]> for Value {
    fn from(value: &'a [u8]) -> Self {
        Value::Bytes(value.to_vec().into())
    }
}

// impl<T> From<T> for Value
// where
//     T: IntoValue,
// {
//     fn from(value: T) -> Self {
//         value.into_value()
//     }
// }
