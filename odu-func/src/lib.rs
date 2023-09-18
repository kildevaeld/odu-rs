#![no_std]

extern crate alloc;

pub mod arguments;
mod callable;
#[cfg(feature = "async")]
mod callable_async;
mod callable_fn;
mod error;
mod func;
mod resultable;
pub mod signature;

pub use self::{callable::*, callable_fn::*, error::*, resultable::*};

#[cfg(feature = "async")]
pub use self::callable_async::*;

#[cfg(feature = "derive")]
pub use odu_macros::*;
