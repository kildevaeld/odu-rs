use crate::{Validation, ValidationError};
use odu_types::{HasType, Primitive, Type};
use odu_value::Value;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy)]
pub struct TypeValidation(Primitive);

#[cfg_attr(feature = "serde", typetag::serde(name = "type"))]
impl Validation for TypeValidation {
    fn validate(&self, value: &Value) -> Result<(), ValidationError> {
        let ty = value.typed();

        match ty.as_primitive() {
            Some(primitive) if primitive == &self.0 => Ok(()),
            _ => Err(ValidationError::InvalidType {
                expected: Type::Primitive(self.0),
                found: ty,
            }),
        }
    }
}

pub fn typed(ty: Primitive) -> TypeValidation {
    TypeValidation(ty)
}
