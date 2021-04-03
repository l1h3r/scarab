use core::convert::TryInto;
use wasmlib::*;

use crate::traits::core::Value;

/// Extensions for smart contract [values][Value].
pub trait ValueExt: Value {
  /// Creates a new [Value] from the corresponding primitive.
  fn from_primitive(this: Self::Primitive) -> Self;

  /// Converts the value into the corresponding primitive.
  fn into_primitive(self) -> Self::Primitive;
}

// Blanket implementation for actual primitives
impl<T> ValueExt for T
where
  T: Value<Primitive = T>,
{
  fn from_primitive(this: Self::Primitive) -> Self {
    this
  }

  fn into_primitive(self) -> Self::Primitive {
    self
  }
}

// Special implementation for ScHname
impl ValueExt for ScHname {
  fn from_primitive(this: Self::Primitive) -> Self {
    Self::from_bytes(&this.to_le_bytes())
  }

  fn into_primitive(self) -> Self::Primitive {
    self
      .to_bytes()
      .try_into()
      .map(u32::from_le_bytes)
      .expect("ValueExt::into_primitive: infallible")
  }
}

// Generic implementation for hash types
macro_rules! impl_ValueExt {
  ($ident:ident) => {
    impl ValueExt for $ident {
      fn from_primitive(this: Self::Primitive) -> Self {
        Self::from_bytes(&this)
      }

      fn into_primitive(self) -> Self::Primitive {
        self
          .to_bytes()
          .try_into()
          .expect("ValueExt::into_primitive: infallible")
      }
    }
  };
  ($($ident:ident,)+) => {
    $(
      impl_ValueExt!($ident);
    )+
  };
}

impl_ValueExt! {
  ScAddress,
  ScAgentId,
  ScChainId,
  ScColor,
  ScHash,
  ScRequestId,
}
