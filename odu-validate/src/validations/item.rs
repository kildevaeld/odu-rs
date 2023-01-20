use crate::{Validation, ValidationBox, ValidationError};
use alloc::boxed::Box;
use core::fmt;
use odu_value::Value;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Item {
    pub validator: ValidationBox,
}

impl fmt::Debug for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Item").finish()
    }
}

#[cfg_attr(feature = "serde", typetag::serde(name = "item"))]
impl Validation for Item {
    fn validate(&self, _: &Value) -> Result<(), ValidationError> {
        Ok(())
    }
}

pub fn item<V: Validation + 'static>(value: V) -> Item {
    Item {
        validator: Box::new(value),
    }
}
