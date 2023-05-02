use crate::value::Value;
use alloc::string::{String, ToString};
use core::ops;

#[cfg(all(not(feature = "ord"), feature = "std"))]
pub(crate) type MapImpl = std::collections::HashMap<String, Value>;
#[cfg(all(not(feature = "ord"), feature = "std"))]
type Iter<'a, K, V> = std::collections::hash_map::Iter<'a, K, V>;
#[cfg(all(not(feature = "ord"), feature = "std"))]
type IntoIter<K, V> = std::collections::hash_map::IntoIter<K, V>;
#[cfg(all(not(feature = "ord"), feature = "std"))]
type IterMut<'a, K, V> = std::collections::hash_map::IterMut<'a, K, V>;
#[cfg(all(not(feature = "ord"), feature = "std"))]
type Entry<'a, K, V> = std::collections::hash_map::Entry<'a, K, V>;

#[cfg(any(feature = "ord", not(feature = "std")))]
pub(crate) type MapImpl = alloc::collections::BTreeMap<String, Value>;
#[cfg(any(feature = "ord", not(feature = "std")))]
type Iter<'a, K, V> = alloc::collections::btree_map::Iter<'a, K, V>;
#[cfg(any(feature = "ord", not(feature = "std")))]
type IntoIter<K, V> = alloc::collections::btree_map::IntoIter<K, V>;
#[cfg(any(feature = "ord", not(feature = "std")))]
type IterMut<'a, K, V> = alloc::collections::btree_map::IterMut<'a, K, V>;
#[cfg(any(feature = "ord", not(feature = "std")))]
type Entry<'a, K, V> = alloc::collections::btree_map::Entry<'a, K, V>;

#[cfg_attr(feature = "ord", derive(Hash, PartialOrd, Ord))]
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Map {
    pub(crate) inner: MapImpl,
}

impl Map {
    #[cfg(all(not(feature = "ord"), feature = "std"))]
    pub fn with_capacity(len: usize) -> Map {
        Map {
            inner: MapImpl::with_capacity(len),
        }
    }

    #[cfg(any(feature = "ord", not(feature = "std")))]
    pub fn with_capacity(_: usize) -> Map {
        Map {
            inner: MapImpl::default(),
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

impl FromIterator<(String, Value)> for Map {
    fn from_iter<T: IntoIterator<Item = (String, Value)>>(iter: T) -> Self {
        Map {
            inner: MapImpl::from_iter(iter),
        }
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

// #[cfg(all(not(feature = "ord"), feature = "std"))]
// impl From<std::collections::HashMap<String, Value>> for Map {
//     fn from(map: std::collections::HashMap<String, Value>) -> Map {
//         Map { inner: map }
//     }
// }

#[cfg(all(not(feature = "ord"), feature = "std"))]
impl<S> From<std::collections::HashMap<String, Value, S>> for Map {
    fn from(map: std::collections::HashMap<String, Value, S>) -> Map {
        Map {
            inner: MapImpl::from_iter(map),
        }
    }
}

#[cfg(any(feature = "ord", not(feature = "std")))]
impl From<alloc::collections::BTreeMap<String, Value>> for Map {
    fn from(map: alloc::collections::BTreeMap<String, Value>) -> Map {
        Map { inner: map }
    }
}
