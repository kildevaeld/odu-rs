use crate::{HashBuilder, List, Map, Value};
use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use bytes::Bytes;
use hashbrown::HashMap;

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
    ($($ty: ty),*) => {
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
}

into_value!(
    String => String,
    bool => Bool,
    Vec<Value> => List,
    List => List,
    Map => Map,
    HashMap<String, Value, HashBuilder> => Map
);

into_value!(i8, u8, i16, u16, i32, u32, i64, u64, f32, f64);

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
