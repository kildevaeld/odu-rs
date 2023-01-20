use crate::arguments::ArgumentError;

#[derive(Debug)]
pub enum Error {
    Argument(ArgumentError),
}

impl From<ArgumentError> for Error {
    fn from(value: ArgumentError) -> Self {
        Error::Argument(value)
    }
}
