#![no_std]

extern crate alloc;

use alloc::{
    boxed::Box,
    string::{String, ToString},
    vec::Vec,
};
use odu_types::{PrimitiveType, Struct, Type};
use odu_validate::{StringValidator, ToValidator, Validation, ValidatorBuilderExt};
use odu_value::{Map, Value};

pub enum Input {
    Text(Text),
    LongText,
    Number(Number),
    Bool,
    List(List),
    Form(Form),
}

pub struct List {
    input: Box<Input>,
}

pub struct Form {
    pub fields: Vec<Field>,
}

pub struct Field {
    pub name: String,
    pub input: Input,
}

pub struct Text {
    pub default: Option<String>,
    pub required: bool,
}

impl ToValidator for Text {
    fn validator(&self) -> odu_validate::Validator {
        StringValidator::default().into()
    }
}

pub struct Number {
    pub default: Option<odu_value::Number>,
    pub float: bool,
}

impl From<PrimitiveType> for Input {
    fn from(value: PrimitiveType) -> Self {
        match value {
            PrimitiveType::String => Input::Text(Text {
                default: None,
                required: false,
            }),
            _ => todo!(),
        }
    }
}

// impl From<Struct> for Input {
//     fn from(value: Struct) -> Self {
//         Input::Form(Form {
//             fields: value
//                 .fields
//                 .into_iter()
//                 .map(|m| Field {
//                     name: m.name.to_string(),
//                     input: m.kind.into(),
//                 })
//                 .collect(),
//         })
//     }
// }

// impl From<Type> for Input {
//     fn from(value: Type) -> Self {
//         match value {
//             Type::Primitive(p) => p.into(),
//             Type::Complex(s) => s.into(),
//             _ => todo!(),
//         }
//     }
// }

pub trait Ui {
    type Error;
    fn text(&self, name: &str, text: &Text) -> Result<String, Self::Error>;

    fn number(&self, name: &str, text: &Number) -> Result<Number, Self::Error>;

    fn form(&self, name: &str, form: &Form) -> Result<Map, Self::Error>;

    fn input(&self, name: &str, input: &Input) -> Result<Value, Self::Error> {
        match input {
            Input::Text(text) => self.text(name, text).map(Into::into),
            Input::Form(form) => self.form(name, form).map(Into::into),
            //Input::Number(number) => self.number(name, number).map(Into::into),
            _ => todo!(),
        }
    }
}
