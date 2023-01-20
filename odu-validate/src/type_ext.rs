use alloc::{boxed::Box, vec::Vec};
use odu_types::{List, Primitive, Struct, Type, Union};

use crate::{
    validations,
    validators::{
        any_of, BoolValidator, ListValidator, NumberValidator, ObjectValidator, StringValidator,
        Validator, ValidatorBuilderCommon, ValidatorBuilderExt,
    },
    Validation,
};

pub trait HasValidation {
    type Validation: Validation;

    fn validator() -> Self::Validation;
}

impl HasValidation for alloc::string::String {
    type Validation = StringValidator;
    fn validator() -> Self::Validation {
        StringValidator::default()
    }
}

mod sealed {
    pub trait Sealed {}

    impl Sealed for odu_types::Type {}
    impl Sealed for odu_types::Primitive {}
    impl Sealed for odu_types::Struct {}
}

pub trait ToValidator {
    fn validator(&self) -> Validator;
}

impl ToValidator for Primitive {
    fn validator(&self) -> Validator {
        match self {
            Primitive::Bool => BoolValidator::default().into(),
            Primitive::String => StringValidator::default().into(),
            Primitive::I8
            | Primitive::U8
            | Primitive::I16
            | Primitive::U16
            | Primitive::I64
            | Primitive::U64 => NumberValidator::default().into(),
            _ => todo!(),
        }
    }
}

impl ToValidator for Struct {
    fn validator(&self) -> Validator {
        let mut builder = ObjectValidator::default();

        for field in &self.fields {
            builder.add_field(&field.name, field.kind.validator());
        }

        builder.into()
    }
}

impl ToValidator for List {
    fn validator(&self) -> Validator {
        ListValidator::default()
            .required()
            .and(validations::item(self.item.validator()))
            .into()
    }
}

impl ToValidator for Union {
    fn validator(&self) -> Validator {
        let validations = self.items.iter().map(|a| a.validator()).collect::<Vec<_>>();
        any_of(validations).into()
    }
}

impl ToValidator for Type {
    fn validator(&self) -> Validator {
        match self {
            Type::Primitive(primitive) => primitive.validator(),
            Type::List(list) => list.validator(),

            Type::Map(map) => {
                todo!("map")
            }
            Type::Struct(stru) => stru.validator(),
            Type::Optional(ty) => ty.kind.validator(),
            Type::Union(a) => a.validator(),
        }
    }
}
