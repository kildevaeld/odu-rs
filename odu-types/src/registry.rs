use core::sync::atomic::{AtomicUsize, Ordering};
use once_cell::sync::OnceCell;
use parking_lot::RwLock;

use crate::types::ComplexType;

pub trait HasStaticType {
    fn create_type_info() -> ComplexType;
}

static REGISTRY: OnceCell<RwLock<Registry>> = OnceCell::new();

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TypeId(usize);

impl TypeId {
    fn new() -> TypeId {
        static COUNTER: AtomicUsize = AtomicUsize::new(1);
        TypeId(COUNTER.fetch_add(1, Ordering::Relaxed))
    }

    pub fn data(&self) -> ComplexType {
        Registry::get(self)
    }
}

pub(crate) struct Registry {
    types: ahash::HashMap<TypeId, ComplexType>,
    map: ahash::HashMap<core::any::TypeId, TypeId>,
}

fn registry() -> &'static RwLock<Registry> {
    REGISTRY.get_or_init(|| {
        RwLock::new(Registry {
            types: Default::default(),
            map: Default::default(),
        })
    })
}

pub fn type_id<T: HasStaticType + 'static>() -> TypeId {
    Registry::register::<T>()
}

pub fn type_info(id: TypeId) -> ComplexType {
    Registry::get(&id)
}

pub fn register<V: 'static, F: FnOnce(TypeId) -> ComplexType>(func: F) -> TypeId {
    Registry::register_dynamic::<V, _>(func)
}

impl Registry {
    pub fn register<T: HasStaticType + 'static>() -> TypeId {
        let key = core::any::TypeId::of::<T>();
        if let Some(id) = registry().read().map.get(&key) {
            return *id;
        }

        let type_id = TypeId::new();

        let type_info = T::create_type_info();

        let mut w = registry().write();
        w.types.insert(type_id, type_info);
        w.map.insert(key, type_id);

        type_id
    }

    pub fn register_dynamic<V: 'static, F: FnOnce(TypeId) -> ComplexType>(func: F) -> TypeId {
        let key = core::any::TypeId::of::<V>();
        if let Some(id) = registry().read().map.get(&key) {
            return *id;
        }

        let type_id = TypeId::new();

        let ty = func(type_id);

        let mut w = registry().write();
        w.types.insert(type_id, ty);
        w.map.insert(key, type_id);

        type_id
    }

    pub fn get(id: &TypeId) -> ComplexType {
        let map = registry().read();
        map.types[id].clone()
    }
}
