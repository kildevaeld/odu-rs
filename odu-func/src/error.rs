use crate::arguments::ArgumentError;
use alloc::boxed::Box;

#[derive(Debug)]
pub enum Error {
    Argument(ArgumentError),
    Runtime(Box<dyn core::fmt::Debug + Send + Sync>),
    Infallible,
}

impl From<ArgumentError> for Error {
    fn from(value: ArgumentError) -> Self {
        Error::Argument(value)
    }
}

impl From<core::convert::Infallible> for Error {
    fn from(_value: core::convert::Infallible) -> Self {
        Error::Infallible
    }
}
