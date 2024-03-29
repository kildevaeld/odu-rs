use crate::{Validation, ValidationBox, ValidationError, ValidationList};
use alloc::vec::Vec;
use odu_value::Value;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug)]
pub struct Tuple(pub Vec<ValidationBox>);

#[cfg_attr(feature = "serde", typetag::serde(name = "tuple"))]
impl Validation for Tuple {
    // fn as_any(&self) -> &dyn Any {
    //     self
    // }
    fn validate(&self, value: &Value) -> Result<(), ValidationError> {
        let list = match value.as_list() {
            Some(list) => list,
            None => {
                // return Err(ValidationError::InvalidType {
                //     expected: ValueType::List.into(),
                //     found: value.ty().into(),
                // })
                todo!()
            }
        };

        if list.len() != self.0.len() {
            panic!("not equal len");
        }

        let values = self.0.iter().zip(list.iter());

        let mut errors = Vec::default();
        for (idx, (validation, value)) in values.enumerate() {
            if let Err(err) = validation.validate(value) {
                errors.push((idx, err));
            }
        }

        Ok(())
    }
}

pub fn tuple<V: ValidationList>(value: V) -> Tuple {
    Tuple(value.into_list())
}
