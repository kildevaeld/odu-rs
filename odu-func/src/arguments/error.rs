use core::{convert::Infallible, fmt};
use odu_types::Type;
use odu_value::FromValueErr;

#[derive(Debug)]
pub enum ArgumentError {
    Infallible,
    IvalidType { expected: Type, found: Type },
    Missing { index: usize, arity: usize },
    Value(FromValueErr<'static>),
}

impl fmt::Display for ArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArgumentError::Infallible => {
                write!(f, "infallible")
            }
            ArgumentError::IvalidType { expected, found } => {
                write!(f, "invalid type. Expected: {expected:?}, found: {found:?}")
            }
            ArgumentError::Missing { index, .. } => {
                write!(f, "missing argument at index: {index:}")
            }
            ArgumentError::Value(err) => {
                write!(f, "invalid type: {err:?}")
            }
        }
    }
}

impl From<Infallible> for ArgumentError {
    fn from(_: Infallible) -> Self {
        ArgumentError::Infallible
    }
}

impl<'a> From<FromValueErr<'a>> for ArgumentError {
    fn from(e: FromValueErr<'a>) -> Self {
        ArgumentError::Value(e.to_owned())
    }
}
