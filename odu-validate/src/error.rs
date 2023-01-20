use alloc::vec::Vec;
use odu_types::Type;
use odu_value::Value;

#[derive(Debug)]
pub enum Operator {
    Min,
    Max,
    Equal,
    NotEqual,
}

#[derive(Debug)]
pub enum ValidationError {
    Required,
    InvalidType {
        expected: Type,
        found: Type,
    },
    Min {
        expected: usize,
        found: usize,
    },
    Compare {
        operator: Operator,
        expected: Value,
        found: Value,
    },
    OneOf(Vec<ValidationError>),
    Multi(Vec<ValidationError>),
}

#[derive(Debug)]
pub enum Error {
    Multi(Vec<ValidationError>),
    Validation(ValidationError),
}
