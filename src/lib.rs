//! Tools for working with IOTA Smart Contracts

#![feature(core_intrinsics)]
#![feature(const_unreachable_unchecked)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(
  rust_2018_idioms,
  unreachable_pub,
  // missing_docs,
  rustdoc::missing_crate_level_docs,
  rustdoc::broken_intra_doc_links,
  rustdoc::private_intra_doc_links,
  rustdoc::private_doc_tests,
  clippy::missing_safety_doc,
  clippy::missing_errors_doc
)]

// Enable derive macros within this crate
#[allow(unused_extern_crates)]
extern crate self as scarab;

// Re-export wasmlib for convenience
pub use ::wasmlib;

#[macro_use]
mod macros;

#[doc(hidden)]
pub use scarab_derive::*;

#[doc(hidden)]
pub mod panic;

pub mod consts;
pub mod contracts;
pub mod traits;
pub mod types;
pub mod utils;

#[cfg(feature = "rand")]
#[cfg_attr(docsrs, doc(cfg(feature = "rand")))]
pub mod rand;

pub mod prelude {
  //! The Scarab Prelude.
  //!
  //! This module exists to alleviate imports of common traits for working with
  //! IOTA Smart Contracts.
  pub use crate::traits::extension::ColorExt;
  pub use crate::traits::extension::ContextExt;
  pub use crate::traits::extension::HashExt;
  pub use crate::traits::extension::MapExt;
  pub use crate::traits::extension::ValueExt;
  pub use crate::traits::math::Integer;
  pub use crate::traits::utility::Export;
}

#[doc(hidden)]
pub mod export {
  pub use crate::traits::utility::Decode;
  pub use crate::traits::utility::Encode;
  pub use wasmlib::BytesDecoder;
  pub use wasmlib::BytesEncoder;
}
