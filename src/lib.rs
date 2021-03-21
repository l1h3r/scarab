//! Tools for working with IOTA Smart Contracts

#![warn(rust_2018_idioms, unreachable_pub)]

// Enable derive macros within this crate
extern crate self as scarab;

// Re-export wasmlib for convenience
pub use ::wasmlib;

#[macro_use]
mod macros;

#[doc(hidden)]
pub use scarab_derive::*;

pub mod consts;
pub mod contracts;
pub mod traits;
pub mod types;
pub mod utils;

pub mod prelude {
  pub use crate::traits::ColorExt;
  pub use crate::traits::ContextExt;
  pub use crate::traits::Export;
  pub use crate::traits::MapExt;
  pub use crate::traits::Zero;
}

#[doc(hidden)]
pub mod export {
  pub use crate::traits::Decode;
  pub use crate::traits::Encode;
  pub use wasmlib::BytesDecoder;
  pub use wasmlib::BytesEncoder;
}
