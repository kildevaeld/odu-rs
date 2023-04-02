use crate::time::Time;
use alloc::string::ToString;
use core::fmt;
use core::marker::PhantomData;
use serde::{de, forward_to_deserialize_any};

use super::DeserializerError;

// pub(crate) fn unexpected(value: &Number) -> serde::de::Unexpected {
//     match *value {
//         Number::U8(n) => serde::de::Unexpected::Unsigned(n as u64),
//         Number::U16(n) => serde::de::Unexpected::Unsigned(n as u64),
//         Number::U32(n) => serde::de::Unexpected::Unsigned(n as u64),
//         Number::U64(n) => serde::de::Unexpected::Unsigned(n),
//         Number::I8(n) => serde::de::Unexpected::Signed(n as i64),
//         Number::I16(n) => serde::de::Unexpected::Signed(n as i64),
//         Number::I32(n) => serde::de::Unexpected::Signed(n as i64),
//         Number::I64(n) => serde::de::Unexpected::Signed(n),
//         Number::F32(n) => serde::de::Unexpected::Float(n as f64),
//         Number::F64(n) => serde::de::Unexpected::Float(n),
//     }
// }

pub struct TimeVisitor;

impl<'de> de::Visitor<'de> for TimeVisitor {
    type Value = Time;

    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("any numeric value")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if let Ok(datetime) = v.parse() {
            Ok(Time::DateTime(datetime))
        } else if let Ok(date) = v.parse() {
            Ok(Time::Date(date))
        } else if let Ok(time) = v.parse() {
            Ok(Time::Time(time))
        } else {
            panic!()
        }
    }

    fn visit_string<E>(self, v: alloc::string::String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_str(&v)
    }
}

pub struct TimeDeserializer<E> {
    value: Time,
    error: PhantomData<fn() -> E>,
}

impl<E> TimeDeserializer<E> {
    pub fn new(value: Time) -> Self {
        TimeDeserializer {
            value,
            error: Default::default(),
        }
    }

    // pub fn into_time(self) -> Time {
    //     self.value
    // }
}

impl<'de, E> de::Deserializer<'de> for TimeDeserializer<E>
where
    E: de::Error,
{
    type Error = E;

    fn deserialize_any<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        match self.value {
            Time::Time(time) => visitor.visit_string(time.to_string()),
            Time::Date(m) => visitor.visit_string(m.to_string()),
            Time::DateTime(m) => visitor.visit_string(m.to_string()),
        }
    }

    // fn deserialize_option<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
    //     match self.value {
    //         Value::None => self.deserialize_any(visitor),
    //         // Value::Unit => visitor.visit_unit(),
    //         _ => visitor.visit_some(self),
    //     }
    // }

    // fn deserialize_enum<V: de::Visitor<'de>>(
    //     self,
    //     _name: &'static str,
    //     _variants: &'static [&'static str],
    //     visitor: V,
    // ) -> Result<V::Value, Self::Error> {
    //     let (variant, value) = match self.value {
    //         Value::Map(value) => {
    //             let mut iter = value.into_iter();
    //             let (variant, value) = match iter.next() {
    //                 Some(v) => v,
    //                 None => {
    //                     return Err(de::Error::invalid_value(
    //                         de::Unexpected::Map,
    //                         &"map with a single key",
    //                     ));
    //                 }
    //             };
    //             // enums are encoded as maps with a single key:value pair
    //             if iter.next().is_some() {
    //                 return Err(de::Error::invalid_value(
    //                     de::Unexpected::Map,
    //                     &"map with a single key",
    //                 ));
    //             }
    //             (variant, Some(value))
    //         }
    //         Value::String(variant) => (variant, None),
    //         other => {
    //             return Err(de::Error::invalid_type(
    //                 unexpected(&other),
    //                 &"string or map",
    //             ));
    //         }
    //     };

    //     let d = EnumDeserializer {
    //         variant: variant,
    //         value: value,
    //         error: Default::default(),
    //     };
    //     visitor.visit_enum(d)
    // }

    // fn deserialize_newtype_struct<V: de::Visitor<'de>>(
    //     self,
    //     _name: &'static str,
    //     visitor: V,
    // ) -> Result<V::Value, Self::Error> {
    //     match self.value {
    //         // Value::Newtype(v) => visitor.visit_newtype_struct(TimeDeserializer::new(*v)),
    //         _ => visitor.visit_newtype_struct(self),
    //     }
    // }

    forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string unit
        seq bytes byte_buf map unit_struct option
        tuple_struct struct tuple ignored_any identifier newtype_struct enum
    }
}

impl<'de, E> de::IntoDeserializer<'de, E> for TimeDeserializer<E>
where
    E: de::Error,
{
    type Deserializer = Self;

    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}

impl<'de> de::Deserializer<'de> for Time {
    type Error = DeserializerError;

    fn deserialize_any<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        TimeDeserializer::new(self).deserialize_any(visitor)
    }

    fn deserialize_option<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        TimeDeserializer::new(self).deserialize_option(visitor)
    }

    fn deserialize_enum<V: de::Visitor<'de>>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        TimeDeserializer::new(self).deserialize_enum(name, variants, visitor)
    }

    fn deserialize_newtype_struct<V: de::Visitor<'de>>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        TimeDeserializer::new(self).deserialize_newtype_struct(name, visitor)
    }

    forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string unit
        seq bytes byte_buf map unit_struct
        tuple_struct struct tuple ignored_any identifier
    }
}
