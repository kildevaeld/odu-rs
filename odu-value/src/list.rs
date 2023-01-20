use alloc::vec::Vec;
use odu_types::HasType;

use crate::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct List {
    pub(crate) v: Vec<Value>,
}

impl List {
    pub const fn new() -> List {
        List { v: Vec::new() }
    }

    pub fn get(&self, idx: usize) -> Option<&Value> {
        self.v.get(idx)
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut Value> {
        self.v.get_mut(idx)
    }

    pub fn len(&self) -> usize {
        self.v.len()
    }

    pub fn is_empty(&self) -> bool {
        self.v.is_empty()
    }

    pub fn extend<I: IntoIterator<Item = Value>>(&mut self, iter: I) {
        self.v.extend(iter)
    }

    pub fn iter(&self) -> core::slice::Iter<'_, Value> {
        self.v.iter()
    }
}

impl From<Vec<Value>> for List {
    fn from(value: Vec<Value>) -> Self {
        List { v: value }
    }
}

impl IntoIterator for List {
    type Item = Value;

    type IntoIter = alloc::vec::IntoIter<Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.v.into_iter()
    }
}

impl HasType for List {
    fn typed(&self) -> odu_types::Type {
        todo!()
    }
}
