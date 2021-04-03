use wasmlib::ScAddress;
use wasmlib::ScAgentId;
use wasmlib::ScChainId;
use wasmlib::ScHash;
use wasmlib::ScRequestId;

mod private {
  pub trait Sealed: Sized + PartialEq {}
}

/// Extensions for hash types (eg. [ScAgentId]/[ScHash]).
pub trait HashExt<const SIZE: usize>: private::Sealed {
  /// The size of the hash type (in bytes).
  const SIZE: usize = SIZE;

  /// The `zero` value of the hash type.
  const ZERO: &'static [u8; SIZE] = &[0; SIZE];

  /// Returns the `zero` value of the hash type.
  fn zero() -> Self {
    Self::import(Self::ZERO)
  }

  /// Returns `true` if the hash type is `zero`.
  fn is_zero(&self) -> bool {
    self.export() == Self::ZERO
  }

  /// Creates a new hash type from a slice of bytes.
  fn import(bytes: &[u8]) -> Self;

  /// Exports the hash type as a slice of bytes.
  fn export(&self) -> &[u8];
}

macro_rules! impl_HashExt {
  ($ident:ident, $size:expr) => {
    impl private::Sealed for $ident {}

    impl HashExt<$size> for $ident {
      fn import(bytes: &[u8]) -> Self {
        Self::from_bytes(bytes)
      }

      fn export(&self) -> &[u8] {
        self.to_bytes()
      }
    }
  };
  ($(($ident:ident, $size:expr),)+) => {
    $(
      impl_HashExt!($ident, $size);
    )+
  };
}

impl_HashExt! {
  (ScAddress, 33),
  (ScAgentId, 37),
  (ScChainId, 33),
  (ScHash, 32),
  (ScRequestId, 34),
}
