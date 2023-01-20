use super::Parameters;
use odu_types::{Primitive, Type};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct Signature {
    params: Parameters,
    return_type: Type,
}

impl Signature {
    pub fn new(params: Parameters, return_type: Type) -> Signature {
        Signature {
            params,
            return_type,
        }
    }

    pub fn params(&self) -> &Parameters {
        &self.params
    }

    pub fn return_type(&self) -> &Type {
        &self.return_type
    }
}

impl Default for Signature {
    fn default() -> Self {
        Signature {
            params: Parameters::default(),
            return_type: Primitive::Void.into(),
        }
    }
}

pub struct Call {}
