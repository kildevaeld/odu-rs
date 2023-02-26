#![no_std]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod index;
mod list;
mod number;
mod object;
mod value;

mod from_value;
mod into_value;

mod macros;

mod merge;

pub use self::{
    from_value::*, index::Index, into_value::*, list::*, macros::*, merge::*, number::*, object::*,
    value::*,
};

#[cfg(feature = "serde")]
pub mod ser;

#[cfg(feature = "serde")]
pub mod de;

#[cfg(feature = "serde")]
pub use self::{de::from_value, ser::to_value};
