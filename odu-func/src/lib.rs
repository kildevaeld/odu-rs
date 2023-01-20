#![no_std]

extern crate alloc;

pub mod arguments;
mod callable;
mod callable_async;
mod callable_fn;
mod error;
mod func;
mod resultable;
pub mod signature;

pub use self::{callable::*, callable_async::*, callable_fn::*, error::*, resultable::*};

#[cfg(feature = "derive")]
pub use odu_macros::*;
