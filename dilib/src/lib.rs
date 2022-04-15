// Include docs
#![doc = include_str!("../../README.md")]

mod lib_macros;
pub use lib_macros::*;

mod injectable;
pub use injectable::*;

mod key;
pub use key::*;

mod container;
pub use container::*;

mod provider;
pub use provider::*;

mod scoped;
pub use scoped::*;

mod shared;
pub use shared::*;

mod resolved;
pub use resolved::*;

#[cfg(feature = "derive")]
mod derive {
    extern crate dilib_derive;
    pub use dilib_derive::*;
}

#[cfg(feature = "derive")]
pub use derive::*;

/// A global instance of `Container`.
#[cfg(feature = "global")]
pub mod global;

/// procedural macros of `dilib`.
#[cfg(feature = "unstable_provide")]
pub mod macros {
    pub use dilib_macros::*;
}

#[cfg(feature = "unstable_provide")]
pub use macros::*;

#[cfg(feature = "unstable_provide")]
pub use ctor::*;
