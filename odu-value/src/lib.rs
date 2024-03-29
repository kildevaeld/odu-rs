#![no_std]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod index;
mod list;
mod number;
mod object;
mod time;
mod value;

mod from_value;
mod into_value;

mod macros;

mod merge;

#[cfg(feature = "types")]
pub mod types;

pub use self::{
    from_value::*, index::Index, into_value::*, list::*, macros::*, merge::*, number::*, object::*,
    time::Time, value::*,
};

#[cfg(feature = "serde")]
pub mod ser;

#[cfg(feature = "serde")]
pub mod de;

#[cfg(feature = "serde")]
pub use self::{de::from_value, ser::to_value};
