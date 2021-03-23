//! Tools for working with IOTA Smart Contracts

#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(
  rust_2018_idioms,
  unreachable_pub,
  rustdoc::missing_crate_level_docs,
  rustdoc::broken_intra_doc_links,
  rustdoc::private_intra_doc_links,
  rustdoc::private_doc_tests,
  clippy::missing_safety_doc,
  clippy::missing_errors_doc
)]

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

#[cfg(feature = "rand")]
#[cfg_attr(docsrs, doc(cfg(feature = "rand")))]
pub mod rand;

pub mod prelude {
  //! The Scarab Prelude.
  //!
  //! This module exists to alleviate imports of common traits for working with
  //! IOTA Smart Contracts.
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
