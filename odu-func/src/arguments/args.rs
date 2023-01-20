use super::error::ArgumentError;
use alloc::vec::Vec;
use odu_types::{HasType, Type};
use odu_value::Value;

#[derive(Debug, Default)]
pub struct Arguments {
    args: Vec<Value>,
}

impl Arguments {
    pub fn new(arity: usize) -> Arguments {
        Arguments {
            args: (0..arity).map(|_| Value::None).collect(),
        }
    }
}

impl Arguments {
    pub fn try_get_ref<'a, V: TryFrom<&'a Value>>(&'a self, idx: usize) -> Result<V, ArgumentError>
    where
        V::Error: Into<ArgumentError>,
    {
        let val = match self.args.get(idx) {
            Some(ret) => ret,
            None => {
                return Err(ArgumentError::Missing {
                    index: idx,
                    arity: self.args.len(),
                })
            }
        };

        V::try_from(val).map_err(|err| err.into())
    }

    pub fn try_get<V: TryFrom<Value>>(&self, idx: usize) -> Result<V, ArgumentError>
    where
        V::Error: Into<ArgumentError>,
    {
        let val = match self.args.get(idx) {
            Some(ret) => ret,
            None => {
                return Err(ArgumentError::Missing {
                    index: idx,
                    arity: self.args.len(),
                })
            }
        };

        V::try_from(val.clone()).map_err(|err| err.into())
    }

    pub fn get(&self, idx: usize) -> Option<&Value> {
        self.args.get(idx)
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut Value> {
        self.args.get_mut(idx)
    }

    pub fn len(&self) -> usize {
        self.args.len()
    }

    pub fn is_empty(&self) -> bool {
        self.args.is_empty()
    }

    pub fn types(&self) -> Vec<Type> {
        self.args.iter().map(|m| m.typed()).collect()
    }
}

#[derive(Debug, Default)]
pub struct ArgumentsBuilder {
    args: Vec<Value>,
}

impl ArgumentsBuilder {
    pub fn with<V: Into<Value>>(mut self, value: V) -> Self {
        self.args.push(value.into());
        self
    }

    pub fn add<V: Into<Value>>(&mut self, value: V) -> &mut Self {
        self.args.push(value.into());
        self
    }

    pub fn build(self) -> Arguments {
        Arguments { args: self.args }
    }
}
