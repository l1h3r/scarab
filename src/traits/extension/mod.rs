//! Extension traits provide new functionality for WasmLib values and proxy objects.

mod color;
mod context;
mod hash;
mod map;
mod value;

pub use self::color::*;
pub use self::context::*;
pub use self::hash::*;
pub use self::map::*;
pub use self::value::*;
