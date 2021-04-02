//! Commonly used types.

mod alias;
mod tag;
mod value;

pub use self::alias::*;
pub use self::tag::*;
pub use self::value::*;

pub mod wasm {
  pub use wasmlib::ScAddress;
  pub use wasmlib::ScAgentId;
  pub use wasmlib::ScChainId;
  pub use wasmlib::ScColor;
  pub use wasmlib::ScContractId;
  pub use wasmlib::ScHash;
  pub use wasmlib::ScHname;
  pub use wasmlib::ScRequestId;

  pub use super::ScBytes;
  pub use super::ScInt64;
  pub use super::ScString;
  pub use super::ScValue;
}
