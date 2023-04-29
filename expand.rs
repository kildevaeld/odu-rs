#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use inquire::Text;
use odu_input::{Input, Ui};
use odu_types::{FromValue, StaticTyped, Type};
use odu_value::{Map, Number, Value};
struct Terminal;
impl Ui for Terminal {
    type Error = inquire::InquireError;
    fn text(&self, name: &str, text: &odu_input::Text) -> Result<String, Self::Error> {
        Text::new(name).prompt()
    }
    fn number(
        &self,
        name: &str,
        text: &odu_input::Number,
    ) -> Result<odu_input::Number, Self::Error> {
        ::core::panicking::panic("not yet implemented")
    }
    fn form(&self, name: &str, form: &odu_input::Form) -> Result<Map, Self::Error> {
        let mut map = Map::default();
        {
            ::std::io::_print(format_args!("{0}\n", name));
        };
        for field in &form.fields {
            let value = self.input(&field.name, &field.input)?;
            map.insert(&field.name, value);
        }
        Ok(map)
    }
}
struct Test {
    name: String,
    rapper: String,
    other: Other,
}
#[automatically_derived]
impl ::core::fmt::Debug for Test {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "Test",
            "name",
            &self.name,
            "rapper",
            &self.rapper,
            "other",
            &&self.other,
        )
    }
}
impl odu_types::HasStaticType for Test {
    fn create_type_info() -> odu_types::ComplexType {
        static L: odu_types::Lazy<odu_types::ComplexType> = odu_types::Lazy::new(|| {
            odu_types::ComplexType::Struct(std::sync::Arc::new(odu_types::Struct::new(
                "Test",
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        odu_types::Field {
                            name: "name",
                            kind: String::typed(),
                        },
                        odu_types::Field {
                            name: "rapper",
                            kind: String::typed(),
                        },
                        odu_types::Field {
                            name: "other",
                            kind: Other::typed(),
                        },
                    ]),
                ),
            )))
        });
        L.clone()
    }
}
impl odu_types::Typed for Test {
    fn typed(&self) -> odu_types::Type {
        <Test as odu_types::StaticTyped>::typed()
    }
}
impl TryFrom<odu_value::Value> for Test {
    type Error = odu_value::FromValueErr<'static>;
    fn try_from(from: odu_value::Value) -> Result<Test, Self::Error> {
        let map = from.into_map().map_err(odu_value::FromValueErr::Value)?;
        Ok(Test {
            name: match map.get("name") {
                Some(value) => value.clone().try_into()?,
                None => return Err(odu_value::FromValueErr::Value(odu_value::Value::Map(map))),
            },
            rapper: match map.get("rapper") {
                Some(value) => value.clone().try_into()?,
                None => return Err(odu_value::FromValueErr::Value(odu_value::Value::Map(map))),
            },
            other: match map.get("other") {
                Some(value) => value.clone().try_into()?,
                None => return Err(odu_value::FromValueErr::Value(odu_value::Value::Map(map))),
            },
        })
    }
}
impl<'a> TryFrom<&'a odu_value::Value> for Test {
    type Error = odu_value::FromValueErr<'a>;
    fn try_from(from: &'a odu_value::Value) -> Result<Test, Self::Error> {
        let map = match from.as_map() {
            Some(map) => map,
            None => return Err(odu_value::FromValueErr::Ref(from)),
        };
        Ok(Test {
            name: match map.get("name") {
                Some(value) => value.clone().try_into()?,
                None => return Err(odu_value::FromValueErr::Ref(from)),
            },
            rapper: match map.get("rapper") {
                Some(value) => value.clone().try_into()?,
                None => return Err(odu_value::FromValueErr::Ref(from)),
            },
            other: match map.get("other") {
                Some(value) => value.clone().try_into()?,
                None => return Err(odu_value::FromValueErr::Ref(from)),
            },
        })
    }
}
struct Other {
    ost: String,
}
#[automatically_derived]
impl ::core::fmt::Debug for Other {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(f, "Other", "ost", &&self.ost)
    }
}
impl odu_types::HasStaticType for Other {
    fn create_type_info() -> odu_types::ComplexType {
        static L: odu_types::Lazy<odu_types::ComplexType> = odu_types::Lazy::new(|| {
            odu_types::ComplexType::Struct(std::sync::Arc::new(odu_types::Struct::new(
                "Other",
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([odu_types::Field {
                        name: "ost",
                        kind: String::typed(),
                    }]),
                ),
            )))
        });
        L.clone()
    }
}
impl odu_types::Typed for Other {
    fn typed(&self) -> odu_types::Type {
        <Other as odu_types::StaticTyped>::typed()
    }
}
impl TryFrom<odu_value::Value> for Other {
    type Error = odu_value::FromValueErr<'static>;
    fn try_from(from: odu_value::Value) -> Result<Other, Self::Error> {
        let map = from.into_map().map_err(odu_value::FromValueErr::Value)?;
        Ok(Other {
            ost: match map.get("ost") {
                Some(value) => value.clone().try_into()?,
                None => return Err(odu_value::FromValueErr::Value(odu_value::Value::Map(map))),
            },
        })
    }
}
impl<'a> TryFrom<&'a odu_value::Value> for Other {
    type Error = odu_value::FromValueErr<'a>;
    fn try_from(from: &'a odu_value::Value) -> Result<Other, Self::Error> {
        let map = match from.as_map() {
            Some(map) => map,
            None => return Err(odu_value::FromValueErr::Ref(from)),
        };
        Ok(Other {
            ost: match map.get("ost") {
                Some(value) => value.clone().try_into()?,
                None => return Err(odu_value::FromValueErr::Ref(from)),
            },
        })
    }
}
fn main() {
    let ty: Input = Test::typed().into();
    {
        ::std::io::_print(format_args!("test: {0:#?}\n", Number::typed().as_complex()));
    };
}
