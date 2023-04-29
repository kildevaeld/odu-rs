use alloc::{string::String, vec};
use odu_types::{
    ComplexType, List as ListType, Map as MapType, StaticTyped, Type, Union, DATE, DATE_TIME, TIME,
};
use once_cell::sync::Lazy;

use crate::{List, Map, Number, Time, Value};

static LIST_TYPE: Lazy<Type> = Lazy::new(|| {
    let list = odu_types::register::<List, _>(|_| {
        odu_types::ComplexType::List(ListType { item: *VALUE_UNION })
    });
    Type::Complex(list)
});

static MAP_TYPE: Lazy<Type> = Lazy::new(|| {
    let map = odu_types::register::<Map, _>(|_| {
        ComplexType::Map(MapType {
            key: String::typed(),
            value: *VALUE_UNION,
        })
    });
    Type::Complex(map)
});

static TIME_TYPE: Lazy<Type> = Lazy::new(|| {
    let types = vec![DATE, DATE_TIME, TIME];

    let type_id = odu_types::register::<Time, _>(|_| ComplexType::Union(Union { items: types }));
    Type::Complex(type_id)
});

static OPTIONAL_TYPE: Lazy<Type> = Lazy::new(|| {
    let type_id = odu_types::register::<Option<Value>, _>(|_| {
        ComplexType::Optional(odu_types::Optional { kind: *VALUE_UNION })
    });
    Type::Complex(type_id)
});

static VALUE_UNION: Lazy<Type> = Lazy::new(|| {
    //
    let id = odu_types::register::<Value, _>(|id| {
        let list = odu_types::register::<List, _>(|_| {
            ComplexType::List(ListType {
                item: Type::Complex(id),
            })
        });

        let map = odu_types::register::<Map, _>(|_| {
            ComplexType::Map(MapType {
                key: String::typed(),
                value: Type::Complex(id),
            })
        });

        let items = vec![
            bool::typed(),
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
            String::typed(),
            Type::Complex(list),
            Type::Complex(map),
            DATE,
            DATE_TIME,
        ];

        odu_types::ComplexType::Union(Union { items })
    });

    Type::Complex(id)
});

impl odu_types::StaticTyped for Value {
    fn typed() -> Type {
        *VALUE_UNION
    }
}

impl odu_types::Typed for Value {
    fn typed(&self) -> odu_types::Type {
        use odu_types::PrimitiveType;
        match self {
            Value::Bool(_) => PrimitiveType::Bool.into(),
            Value::Bytes(_) => PrimitiveType::Bytes.into(),
            Value::Char(_) => PrimitiveType::U8.into(),
            Value::List(l) => l.typed(),
            Value::Map(m) => m.typed(),
            Value::None => *OPTIONAL_TYPE,
            Value::String(_) => PrimitiveType::String.into(),
            Value::Number(n) => n.typed(),
            Value::Time(t) => t.typed(),
        }
    }
}

impl odu_types::StaticTyped for Number {
    fn typed() -> Type {
        *odu_types::NUMBERS
    }
}

impl odu_types::Typed for Number {
    fn typed(&self) -> odu_types::Type {
        use odu_types::PrimitiveType;
        let ty = match *self {
            Number::U8(_) => PrimitiveType::U8,
            Number::I8(_) => PrimitiveType::I8,
            Number::U16(_) => PrimitiveType::U16,
            Number::I16(_) => PrimitiveType::I16,
            Number::I32(_) => PrimitiveType::I32,
            Number::U32(_) => PrimitiveType::U32,
            Number::I64(_) => PrimitiveType::I64,
            Number::U64(_) => PrimitiveType::U64,
            Number::F32(_) => PrimitiveType::F32,
            Number::F64(_) => PrimitiveType::F64,
        };

        Type::Primitive(ty)
    }
}

impl odu_types::Typed for List {
    fn typed(&self) -> odu_types::Type {
        *LIST_TYPE
    }
}

impl StaticTyped for List {
    fn typed() -> odu_types::Type {
        *LIST_TYPE
    }
}

impl odu_types::Typed for Map {
    fn typed(&self) -> odu_types::Type {
        *MAP_TYPE
    }
}

impl StaticTyped for Map {
    fn typed() -> odu_types::Type {
        *MAP_TYPE
    }
}

impl odu_types::Typed for Time {
    fn typed(&self) -> Type {
        match self {
            Time::Date(_) => DATE,
            Time::DateTime(_) => DATE_TIME,
            Time::Time(_) => TIME,
        }
    }
}

impl StaticTyped for Time {
    fn typed() -> Type {
        *TIME_TYPE
    }
}
