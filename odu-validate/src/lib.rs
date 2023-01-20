#![no_std]

extern crate alloc;

mod error;
mod validation;
pub mod validations;
mod validators;

pub use self::{
    error::{Error, ValidationError},
    type_ext::*,
    validation::{Validation, ValidationBox, ValidationList},
    validators::*,
};

mod type_ext;
