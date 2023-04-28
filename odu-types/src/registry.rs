use core::sync::atomic::{AtomicUsize, Ordering};

use once_cell::sync::OnceCell;
use parking_lot::RwLock;

use crate::types::StaticType;

pub trait HasStaticType {
    fn static_type() -> StaticType;
}

static REGISTRY: OnceCell<RwLock<Registry>> = OnceCell::new();

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TypeId(usize);

impl TypeId {
    fn new() -> TypeId {
        static COUNTER: AtomicUsize = AtomicUsize::new(1);
        TypeId(COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}

struct Registry {
    types: ahash::HashMap<TypeId, StaticType>,
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

impl Registry {
    pub fn register<T: HasStaticType + 'static>() -> TypeId {
        let key = core::any::TypeId::of::<T>();
        if let Some(id) = registry().read().map.get(&key) {
            return *id;
        }

        let type_id = TypeId::new();

        let mut w = registry().write();
        w.types.insert(type_id, T::static_type());
        w.map.insert(key, type_id);

        type_id
    }

    pub fn get(id: &TypeId) -> StaticType {
        let map = registry().read();
        map.types[id].clone()
    }
}
