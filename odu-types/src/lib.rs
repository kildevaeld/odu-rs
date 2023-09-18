#![no_std]

mod registry;
mod r#struct;
mod traits;
mod types;

extern crate alloc;

pub use self::{
    r#struct::*,
    registry::{register, register_dynamic, type_id, type_info, HasStaticType, TypeId, len as types_len},
    traits::*,
    types::*,
};

pub use once_cell::sync::Lazy;

#[cfg(feature = "derive")]
pub use odu_macros::*;

