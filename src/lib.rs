//! Tools for working with IOTA Smart Contracts

#![warn(rust_2018_idioms, unreachable_pub)]

#[macro_use]
mod macros;

pub mod consts;
pub mod contracts;
pub mod traits;
pub mod utils;

pub mod prelude {
  pub use crate::traits::ColorExt;
  pub use crate::traits::ContextExt;
  pub use crate::traits::Export;
  pub use crate::traits::MapExt;
  pub use crate::traits::Zero;
}
