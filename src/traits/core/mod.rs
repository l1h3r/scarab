//! Core traits associate WasmLib values and proxy objects.

mod array;
mod impls;
mod proxy;
mod state;
mod value;

pub use self::array::*;
pub use self::proxy::*;
pub use self::state::*;
pub use self::value::*;
