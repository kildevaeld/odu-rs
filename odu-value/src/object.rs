use crate::value::Value;
use alloc::string::{String, ToString};
use core::ops;
use hashbrown::{
    hash_map::{IntoIter, Iter, IterMut},
    HashMap as StdHashMap,
};
use odu_types::HasType;

#[cfg(not(feature = "std"))]
pub type HashBuilder = hashbrown::hash_map::DefaultHashBuilder;

#[cfg(feature = "std")]
pub type HashBuilder = std::collections::hash_map::RandomState;

pub type Entry<'a, K, V> = hashbrown::hash_map::Entry<'a, K, V, HashBuilder>;

pub type HashMap<K, V> = StdHashMap<K, V, HashBuilder>;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Map {
    pub(crate) inner: StdHashMap<String, Value, HashBuilder>,
}

impl Map {
    pub fn with_capacity(len: usize) -> Map {
        Map {
            inner: HashMap::with_capacity(len),
        }
    }

    #[inline]
    pub fn insert(&mut self, name: impl ToString, value: impl Into<Value>) -> Option<Value> {
        self.inner.insert(name.to_string(), value.into())
    }

    #[inline]
    pub fn get(&self, name: impl AsRef<str>) -> Option<&Value> {
        self.inner.get(name.as_ref())
    }

    #[inline]
    pub fn get_mut(&mut self, name: impl AsRef<str>) -> Option<&mut Value> {
        self.inner.get_mut(name.as_ref())
    }

    #[inline]
    pub fn contains(&self, name: impl AsRef<str>) -> bool {
        self.inner.contains_key(name.as_ref())
    }

    #[inline]
    pub fn remove(&mut self, name: impl AsRef<str>) -> Option<Value> {
        self.inner.remove(name.as_ref())
    }

    #[inline]
    pub fn entry<S>(&mut self, key: S) -> Entry<'_, String, Value>
    where
        S: Into<String>,
    {
        self.inner.entry(key.into())
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    #[inline]
    pub fn iter(&self) -> Iter<'_, String, Value> {
        self.inner.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, String, Value> {
        self.inner.iter_mut()
    }
}

impl Extend<(String, Value)> for Map {
    fn extend<T: IntoIterator<Item = (String, Value)>>(&mut self, iter: T) {
        self.inner.extend(iter)
    }
}

impl IntoIterator for Map {
    type Item = (String, Value);
    type IntoIter = IntoIter<String, Value>;
    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a> IntoIterator for &'a Map {
    type Item = (&'a String, &'a Value);
    type IntoIter = Iter<'a, String, Value>;
    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl<'a> ops::Index<&'a str> for Map {
    type Output = Value;

    fn index(&self, index: &'a str) -> &Value {
        static NULL: Value = Value::None;
        self.inner.get(index).unwrap_or(&NULL)
    }
}

impl<'a> ops::IndexMut<&'a str> for Map {
    fn index_mut(&mut self, index: &'a str) -> &mut Value {
        if !self.contains(index) {
            self.inner.insert(index.to_string(), Value::None);
        }
        self.inner.get_mut(index).unwrap()
    }
}

impl From<StdHashMap<String, Value, HashBuilder>> for Map {
    fn from(map: StdHashMap<String, Value, HashBuilder>) -> Map {
        Map { inner: map }
    }
}

impl HasType for Map {
    fn typed(&self) -> odu_types::Type {
        todo!()
    }
}
