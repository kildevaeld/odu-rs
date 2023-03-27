use crate::{list::List, number::Number, object::Map, time::Time};
use alloc::string::{String, ToString};
use bytes::Bytes;
use odu_types::HasType;

macro_rules! is_method {
    ($check: ident, $ty: ident) => {
        pub fn $check(&self) -> bool {
            match self {
                Value::$ty(_) => true,
                _ => false,
            }
        }
    };
}

macro_rules! into_method {
    ($into: ident, $ty: ident, $oty: ty) => {
        pub fn $into(self) -> Result<$oty, Value> {
            match self {
                Value::$ty(v) => Ok(v),
                _ => Err(self),
            }
        }
    };
}

macro_rules! as_method {
    ($as: ident, $as_mut: ident, $ty: ident, $oty: ty) => {
        pub fn $as(&self) -> Option<&$oty> {
            match &self {
                Value::$ty(v) => Some(v),
                _ => None,
            }
        }

        pub fn $as_mut(&mut self) -> Option<&mut $oty> {
            match self {
                Value::$ty(v) => Some(v),
                _ => None,
            }
        }
    };
}

#[cfg_attr(feature = "ord", derive(Hash, PartialOrd, Ord))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Bool(bool),
    Number(Number),
    Char(char),
    String(String),
    List(List),
    Map(Map),
    Bytes(Bytes),
    Time(Time),
    None,
}

impl Value {
    pub fn is_number(&self) -> bool {
        matches!(self, Value::Number(_))
    }

    is_method!(is_string, String);
    is_method!(is_bytes, Bytes);
    is_method!(is_bool, Bool);
    is_method!(is_list, List);
    is_method!(is_map, Map);
    is_method!(is_char, Char);
    is_method!(is_time, Time);

    pub fn is_none(&self) -> bool {
        matches!(self, Value::None)
    }

    as_method!(as_number, as_number_mut, Number, Number);
    as_method!(as_string, as_string_mut, String, String);
    as_method!(as_bytes, as_bytes_mut, Bytes, Bytes);
    as_method!(as_bool, as_bool_mut, Bool, bool);
    as_method!(as_list, as_list_mut, List, List);
    as_method!(as_map, as_map_mut, Map, Map);
    as_method!(as_char, as_char_mut, Char, char);
    as_method!(as_time, as_time_mut, Time, Time);

    into_method!(into_string, String, String);
    into_method!(into_bytes, Bytes, Bytes);
    into_method!(into_bool, Bool, bool);
    into_method!(into_list, List, List);
    into_method!(into_map, Map, Map);
    into_method!(into_char, Char, char);
    into_method!(into_number, Number, Number);
    into_method!(into_time, Time, Time);

    pub fn into_option(self) -> Option<Value> {
        match self {
            Value::None => None,
            _ => Some(self),
        }
    }

    pub fn remove<S: AsRef<str>>(&mut self, field: S) -> Option<Value> {
        match self.as_map_mut() {
            Some(map) => map.remove(field),
            None => None,
        }
    }

    pub fn get<S: AsRef<str>>(&self, field: S) -> Option<&Value> {
        match self.as_map() {
            Some(map) => map.get(field),
            None => None,
        }
    }

    pub fn get_mut<S: AsRef<str>>(&mut self, field: S) -> Option<&mut Value> {
        match self.as_map_mut() {
            Some(map) => map.get_mut(field),
            None => None,
        }
    }

    pub fn insert<S: AsRef<str>, V: Into<Value>>(&mut self, field: S, value: V) -> Option<Value> {
        match self.as_map_mut() {
            Some(map) => map.insert(field.as_ref(), value.into()),
            None => None,
        }
    }

    pub fn to_string(&self) -> Option<String> {
        match self {
            Value::Bool(b) => Some(b.to_string()),
            Value::String(s) => Some(s.clone()),
            Value::Char(c) => Some(c.to_string()),
            Value::Number(n) => Some(n.to_string()),
            Value::Time(t) => Some(t.to_string()),
            _ => None,
        }
    }
}

impl AsRef<Value> for Value {
    fn as_ref(&self) -> &Value {
        self
    }
}

impl AsMut<Value> for Value {
    fn as_mut(&mut self) -> &mut Value {
        self
    }
}

impl HasType for Value {
    fn typed(&self) -> odu_types::Type {
        use odu_types::{Primitive, Type};
        match self {
            Value::Bool(_) => Primitive::Bool.into(),
            Value::Bytes(_) => Primitive::Bytes.into(),
            Value::Char(_) => Primitive::U8.into(),
            Value::List(l) => l.typed(),
            Value::Map(m) => m.typed(),
            Value::None => Type::Optional(odu_types::Optional {
                kind: Type::Primitive(Primitive::Bytes).into(),
            }),
            Value::String(_) => Primitive::String.into(),
            Value::Number(n) => n.typed(),
            Value::Time(t) => t.typed(),
        }
    }
}
