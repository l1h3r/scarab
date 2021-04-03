use core::convert::TryInto;
use core::mem;

use crate::traits::core::Proxy;
use crate::types::ScBytes;

mod private {
  pub trait Sealed {}
}

pub trait Integer: private::Sealed + Sized {
  type Output: AsRef<[u8]>;

  /// Decodes a new integer from little-endian bytes.
  fn decode(slice: &[u8]) -> Option<Self>;

  /// Encodes an integer as little-endian bytes.
  fn encode(&self) -> Self::Output;

  /// Decodes a new integer from a vector of little-endian bytes.
  fn decode_vec(data: Vec<u8>) -> Option<Self> {
    Self::decode(&data)
  }

  /// Encodes an integer as a vector of little-endian bytes.
  fn encode_vec(&self) -> Vec<u8> {
    self.encode().as_ref().to_vec()
  }
}

macro_rules! impl_Integer {
  ($ident:ident) => {
    impl private::Sealed for $ident {}

    impl Integer for $ident {
      type Output = [u8; mem::size_of::<$ident>()];

      fn decode(slice: &[u8]) -> Option<Self> {
        slice.try_into().ok().map($ident::from_le_bytes)
      }

      fn encode(&self) -> Self::Output {
        self.to_le_bytes()
      }
    }
  };
  ($($ident:ident),+ $(,)*) => {
    $(
      impl_Integer!($ident);
    )+
  };
}

impl_Integer! {
  u8, u16, u32, u64,
  i8, i16, i32, i64,
}

/// An interface for converting values to [integers][Integer].
pub trait ToInteger {
  fn to_integer<I: Integer>(&self) -> Option<I>;
}

// Implement for byte vectors
impl ToInteger for ScBytes {
  fn to_integer<I: Integer>(&self) -> Option<I> {
    I::decode(self)
  }
}

// Implement for byte vector proxies
impl<T: Proxy<Value = ScBytes>> ToInteger for T {
  fn to_integer<I: Integer>(&self) -> Option<I> {
    if self.has() {
      self.get().to_integer()
    } else {
      None
    }
  }
}
