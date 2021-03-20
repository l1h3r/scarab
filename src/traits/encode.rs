use wasmlib::BytesDecoder;
use wasmlib::BytesEncoder;
use wasmlib::ScAddress;
use wasmlib::ScAgentId;
use wasmlib::ScChainId;
use wasmlib::ScColor;
use wasmlib::ScContractId;
use wasmlib::ScHash;
use wasmlib::ScHname;
use wasmlib::ScRequestId;

use crate::types::ScBytes;
use crate::types::ScInt64;
use crate::types::ScString;

pub trait Encode {
  fn encode(&self, encoder: &mut BytesEncoder);

  fn to_bytes(&self) -> Vec<u8> {
    let mut encoder: BytesEncoder = BytesEncoder::new();

    self.encode(&mut encoder);

    encoder.data()
  }
}

impl<'a, T> Encode for &'a T
where
  T: Encode,
{
  fn encode(&self, encoder: &mut BytesEncoder) {
    (**self).encode(encoder);
  }
}

pub trait Decode: Sized {
  fn decode(decoder: &mut BytesDecoder<'_>) -> Self;

  fn from_bytes(bytes: &[u8]) -> Self {
    Self::decode(&mut BytesDecoder::new(bytes))
  }
}

// =============================================================================
// Implementations for Core Types
// =============================================================================

macro_rules! impl_Encode {
  (@encode, $ident:ident, $fn:ident) => {
    impl Encode for $ident {
      fn encode(&self, encoder: &mut BytesEncoder) {
        encoder.$fn(self);
      }
    }
  };
  (@decode, $ident:ident, $fn:ident) => {
    impl Decode for $ident {
      fn decode(decoder: &mut BytesDecoder<'_>) -> Self {
        decoder.$fn()
      }
    }
  };
  (ScInt64, $fn:ident) => {
    impl Encode for ScInt64 {
      fn encode(&self, encoder: &mut BytesEncoder) {
        encoder.$fn(*self);
      }
    }
    impl_Encode!(@decode, ScInt64, $fn);
  };
  (ScBytes, $fn:ident) => {
    impl Decode for ScBytes {
      fn decode(decoder: &mut BytesDecoder<'_>) -> Self {
        decoder.$fn().to_vec()
      }
    }
    impl_Encode!(@encode, ScBytes, $fn);
  };
  ($ident:ident, $fn:ident) => {
    impl_Encode!(@encode, $ident, $fn);
    impl_Encode!(@decode, $ident, $fn);
  };
  ($(($ident:ident, $fn:ident),)+) => {
    $(
      impl_Encode!($ident, $fn);
    )+
  };
}

impl_Encode! {
  (ScAddress, address),
  (ScAgentId, agent_id),
  (ScBytes, bytes),
  (ScChainId, chain_id),
  (ScColor, color),
  (ScContractId, contract_id),
  (ScHash, hash),
  (ScHname, hname),
  (ScInt64, int64),
  (ScRequestId, request_id),
  (ScString, string),
}

// Add support for encoding byte slices
impl Encode for [u8] {
  fn encode(&self, encoder: &mut BytesEncoder) {
    encoder.bytes(self);
  }
}

// Add support for encoding string slices
impl Encode for str {
  fn encode(&self, encoder: &mut BytesEncoder) {
    encoder.string(self);
  }
}
