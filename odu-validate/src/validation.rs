use crate::error::ValidationError;
use alloc::{boxed::Box, vec::Vec};
use odu_value::Value;

pub type ValidationBox = alloc::boxed::Box<dyn Validation>;

#[cfg_attr(feature = "serde", typetag::serde(tag = "type"))]
pub trait Validation: core::fmt::Debug {
    // fn as_any(&self) -> &dyn Any;
    fn validate(&self, value: &Value) -> Result<(), ValidationError>;
}

#[cfg_attr(feature = "serde", typetag::serde(name = "boxed"))]
impl Validation for Box<dyn Validation> {
    fn validate(&self, value: &Value) -> Result<(), ValidationError> {
        (**self).validate(value)
    }
}

pub trait ValidationList {
    fn into_list(self) -> Vec<ValidationBox>;
}

impl ValidationList for Vec<ValidationBox> {
    fn into_list(self) -> Vec<ValidationBox> {
        self
    }
}
