use wasmlib::ScAddress;
use wasmlib::ScAgentId;
use wasmlib::ScChainId;
use wasmlib::ScHash;
use wasmlib::ScRequestId;

pub trait Zero: Sized {
  /// Returns the `0` representation of `Self`.
  fn zero() -> Self;

  /// Returns `true` if the value is `0`.
  fn is_zero(&self) -> bool;
}

macro_rules! impl_zero {
  ($ident:ident, $size:expr) => {
    impl Zero for $ident {
      fn zero() -> Self {
        Self::from_bytes(&[0x00; $size])
      }

      fn is_zero(&self) -> bool {
        self == &Self::zero()
      }
    }
  };
  ($(($ident:ident, $size:expr),)+) => {
    $(
      impl_zero!($ident, $size);
    )+
  };
}

impl_zero! {
  (ScAddress, 33),
  (ScAgentId, 37),
  (ScChainId, 33),
  (ScHash, 32),
  (ScRequestId, 34),
}
