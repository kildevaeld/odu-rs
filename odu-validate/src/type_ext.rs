use alloc::vec::Vec;
use odu_types::{ComplexType, List, PrimitiveType, Struct, Type, Union};

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
    impl Sealed for odu_types::PrimitiveType {}
    impl Sealed for odu_types::Struct {}
}

pub trait ToValidator {
    fn validator(&self) -> Validator;
}

impl ToValidator for PrimitiveType {
    fn validator(&self) -> Validator {
        match self {
            PrimitiveType::Bool => BoolValidator::default().into(),
            PrimitiveType::String => StringValidator::default().into(),
            PrimitiveType::I8
            | PrimitiveType::U8
            | PrimitiveType::I16
            | PrimitiveType::U16
            | PrimitiveType::I64
            | PrimitiveType::U64 => NumberValidator::default().into(),
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
            Type::Complex(ty) => match ty.data() {
                ComplexType::List(list) => list.validator(),

                ComplexType::Map(_) => {
                    todo!("map")
                }
                ComplexType::Struct(stru) => stru.validator(),
                ComplexType::Optional(ty) => ty.kind.validator(),
                ComplexType::Union(a) => a.validator(),
            },
        }
    }
}
