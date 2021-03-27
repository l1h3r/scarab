use core::convert::TryInto;

use crate::traits::Container;
use crate::types::ScBytes;

pub trait ToUint<UInt>
where
  Self: Container<Value = ScBytes>,
{
  fn to_uint(&self) -> Option<UInt> {
    if self.has() {
      Self::parse(self.get())
    } else {
      None
    }
  }

  fn parse(data: Vec<u8>) -> Option<UInt>;
}

macro_rules! impl_ToUint {
  ($uint:ty) => {
    impl<T> ToUint<$uint> for T
    where
      Self: Container<Value = ScBytes>
    {
      fn parse(data: Vec<u8>) -> Option<$uint> {
        data.try_into().ok().map(<$uint>::from_le_bytes)
      }
    }
  };
  ($($uint:ty)+) => {
    $(
      impl_ToUint!($uint);
    )+
  };
}

impl_ToUint!(u16 u32 u64);
