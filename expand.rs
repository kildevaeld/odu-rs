#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
#[macro_use]
extern crate compiler_builtins;
mod registry {
    use core::sync::atomic::{AtomicUsize, Ordering};
    use once_cell::sync::OnceCell;
    use parking_lot::RwLock;
    use crate::types::ComplexType;
    pub trait HasStaticType {
        fn create_type_info() -> ComplexType;
    }
    static REGISTRY: OnceCell<RwLock<Registry>> = OnceCell::new();
    pub struct TypeId(usize);
    #[automatically_derived]
    impl ::core::fmt::Debug for TypeId {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "TypeId", &&self.0)
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for TypeId {
        #[inline]
        fn clone(&self) -> TypeId {
            let _: ::core::clone::AssertParamIsClone<usize>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for TypeId {}
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for TypeId {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for TypeId {
        #[inline]
        fn eq(&self, other: &TypeId) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for TypeId {}
    #[automatically_derived]
    impl ::core::cmp::Eq for TypeId {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<usize>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for TypeId {
        #[inline]
        fn partial_cmp(&self, other: &TypeId) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for TypeId {
        #[inline]
        fn cmp(&self, other: &TypeId) -> ::core::cmp::Ordering {
            ::core::cmp::Ord::cmp(&self.0, &other.0)
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for TypeId {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.0, state)
        }
    }
    impl TypeId {
        fn new() -> TypeId {
            static COUNTER: AtomicUsize = AtomicUsize::new(1);
            TypeId(COUNTER.fetch_add(1, Ordering::Relaxed))
        }
        pub fn data(&self) -> ComplexType {
            Registry::get(self)
        }
    }
    pub(crate) struct Registry {
        types: ahash::HashMap<TypeId, ComplexType>,
        map: ahash::HashMap<core::any::TypeId, TypeId>,
    }
    fn registry() -> &'static RwLock<Registry> {
        REGISTRY.get_or_init(|| {
            RwLock::new(Registry {
                types: Default::default(),
                map: Default::default(),
            })
        })
    }
    pub fn type_id<T: HasStaticType + 'static>() -> TypeId {
        Registry::register::<T>()
    }
    pub fn type_info(id: TypeId) -> ComplexType {
        Registry::get(&id)
    }
    pub fn register<V: 'static, F: FnOnce(TypeId) -> ComplexType>(func: F) -> TypeId {
        Registry::register_dynamic::<V, _>(func)
    }
    impl Registry {
        pub fn register<T: HasStaticType + 'static>() -> TypeId {
            let key = core::any::TypeId::of::<T>();
            if let Some(id) = registry().read().map.get(&key) {
                return *id;
            }
            let type_id = TypeId::new();
            let type_info = T::create_type_info();
            let mut w = registry().write();
            w.types.insert(type_id, type_info);
            w.map.insert(key, type_id);
            type_id
        }
        pub fn register_dynamic<V: 'static, F: FnOnce(TypeId) -> ComplexType>(func: F) -> TypeId {
            let key = core::any::TypeId::of::<V>();
            if let Some(id) = registry().read().map.get(&key) {
                return *id;
            }
            let type_id = TypeId::new();
            let ty = func(type_id);
            let mut w = registry().write();
            w.types.insert(type_id, ty);
            w.map.insert(key, type_id);
            type_id
        }
        pub fn get(id: &TypeId) -> ComplexType {
            let map = registry().read();
            map.types[id].clone()
        }
    }
}
mod r#struct {
    use super::types::Type;
    use alloc::vec::Vec;
    pub struct Struct {
        #[serde(borrow)]
        pub name: &'static str,
        pub fields: Vec<Field>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Struct {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Struct",
                "name",
                &self.name,
                "fields",
                &&self.fields,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Struct {
        #[inline]
        fn clone(&self) -> Struct {
            Struct {
                name: ::core::clone::Clone::clone(&self.name),
                fields: ::core::clone::Clone::clone(&self.fields),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Struct {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Struct {
        #[inline]
        fn eq(&self, other: &Struct) -> bool {
            self.name == other.name && self.fields == other.fields
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for Struct {}
    #[automatically_derived]
    impl ::core::cmp::Eq for Struct {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<&'static str>;
            let _: ::core::cmp::AssertParamIsEq<Vec<Field>>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for Struct {
        #[inline]
        fn partial_cmp(&self, other: &Struct) -> ::core::option::Option<::core::cmp::Ordering> {
            match ::core::cmp::PartialOrd::partial_cmp(&self.name, &other.name) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    ::core::cmp::PartialOrd::partial_cmp(&self.fields, &other.fields)
                }
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for Struct {
        #[inline]
        fn cmp(&self, other: &Struct) -> ::core::cmp::Ordering {
            match ::core::cmp::Ord::cmp(&self.name, &other.name) {
                ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.fields, &other.fields),
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for Struct {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.name, state);
            ::core::hash::Hash::hash(&self.fields, state)
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Struct {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "Struct",
                    false as usize + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "name",
                    &self.name,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "fields",
                    &self.fields,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Deserialize<'static> for Struct {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'static>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::__private::Ok(__Field::__field0),
                            "fields" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"name" => _serde::__private::Ok(__Field::__field0),
                            b"fields" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                #[doc(hidden)]
                struct __Visitor {
                    marker: _serde::__private::PhantomData<Struct>,
                    lifetime: _serde::__private::PhantomData<&'static ()>,
                }
                impl _serde::de::Visitor<'static> for __Visitor {
                    type Value = Struct;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct Struct")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'static>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<&'static str>(
                            &mut __seq,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct Struct with 2 elements",
                                ));
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<Vec<Field>>(
                            &mut __seq,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct Struct with 2 elements",
                                ));
                            }
                        };
                        _serde::__private::Ok(Struct {
                            name: __field0,
                            fields: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'static>,
                    {
                        let mut __field0: _serde::__private::Option<&'static str> =
                            _serde::__private::None;
                        let mut __field1: _serde::__private::Option<Vec<Field>> =
                            _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "name",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<&'static str>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "fields",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Vec<Field>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("name") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("fields") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(Struct {
                            name: __field0,
                            fields: __field1,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["name", "fields"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Struct",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Struct>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl Struct {
        pub const fn new(name: &'static str, fields: Vec<Field>) -> Struct {
            Struct { name, fields }
        }
    }
    pub struct Field {
        #[serde(borrow)]
        pub name: &'static str,
        pub kind: Type,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Field {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Field",
                "name",
                &self.name,
                "kind",
                &&self.kind,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Field {
        #[inline]
        fn clone(&self) -> Field {
            Field {
                name: ::core::clone::Clone::clone(&self.name),
                kind: ::core::clone::Clone::clone(&self.kind),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Field {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Field {
        #[inline]
        fn eq(&self, other: &Field) -> bool {
            self.name == other.name && self.kind == other.kind
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for Field {}
    #[automatically_derived]
    impl ::core::cmp::Eq for Field {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<&'static str>;
            let _: ::core::cmp::AssertParamIsEq<Type>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for Field {
        #[inline]
        fn partial_cmp(&self, other: &Field) -> ::core::option::Option<::core::cmp::Ordering> {
            match ::core::cmp::PartialOrd::partial_cmp(&self.name, &other.name) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    ::core::cmp::PartialOrd::partial_cmp(&self.kind, &other.kind)
                }
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for Field {
        #[inline]
        fn cmp(&self, other: &Field) -> ::core::cmp::Ordering {
            match ::core::cmp::Ord::cmp(&self.name, &other.name) {
                ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.kind, &other.kind),
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for Field {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.name, state);
            ::core::hash::Hash::hash(&self.kind, state)
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Field {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "Field",
                    false as usize + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "name",
                    &self.name,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "kind",
                    &self.kind,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Deserialize<'static> for Field {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'static>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::__private::Ok(__Field::__field0),
                            "kind" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"name" => _serde::__private::Ok(__Field::__field0),
                            b"kind" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                #[doc(hidden)]
                struct __Visitor {
                    marker: _serde::__private::PhantomData<Field>,
                    lifetime: _serde::__private::PhantomData<&'static ()>,
                }
                impl _serde::de::Visitor<'static> for __Visitor {
                    type Value = Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct Field")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'static>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<&'static str>(
                            &mut __seq,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct Field with 2 elements",
                                ));
                            }
                        };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<Type>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Field with 2 elements",
                                        ),
                                    );
                                }
                            };
                        _serde::__private::Ok(Field {
                            name: __field0,
                            kind: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'static>,
                    {
                        let mut __field0: _serde::__private::Option<&'static str> =
                            _serde::__private::None;
                        let mut __field1: _serde::__private::Option<Type> = _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "name",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<&'static str>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "kind",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Type>(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("name") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("kind") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(Field {
                            name: __field0,
                            kind: __field1,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["name", "kind"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Field",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Field>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl Field {
        pub const fn new(name: &'static str, kind: Type) -> Field {
            Field { name, kind }
        }
    }
}
mod traits {
    use crate::{
        registry::{self, HasStaticType},
        types::{PrimitiveType, Type},
    };
    use alloc::string::String;
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
    impl StaticTyped for bool {
        fn typed() -> Type {
            BOOL
        }
    }
    impl Typed for bool {
        fn typed(&self) -> Type {
            BOOL
        }
    }
    pub const BOOL: Type = Type::Primitive(PrimitiveType::Bool);
    impl StaticTyped for String {
        fn typed() -> Type {
            STRING
        }
    }
    impl Typed for String {
        fn typed(&self) -> Type {
            STRING
        }
    }
    pub const STRING: Type = Type::Primitive(PrimitiveType::String);
    impl StaticTyped for Bytes {
        fn typed() -> Type {
            BYTES
        }
    }
    impl Typed for Bytes {
        fn typed(&self) -> Type {
            BYTES
        }
    }
    pub const BYTES: Type = Type::Primitive(PrimitiveType::Bytes);
    impl StaticTyped for BytesMut {
        fn typed() -> Type {
            BYTES_MUT
        }
    }
    impl Typed for BytesMut {
        fn typed(&self) -> Type {
            BYTES_MUT
        }
    }
    pub const BYTES_MUT: Type = Type::Primitive(PrimitiveType::Bytes);
    impl StaticTyped for NaiveDate {
        fn typed() -> Type {
            DATE
        }
    }
    impl Typed for NaiveDate {
        fn typed(&self) -> Type {
            DATE
        }
    }
    pub const DATE: Type = Type::Primitive(PrimitiveType::Date);
    impl StaticTyped for NaiveDateTime {
        fn typed() -> Type {
            DATE_TIME
        }
    }
    impl Typed for NaiveDateTime {
        fn typed(&self) -> Type {
            DATE_TIME
        }
    }
    pub const DATE_TIME: Type = Type::Primitive(PrimitiveType::DateTime);
    impl StaticTyped for NaiveTime {
        fn typed() -> Type {
            TIME
        }
    }
    impl Typed for NaiveTime {
        fn typed(&self) -> Type {
            TIME
        }
    }
    pub const TIME: Type = Type::Primitive(PrimitiveType::Time);
    impl StaticTyped for u8 {
        fn typed() -> Type {
            U8
        }
    }
    impl Typed for u8 {
        fn typed(&self) -> Type {
            U8
        }
    }
    pub const U8: Type = Type::Primitive(PrimitiveType::U8);
    impl StaticTyped for i8 {
        fn typed() -> Type {
            I8
        }
    }
    impl Typed for i8 {
        fn typed(&self) -> Type {
            I8
        }
    }
    pub const I8: Type = Type::Primitive(PrimitiveType::I8);
    impl StaticTyped for u16 {
        fn typed() -> Type {
            U16
        }
    }
    impl Typed for u16 {
        fn typed(&self) -> Type {
            U16
        }
    }
    pub const U16: Type = Type::Primitive(PrimitiveType::U16);
    impl StaticTyped for i16 {
        fn typed() -> Type {
            I16
        }
    }
    impl Typed for i16 {
        fn typed(&self) -> Type {
            I16
        }
    }
    pub const I16: Type = Type::Primitive(PrimitiveType::I16);
    impl StaticTyped for u32 {
        fn typed() -> Type {
            U32
        }
    }
    impl Typed for u32 {
        fn typed(&self) -> Type {
            U32
        }
    }
    pub const U32: Type = Type::Primitive(PrimitiveType::U32);
    impl StaticTyped for i32 {
        fn typed() -> Type {
            I32
        }
    }
    impl Typed for i32 {
        fn typed(&self) -> Type {
            I32
        }
    }
    pub const I32: Type = Type::Primitive(PrimitiveType::I32);
    impl StaticTyped for u64 {
        fn typed() -> Type {
            U64
        }
    }
    impl Typed for u64 {
        fn typed(&self) -> Type {
            U64
        }
    }
    pub const U64: Type = Type::Primitive(PrimitiveType::U64);
    impl StaticTyped for i64 {
        fn typed() -> Type {
            I64
        }
    }
    impl Typed for i64 {
        fn typed(&self) -> Type {
            I64
        }
    }
    pub const I64: Type = Type::Primitive(PrimitiveType::I64);
    impl StaticTyped for f32 {
        fn typed() -> Type {
            F32
        }
    }
    impl Typed for f32 {
        fn typed(&self) -> Type {
            F32
        }
    }
    pub const F32: Type = Type::Primitive(PrimitiveType::F32);
    impl StaticTyped for f64 {
        fn typed() -> Type {
            F64
        }
    }
    impl Typed for f64 {
        fn typed(&self) -> Type {
            F64
        }
    }
    pub const F64: Type = Type::Primitive(PrimitiveType::F64);
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
}
mod types {
    use crate::{
        r#struct::Struct,
        registry::{self, TypeId},
    };
    use alloc::{sync::Arc, vec, vec::Vec};
    use once_cell::sync::Lazy;
    #[serde(untagged)]
    pub enum ComplexType {
        Struct(Arc<Struct>),
        List(List),
        Map(Map),
        Union(Union),
        Optional(Optional),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ComplexType {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                ComplexType::Struct(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Struct", &__self_0)
                }
                ComplexType::List(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "List", &__self_0)
                }
                ComplexType::Map(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Map", &__self_0)
                }
                ComplexType::Union(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Union", &__self_0)
                }
                ComplexType::Optional(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Optional", &__self_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ComplexType {
        #[inline]
        fn clone(&self) -> ComplexType {
            match self {
                ComplexType::Struct(__self_0) => {
                    ComplexType::Struct(::core::clone::Clone::clone(__self_0))
                }
                ComplexType::List(__self_0) => {
                    ComplexType::List(::core::clone::Clone::clone(__self_0))
                }
                ComplexType::Map(__self_0) => {
                    ComplexType::Map(::core::clone::Clone::clone(__self_0))
                }
                ComplexType::Union(__self_0) => {
                    ComplexType::Union(::core::clone::Clone::clone(__self_0))
                }
                ComplexType::Optional(__self_0) => {
                    ComplexType::Optional(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ComplexType {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ComplexType {
        #[inline]
        fn eq(&self, other: &ComplexType) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
                && match (self, other) {
                    (ComplexType::Struct(__self_0), ComplexType::Struct(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (ComplexType::List(__self_0), ComplexType::List(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (ComplexType::Map(__self_0), ComplexType::Map(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (ComplexType::Union(__self_0), ComplexType::Union(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (ComplexType::Optional(__self_0), ComplexType::Optional(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for ComplexType {}
    #[automatically_derived]
    impl ::core::cmp::Eq for ComplexType {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Arc<Struct>>;
            let _: ::core::cmp::AssertParamIsEq<List>;
            let _: ::core::cmp::AssertParamIsEq<Map>;
            let _: ::core::cmp::AssertParamIsEq<Union>;
            let _: ::core::cmp::AssertParamIsEq<Optional>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for ComplexType {
        #[inline]
        fn partial_cmp(
            &self,
            other: &ComplexType,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            match (self, other) {
                (ComplexType::Struct(__self_0), ComplexType::Struct(__arg1_0)) => {
                    ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                }
                (ComplexType::List(__self_0), ComplexType::List(__arg1_0)) => {
                    ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                }
                (ComplexType::Map(__self_0), ComplexType::Map(__arg1_0)) => {
                    ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                }
                (ComplexType::Union(__self_0), ComplexType::Union(__arg1_0)) => {
                    ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                }
                (ComplexType::Optional(__self_0), ComplexType::Optional(__arg1_0)) => {
                    ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                }
                _ => ::core::cmp::PartialOrd::partial_cmp(&__self_tag, &__arg1_tag),
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for ComplexType {
        #[inline]
        fn cmp(&self, other: &ComplexType) -> ::core::cmp::Ordering {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            match ::core::cmp::Ord::cmp(&__self_tag, &__arg1_tag) {
                ::core::cmp::Ordering::Equal => match (self, other) {
                    (ComplexType::Struct(__self_0), ComplexType::Struct(__arg1_0)) => {
                        ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                    }
                    (ComplexType::List(__self_0), ComplexType::List(__arg1_0)) => {
                        ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                    }
                    (ComplexType::Map(__self_0), ComplexType::Map(__arg1_0)) => {
                        ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                    }
                    (ComplexType::Union(__self_0), ComplexType::Union(__arg1_0)) => {
                        ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                    }
                    (ComplexType::Optional(__self_0), ComplexType::Optional(__arg1_0)) => {
                        ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() },
                },
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for ComplexType {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_tag, state);
            match self {
                ComplexType::Struct(__self_0) => ::core::hash::Hash::hash(__self_0, state),
                ComplexType::List(__self_0) => ::core::hash::Hash::hash(__self_0, state),
                ComplexType::Map(__self_0) => ::core::hash::Hash::hash(__self_0, state),
                ComplexType::Union(__self_0) => ::core::hash::Hash::hash(__self_0, state),
                ComplexType::Optional(__self_0) => ::core::hash::Hash::hash(__self_0, state),
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for ComplexType {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    ComplexType::Struct(ref __field0) => {
                        _serde::Serialize::serialize(__field0, __serializer)
                    }
                    ComplexType::List(ref __field0) => {
                        _serde::Serialize::serialize(__field0, __serializer)
                    }
                    ComplexType::Map(ref __field0) => {
                        _serde::Serialize::serialize(__field0, __serializer)
                    }
                    ComplexType::Union(ref __field0) => {
                        _serde::Serialize::serialize(__field0, __serializer)
                    }
                    ComplexType::Optional(ref __field0) => {
                        _serde::Serialize::serialize(__field0, __serializer)
                    }
                }
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for ComplexType {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                let __content =
                    match <_serde::__private::de::Content as _serde::Deserialize>::deserialize(
                        __deserializer,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                if let _serde::__private::Ok(__ok) = _serde::__private::Result::map(
                    <Arc<Struct> as _serde::Deserialize>::deserialize(
                        _serde::__private::de::ContentRefDeserializer::<__D::Error>::new(
                            &__content,
                        ),
                    ),
                    ComplexType::Struct,
                ) {
                    return _serde::__private::Ok(__ok);
                }
                if let _serde::__private::Ok(__ok) = _serde::__private::Result::map(
                    <List as _serde::Deserialize>::deserialize(
                        _serde::__private::de::ContentRefDeserializer::<__D::Error>::new(
                            &__content,
                        ),
                    ),
                    ComplexType::List,
                ) {
                    return _serde::__private::Ok(__ok);
                }
                if let _serde::__private::Ok(__ok) = _serde::__private::Result::map(
                    <Map as _serde::Deserialize>::deserialize(
                        _serde::__private::de::ContentRefDeserializer::<__D::Error>::new(
                            &__content,
                        ),
                    ),
                    ComplexType::Map,
                ) {
                    return _serde::__private::Ok(__ok);
                }
                if let _serde::__private::Ok(__ok) = _serde::__private::Result::map(
                    <Union as _serde::Deserialize>::deserialize(
                        _serde::__private::de::ContentRefDeserializer::<__D::Error>::new(
                            &__content,
                        ),
                    ),
                    ComplexType::Union,
                ) {
                    return _serde::__private::Ok(__ok);
                }
                if let _serde::__private::Ok(__ok) = _serde::__private::Result::map(
                    <Optional as _serde::Deserialize>::deserialize(
                        _serde::__private::de::ContentRefDeserializer::<__D::Error>::new(
                            &__content,
                        ),
                    ),
                    ComplexType::Optional,
                ) {
                    return _serde::__private::Ok(__ok);
                }
                _serde::__private::Err(_serde::de::Error::custom(
                    "data did not match any variant of untagged enum ComplexType",
                ))
            }
        }
    };
    #[serde(tag = "type")]
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
    #[automatically_derived]
    impl ::core::fmt::Debug for PrimitiveType {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    PrimitiveType::Bool => "Bool",
                    PrimitiveType::U8 => "U8",
                    PrimitiveType::I8 => "I8",
                    PrimitiveType::U16 => "U16",
                    PrimitiveType::I16 => "I16",
                    PrimitiveType::U32 => "U32",
                    PrimitiveType::I32 => "I32",
                    PrimitiveType::U64 => "U64",
                    PrimitiveType::I64 => "I64",
                    PrimitiveType::F32 => "F32",
                    PrimitiveType::F64 => "F64",
                    PrimitiveType::String => "String",
                    PrimitiveType::Bytes => "Bytes",
                    PrimitiveType::Date => "Date",
                    PrimitiveType::DateTime => "DateTime",
                    PrimitiveType::Time => "Time",
                    PrimitiveType::Void => "Void",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for PrimitiveType {
        #[inline]
        fn clone(&self) -> PrimitiveType {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for PrimitiveType {}
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for PrimitiveType {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for PrimitiveType {
        #[inline]
        fn eq(&self, other: &PrimitiveType) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for PrimitiveType {}
    #[automatically_derived]
    impl ::core::cmp::Eq for PrimitiveType {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for PrimitiveType {
        #[inline]
        fn partial_cmp(
            &self,
            other: &PrimitiveType,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::PartialOrd::partial_cmp(&__self_tag, &__arg1_tag)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for PrimitiveType {
        #[inline]
        fn cmp(&self, other: &PrimitiveType) -> ::core::cmp::Ordering {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::Ord::cmp(&__self_tag, &__arg1_tag)
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for PrimitiveType {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_tag, state)
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for PrimitiveType {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    PrimitiveType::Bool => {
                        let mut __struct = match _serde::Serializer::serialize_struct(
                            __serializer,
                            "PrimitiveType",
                            1,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "type",
                            "Bool",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::ser::SerializeStruct::end(__struct)
                    }
                    PrimitiveType::U8 => {
                        let mut __struct = match _serde::Serializer::serialize_struct(
                            __serializer,
                            "PrimitiveType",
                            1,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "type",
                            "U8",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::ser::SerializeStruct::end(__struct)
                    }
                    PrimitiveType::I8 => {
                        let mut __struct = match _serde::Serializer::serialize_struct(
                            __serializer,
                            "PrimitiveType",
                            1,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "type",
                            "I8",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::ser::SerializeStruct::end(__struct)
                    }
                    PrimitiveType::U16 => {
                        let mut __struct = match _serde::Serializer::serialize_struct(
                            __serializer,
                            "PrimitiveType",
                            1,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "type",
                            "U16",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::ser::SerializeStruct::end(__struct)
                    }
                    PrimitiveType::I16 => {
                        let mut __struct = match _serde::Serializer::serialize_struct(
                            __serializer,
                            "PrimitiveType",
                            1,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "type",
                            "I16",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::ser::SerializeStruct::end(__struct)
                    }
                    PrimitiveType::U32 => {
                        let mut __struct = match _serde::Serializer::serialize_struct(
                            __serializer,
                            "PrimitiveType",
                            1,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "type",
                            "U32",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::ser::SerializeStruct::end(__struct)
                    }
                    PrimitiveType::I32 => {
                        let mut __struct = match _serde::Serializer::serialize_struct(
                            __serializer,
                            "PrimitiveType",
                            1,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "type",
                            "I32",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::ser::SerializeStruct::end(__struct)
                    }
                    PrimitiveType::U64 => {
                        let mut __struct = match _serde::Serializer::serialize_struct(
                            __serializer,
                            "PrimitiveType",
                            1,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "type",
                            "U64",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::ser::SerializeStruct::end(__struct)
                    }
                    PrimitiveType::I64 => {
                        let mut __struct = match _serde::Serializer::serialize_struct(
                            __serializer,
                            "PrimitiveType",
                            1,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "type",
                            "I64",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::ser::SerializeStruct::end(__struct)
                    }
                    PrimitiveType::F32 => {
                        let mut __struct = match _serde::Serializer::serialize_struct(
                            __serializer,
                            "PrimitiveType",
                            1,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "type",
                            "F32",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::ser::SerializeStruct::end(__struct)
                    }
                    PrimitiveType::F64 => {
                        let mut __struct = match _serde::Serializer::serialize_struct(
                            __serializer,
                            "PrimitiveType",
                            1,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "type",
                            "F64",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::ser::SerializeStruct::end(__struct)
                    }
                    PrimitiveType::String => {
                        let mut __struct = match _serde::Serializer::serialize_struct(
                            __serializer,
                            "PrimitiveType",
                            1,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "type",
                            "String",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::ser::SerializeStruct::end(__struct)
                    }
                    PrimitiveType::Bytes => {
                        let mut __struct = match _serde::Serializer::serialize_struct(
                            __serializer,
                            "PrimitiveType",
                            1,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "type",
                            "Bytes",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::ser::SerializeStruct::end(__struct)
                    }
                    PrimitiveType::Date => {
                        let mut __struct = match _serde::Serializer::serialize_struct(
                            __serializer,
                            "PrimitiveType",
                            1,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "type",
                            "Date",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::ser::SerializeStruct::end(__struct)
                    }
                    PrimitiveType::DateTime => {
                        let mut __struct = match _serde::Serializer::serialize_struct(
                            __serializer,
                            "PrimitiveType",
                            1,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "type",
                            "DateTime",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::ser::SerializeStruct::end(__struct)
                    }
                    PrimitiveType::Time => {
                        let mut __struct = match _serde::Serializer::serialize_struct(
                            __serializer,
                            "PrimitiveType",
                            1,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "type",
                            "Time",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::ser::SerializeStruct::end(__struct)
                    }
                    PrimitiveType::Void => {
                        let mut __struct = match _serde::Serializer::serialize_struct(
                            __serializer,
                            "PrimitiveType",
                            1,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "type",
                            "Void",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::ser::SerializeStruct::end(__struct)
                    }
                }
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for PrimitiveType {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __field6,
                    __field7,
                    __field8,
                    __field9,
                    __field10,
                    __field11,
                    __field12,
                    __field13,
                    __field14,
                    __field15,
                    __field16,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "variant identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            4u64 => _serde::__private::Ok(__Field::__field4),
                            5u64 => _serde::__private::Ok(__Field::__field5),
                            6u64 => _serde::__private::Ok(__Field::__field6),
                            7u64 => _serde::__private::Ok(__Field::__field7),
                            8u64 => _serde::__private::Ok(__Field::__field8),
                            9u64 => _serde::__private::Ok(__Field::__field9),
                            10u64 => _serde::__private::Ok(__Field::__field10),
                            11u64 => _serde::__private::Ok(__Field::__field11),
                            12u64 => _serde::__private::Ok(__Field::__field12),
                            13u64 => _serde::__private::Ok(__Field::__field13),
                            14u64 => _serde::__private::Ok(__Field::__field14),
                            15u64 => _serde::__private::Ok(__Field::__field15),
                            16u64 => _serde::__private::Ok(__Field::__field16),
                            _ => _serde::__private::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"variant index 0 <= i < 17",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "Bool" => _serde::__private::Ok(__Field::__field0),
                            "U8" => _serde::__private::Ok(__Field::__field1),
                            "I8" => _serde::__private::Ok(__Field::__field2),
                            "U16" => _serde::__private::Ok(__Field::__field3),
                            "I16" => _serde::__private::Ok(__Field::__field4),
                            "U32" => _serde::__private::Ok(__Field::__field5),
                            "I32" => _serde::__private::Ok(__Field::__field6),
                            "U64" => _serde::__private::Ok(__Field::__field7),
                            "I64" => _serde::__private::Ok(__Field::__field8),
                            "F32" => _serde::__private::Ok(__Field::__field9),
                            "F64" => _serde::__private::Ok(__Field::__field10),
                            "String" => _serde::__private::Ok(__Field::__field11),
                            "Bytes" => _serde::__private::Ok(__Field::__field12),
                            "Date" => _serde::__private::Ok(__Field::__field13),
                            "DateTime" => _serde::__private::Ok(__Field::__field14),
                            "Time" => _serde::__private::Ok(__Field::__field15),
                            "Void" => _serde::__private::Ok(__Field::__field16),
                            _ => _serde::__private::Err(_serde::de::Error::unknown_variant(
                                __value, VARIANTS,
                            )),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"Bool" => _serde::__private::Ok(__Field::__field0),
                            b"U8" => _serde::__private::Ok(__Field::__field1),
                            b"I8" => _serde::__private::Ok(__Field::__field2),
                            b"U16" => _serde::__private::Ok(__Field::__field3),
                            b"I16" => _serde::__private::Ok(__Field::__field4),
                            b"U32" => _serde::__private::Ok(__Field::__field5),
                            b"I32" => _serde::__private::Ok(__Field::__field6),
                            b"U64" => _serde::__private::Ok(__Field::__field7),
                            b"I64" => _serde::__private::Ok(__Field::__field8),
                            b"F32" => _serde::__private::Ok(__Field::__field9),
                            b"F64" => _serde::__private::Ok(__Field::__field10),
                            b"String" => _serde::__private::Ok(__Field::__field11),
                            b"Bytes" => _serde::__private::Ok(__Field::__field12),
                            b"Date" => _serde::__private::Ok(__Field::__field13),
                            b"DateTime" => _serde::__private::Ok(__Field::__field14),
                            b"Time" => _serde::__private::Ok(__Field::__field15),
                            b"Void" => _serde::__private::Ok(__Field::__field16),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(_serde::de::Error::unknown_variant(
                                    __value, VARIANTS,
                                ))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                #[doc(hidden)]
                const VARIANTS: &'static [&'static str] = &[
                    "Bool", "U8", "I8", "U16", "I16", "U32", "I32", "U64", "I64", "F32", "F64",
                    "String", "Bytes", "Date", "DateTime", "Time", "Void",
                ];
                let __tagged = match _serde::Deserializer::deserialize_any(
                    __deserializer,
                    _serde::__private::de::TaggedContentVisitor::<__Field>::new(
                        "type",
                        "internally tagged enum PrimitiveType",
                    ),
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match __tagged.tag {
                    __Field::__field0 => {
                        match _serde::Deserializer::deserialize_any(
                            _serde::__private::de::ContentDeserializer::<__D::Error>::new(
                                __tagged.content,
                            ),
                            _serde::__private::de::InternallyTaggedUnitVisitor::new(
                                "PrimitiveType",
                                "Bool",
                            ),
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::__private::Ok(PrimitiveType::Bool)
                    }
                    __Field::__field1 => {
                        match _serde::Deserializer::deserialize_any(
                            _serde::__private::de::ContentDeserializer::<__D::Error>::new(
                                __tagged.content,
                            ),
                            _serde::__private::de::InternallyTaggedUnitVisitor::new(
                                "PrimitiveType",
                                "U8",
                            ),
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::__private::Ok(PrimitiveType::U8)
                    }
                    __Field::__field2 => {
                        match _serde::Deserializer::deserialize_any(
                            _serde::__private::de::ContentDeserializer::<__D::Error>::new(
                                __tagged.content,
                            ),
                            _serde::__private::de::InternallyTaggedUnitVisitor::new(
                                "PrimitiveType",
                                "I8",
                            ),
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::__private::Ok(PrimitiveType::I8)
                    }
                    __Field::__field3 => {
                        match _serde::Deserializer::deserialize_any(
                            _serde::__private::de::ContentDeserializer::<__D::Error>::new(
                                __tagged.content,
                            ),
                            _serde::__private::de::InternallyTaggedUnitVisitor::new(
                                "PrimitiveType",
                                "U16",
                            ),
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::__private::Ok(PrimitiveType::U16)
                    }
                    __Field::__field4 => {
                        match _serde::Deserializer::deserialize_any(
                            _serde::__private::de::ContentDeserializer::<__D::Error>::new(
                                __tagged.content,
                            ),
                            _serde::__private::de::InternallyTaggedUnitVisitor::new(
                                "PrimitiveType",
                                "I16",
                            ),
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::__private::Ok(PrimitiveType::I16)
                    }
                    __Field::__field5 => {
                        match _serde::Deserializer::deserialize_any(
                            _serde::__private::de::ContentDeserializer::<__D::Error>::new(
                                __tagged.content,
                            ),
                            _serde::__private::de::InternallyTaggedUnitVisitor::new(
                                "PrimitiveType",
                                "U32",
                            ),
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::__private::Ok(PrimitiveType::U32)
                    }
                    __Field::__field6 => {
                        match _serde::Deserializer::deserialize_any(
                            _serde::__private::de::ContentDeserializer::<__D::Error>::new(
                                __tagged.content,
                            ),
                            _serde::__private::de::InternallyTaggedUnitVisitor::new(
                                "PrimitiveType",
                                "I32",
                            ),
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::__private::Ok(PrimitiveType::I32)
                    }
                    __Field::__field7 => {
                        match _serde::Deserializer::deserialize_any(
                            _serde::__private::de::ContentDeserializer::<__D::Error>::new(
                                __tagged.content,
                            ),
                            _serde::__private::de::InternallyTaggedUnitVisitor::new(
                                "PrimitiveType",
                                "U64",
                            ),
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::__private::Ok(PrimitiveType::U64)
                    }
                    __Field::__field8 => {
                        match _serde::Deserializer::deserialize_any(
                            _serde::__private::de::ContentDeserializer::<__D::Error>::new(
                                __tagged.content,
                            ),
                            _serde::__private::de::InternallyTaggedUnitVisitor::new(
                                "PrimitiveType",
                                "I64",
                            ),
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::__private::Ok(PrimitiveType::I64)
                    }
                    __Field::__field9 => {
                        match _serde::Deserializer::deserialize_any(
                            _serde::__private::de::ContentDeserializer::<__D::Error>::new(
                                __tagged.content,
                            ),
                            _serde::__private::de::InternallyTaggedUnitVisitor::new(
                                "PrimitiveType",
                                "F32",
                            ),
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::__private::Ok(PrimitiveType::F32)
                    }
                    __Field::__field10 => {
                        match _serde::Deserializer::deserialize_any(
                            _serde::__private::de::ContentDeserializer::<__D::Error>::new(
                                __tagged.content,
                            ),
                            _serde::__private::de::InternallyTaggedUnitVisitor::new(
                                "PrimitiveType",
                                "F64",
                            ),
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::__private::Ok(PrimitiveType::F64)
                    }
                    __Field::__field11 => {
                        match _serde::Deserializer::deserialize_any(
                            _serde::__private::de::ContentDeserializer::<__D::Error>::new(
                                __tagged.content,
                            ),
                            _serde::__private::de::InternallyTaggedUnitVisitor::new(
                                "PrimitiveType",
                                "String",
                            ),
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::__private::Ok(PrimitiveType::String)
                    }
                    __Field::__field12 => {
                        match _serde::Deserializer::deserialize_any(
                            _serde::__private::de::ContentDeserializer::<__D::Error>::new(
                                __tagged.content,
                            ),
                            _serde::__private::de::InternallyTaggedUnitVisitor::new(
                                "PrimitiveType",
                                "Bytes",
                            ),
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::__private::Ok(PrimitiveType::Bytes)
                    }
                    __Field::__field13 => {
                        match _serde::Deserializer::deserialize_any(
                            _serde::__private::de::ContentDeserializer::<__D::Error>::new(
                                __tagged.content,
                            ),
                            _serde::__private::de::InternallyTaggedUnitVisitor::new(
                                "PrimitiveType",
                                "Date",
                            ),
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::__private::Ok(PrimitiveType::Date)
                    }
                    __Field::__field14 => {
                        match _serde::Deserializer::deserialize_any(
                            _serde::__private::de::ContentDeserializer::<__D::Error>::new(
                                __tagged.content,
                            ),
                            _serde::__private::de::InternallyTaggedUnitVisitor::new(
                                "PrimitiveType",
                                "DateTime",
                            ),
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::__private::Ok(PrimitiveType::DateTime)
                    }
                    __Field::__field15 => {
                        match _serde::Deserializer::deserialize_any(
                            _serde::__private::de::ContentDeserializer::<__D::Error>::new(
                                __tagged.content,
                            ),
                            _serde::__private::de::InternallyTaggedUnitVisitor::new(
                                "PrimitiveType",
                                "Time",
                            ),
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::__private::Ok(PrimitiveType::Time)
                    }
                    __Field::__field16 => {
                        match _serde::Deserializer::deserialize_any(
                            _serde::__private::de::ContentDeserializer::<__D::Error>::new(
                                __tagged.content,
                            ),
                            _serde::__private::de::InternallyTaggedUnitVisitor::new(
                                "PrimitiveType",
                                "Void",
                            ),
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::__private::Ok(PrimitiveType::Void)
                    }
                }
            }
        }
    };
    pub enum Type {
        Primitive(PrimitiveType),
        Complex(TypeId),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Type {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Type::Primitive(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Primitive", &__self_0)
                }
                Type::Complex(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Complex", &__self_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Type {
        #[inline]
        fn clone(&self) -> Type {
            let _: ::core::clone::AssertParamIsClone<PrimitiveType>;
            let _: ::core::clone::AssertParamIsClone<TypeId>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Type {}
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Type {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Type {
        #[inline]
        fn eq(&self, other: &Type) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
                && match (self, other) {
                    (Type::Primitive(__self_0), Type::Primitive(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (Type::Complex(__self_0), Type::Complex(__arg1_0)) => *__self_0 == *__arg1_0,
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for Type {}
    #[automatically_derived]
    impl ::core::cmp::Eq for Type {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<PrimitiveType>;
            let _: ::core::cmp::AssertParamIsEq<TypeId>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for Type {
        #[inline]
        fn partial_cmp(&self, other: &Type) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            match (self, other) {
                (Type::Primitive(__self_0), Type::Primitive(__arg1_0)) => {
                    ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                }
                (Type::Complex(__self_0), Type::Complex(__arg1_0)) => {
                    ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                }
                _ => ::core::cmp::PartialOrd::partial_cmp(&__self_tag, &__arg1_tag),
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for Type {
        #[inline]
        fn cmp(&self, other: &Type) -> ::core::cmp::Ordering {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            match ::core::cmp::Ord::cmp(&__self_tag, &__arg1_tag) {
                ::core::cmp::Ordering::Equal => match (self, other) {
                    (Type::Primitive(__self_0), Type::Primitive(__arg1_0)) => {
                        ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                    }
                    (Type::Complex(__self_0), Type::Complex(__arg1_0)) => {
                        ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() },
                },
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for Type {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_tag, state);
            match self {
                Type::Primitive(__self_0) => ::core::hash::Hash::hash(__self_0, state),
                Type::Complex(__self_0) => ::core::hash::Hash::hash(__self_0, state),
            }
        }
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
    impl<'de> serde::Deserialize<'de> for Type {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            ::core::panicking::panic("not yet implemented")
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
            let Type :: Complex (complex) = self else { return false } ;
            match registry::Registry::get(complex) {
                ComplexType::Optional(_) => true,
                _ => false,
            }
        }
    }
    impl From<PrimitiveType> for Type {
        fn from(value: PrimitiveType) -> Self {
            Type::Primitive(value)
        }
    }
    pub struct Optional {
        pub kind: Type,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Optional {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(f, "Optional", "kind", &&self.kind)
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Optional {
        #[inline]
        fn clone(&self) -> Optional {
            Optional {
                kind: ::core::clone::Clone::clone(&self.kind),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Optional {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Optional {
        #[inline]
        fn eq(&self, other: &Optional) -> bool {
            self.kind == other.kind
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for Optional {}
    #[automatically_derived]
    impl ::core::cmp::Eq for Optional {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Type>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for Optional {
        #[inline]
        fn partial_cmp(&self, other: &Optional) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::cmp::PartialOrd::partial_cmp(&self.kind, &other.kind)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for Optional {
        #[inline]
        fn cmp(&self, other: &Optional) -> ::core::cmp::Ordering {
            ::core::cmp::Ord::cmp(&self.kind, &other.kind)
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for Optional {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.kind, state)
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Optional {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "Optional",
                    false as usize + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "kind",
                    &self.kind,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Optional {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "kind" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"kind" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Optional>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Optional;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct Optional")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<Type>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Optional with 1 element",
                                        ),
                                    );
                                }
                            };
                        _serde::__private::Ok(Optional { kind: __field0 })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<Type> = _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "kind",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Type>(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("kind") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(Optional { kind: __field0 })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["kind"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Optional",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Optional>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    pub struct List {
        pub item: Type,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for List {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(f, "List", "item", &&self.item)
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for List {
        #[inline]
        fn clone(&self) -> List {
            List {
                item: ::core::clone::Clone::clone(&self.item),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for List {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for List {
        #[inline]
        fn eq(&self, other: &List) -> bool {
            self.item == other.item
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for List {}
    #[automatically_derived]
    impl ::core::cmp::Eq for List {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Type>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for List {
        #[inline]
        fn partial_cmp(&self, other: &List) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::cmp::PartialOrd::partial_cmp(&self.item, &other.item)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for List {
        #[inline]
        fn cmp(&self, other: &List) -> ::core::cmp::Ordering {
            ::core::cmp::Ord::cmp(&self.item, &other.item)
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for List {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.item, state)
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for List {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "List",
                    false as usize + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "item",
                    &self.item,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for List {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "item" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"item" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<List>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = List;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct List")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<Type>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct List with 1 element",
                                        ),
                                    );
                                }
                            };
                        _serde::__private::Ok(List { item: __field0 })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<Type> = _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "item",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Type>(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("item") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(List { item: __field0 })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["item"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "List",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<List>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    pub struct Union {
        pub items: Vec<Type>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Union {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(f, "Union", "items", &&self.items)
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Union {
        #[inline]
        fn clone(&self) -> Union {
            Union {
                items: ::core::clone::Clone::clone(&self.items),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Union {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Union {
        #[inline]
        fn eq(&self, other: &Union) -> bool {
            self.items == other.items
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for Union {}
    #[automatically_derived]
    impl ::core::cmp::Eq for Union {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Vec<Type>>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for Union {
        #[inline]
        fn partial_cmp(&self, other: &Union) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::cmp::PartialOrd::partial_cmp(&self.items, &other.items)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for Union {
        #[inline]
        fn cmp(&self, other: &Union) -> ::core::cmp::Ordering {
            ::core::cmp::Ord::cmp(&self.items, &other.items)
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for Union {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.items, state)
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Union {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "Union",
                    false as usize + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "items",
                    &self.items,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Union {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "items" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"items" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Union>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Union;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct Union")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<Vec<Type>>(
                            &mut __seq,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct Union with 1 element",
                                ));
                            }
                        };
                        _serde::__private::Ok(Union { items: __field0 })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<Vec<Type>> =
                            _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "items",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Vec<Type>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("items") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(Union { items: __field0 })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["items"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Union",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Union>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    pub struct Map {
        pub key: Type,
        pub value: Type,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Map {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Map",
                "key",
                &self.key,
                "value",
                &&self.value,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Map {
        #[inline]
        fn clone(&self) -> Map {
            Map {
                key: ::core::clone::Clone::clone(&self.key),
                value: ::core::clone::Clone::clone(&self.value),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Map {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Map {
        #[inline]
        fn eq(&self, other: &Map) -> bool {
            self.key == other.key && self.value == other.value
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for Map {}
    #[automatically_derived]
    impl ::core::cmp::Eq for Map {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Type>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for Map {
        #[inline]
        fn partial_cmp(&self, other: &Map) -> ::core::option::Option<::core::cmp::Ordering> {
            match ::core::cmp::PartialOrd::partial_cmp(&self.key, &other.key) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    ::core::cmp::PartialOrd::partial_cmp(&self.value, &other.value)
                }
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for Map {
        #[inline]
        fn cmp(&self, other: &Map) -> ::core::cmp::Ordering {
            match ::core::cmp::Ord::cmp(&self.key, &other.key) {
                ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.value, &other.value),
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for Map {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.key, state);
            ::core::hash::Hash::hash(&self.value, state)
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Map {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "Map",
                    false as usize + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "key",
                    &self.key,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "value",
                    &self.value,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Map {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "key" => _serde::__private::Ok(__Field::__field0),
                            "value" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"key" => _serde::__private::Ok(__Field::__field0),
                            b"value" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Map>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Map;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct Map")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<Type>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Map with 2 elements",
                                        ),
                                    );
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<Type>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Map with 2 elements",
                                        ),
                                    );
                                }
                            };
                        _serde::__private::Ok(Map {
                            key: __field0,
                            value: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<Type> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<Type> = _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "key",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Type>(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "value",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Type>(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("key") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("value") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(Map {
                            key: __field0,
                            value: __field1,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["key", "value"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Map",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Map>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    pub static NUMBERS: Lazy<Type> = Lazy::new(|| {
        use crate::StaticTyped;
        let id = registry::register::<(i8, u8, i16, u16, i32, u32, i64, u64, f32, f64), _>(|_id| {
            let union = Union {
                items: <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
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
                    ]),
                ),
            };
            ComplexType::Union(union)
        });
        Type::Complex(id)
    });
}
extern crate alloc;
pub use self::{
    r#struct::*,
    registry::{register, type_id, type_info, HasStaticType, TypeId},
    traits::*,
    types::*,
};
pub use once_cell::sync::Lazy;
