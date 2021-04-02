use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result;
use wasmlib::host;
use wasmlib::BytesDecoder;
use wasmlib::BytesEncoder;

use crate::types::wasm::*;
use crate::types::ScTag;

#[derive(Clone, PartialEq)]
pub enum ScValue {
  Address(ScAddress),
  AgentId(ScAgentId),
  Bytes(ScBytes),
  ChainId(ScChainId),
  Color(ScColor),
  Hash(ScHash),
  Hname(ScHname),
  Int64(ScInt64),
  RequestId(ScRequestId),
  String(ScString),
}

impl ScValue {
  pub fn from_bytes(tag: ScTag, bytes: &[u8]) -> Self {
    let mut decoder: BytesDecoder<'_> = BytesDecoder::new(bytes);

    match tag {
      ScTag::Address => Self::Address(decoder.address()),
      ScTag::AgentId => Self::AgentId(decoder.agent_id()),
      ScTag::Bytes => Self::Bytes(decoder.bytes().to_vec()),
      ScTag::ChainId => Self::ChainId(decoder.chain_id()),
      ScTag::Color => Self::Color(decoder.color()),
      ScTag::Hash => Self::Hash(decoder.hash()),
      ScTag::Hname => Self::Hname(decoder.hname()),
      ScTag::Int64 => Self::Int64(decoder.int64()),
      ScTag::RequestId => Self::RequestId(decoder.request_id()),
      ScTag::String => Self::String(decoder.string()),
    }
  }

  pub fn to_bytes(&self) -> (ScTag, Vec<u8>) {
    (self.tag(), self.to_untagged_bytes())
  }

  pub fn to_untagged_bytes(&self) -> Vec<u8> {
    let mut encoder: BytesEncoder = BytesEncoder::new();

    match self {
      Self::Address(inner) => encoder.address(inner).data(),
      Self::AgentId(inner) => encoder.agent_id(inner).data(),
      Self::Bytes(inner) => encoder.bytes(inner).data(),
      Self::ChainId(inner) => encoder.chain_id(inner).data(),
      Self::Color(inner) => encoder.color(inner).data(),
      Self::Hash(inner) => encoder.hash(inner).data(),
      Self::Hname(inner) => encoder.hname(inner).data(),
      Self::Int64(inner) => encoder.int64(*inner).data(),
      Self::RequestId(inner) => encoder.request_id(inner).data(),
      Self::String(inner) => encoder.string(inner).data(),
    }
  }

  pub const fn tag(&self) -> ScTag {
    match self {
      Self::Address(_) => ScTag::Address,
      Self::AgentId(_) => ScTag::AgentId,
      Self::Bytes(_) => ScTag::Bytes,
      Self::ChainId(_) => ScTag::ChainId,
      Self::Color(_) => ScTag::Color,
      Self::Hash(_) => ScTag::Hash,
      Self::Hname(_) => ScTag::Hname,
      Self::Int64(_) => ScTag::Int64,
      Self::RequestId(_) => ScTag::RequestId,
      Self::String(_) => ScTag::String,
    }
  }

  pub const fn is_address(&self) -> bool {
    matches!(self, Self::Address(_))
  }

  pub const fn is_agent_id(&self) -> bool {
    matches!(self, Self::AgentId(_))
  }

  pub const fn is_bytes(&self) -> bool {
    matches!(self, Self::Bytes(_))
  }

  pub const fn is_chain_id(&self) -> bool {
    matches!(self, Self::ChainId(_))
  }

  pub const fn is_color(&self) -> bool {
    matches!(self, Self::Color(_))
  }

  pub const fn is_hash(&self) -> bool {
    matches!(self, Self::Hash(_))
  }

  pub const fn is_hname(&self) -> bool {
    matches!(self, Self::Hname(_))
  }

  pub const fn is_int64(&self) -> bool {
    matches!(self, Self::Int64(_))
  }

  pub const fn is_request_id(&self) -> bool {
    matches!(self, Self::RequestId(_))
  }

  pub const fn is_string(&self) -> bool {
    matches!(self, Self::String(_))
  }

  pub const fn host_type(&self) -> i32 {
    match self {
      Self::Address(_) => host::TYPE_ADDRESS,
      Self::AgentId(_) => host::TYPE_AGENT_ID,
      Self::Bytes(_) => host::TYPE_BYTES,
      Self::ChainId(_) => host::TYPE_CHAIN_ID,
      Self::Color(_) => host::TYPE_COLOR,
      Self::Hash(_) => host::TYPE_HASH,
      Self::Hname(_) => host::TYPE_HNAME,
      Self::Int64(_) => host::TYPE_INT64,
      Self::RequestId(_) => host::TYPE_REQUEST_ID,
      Self::String(_) => host::TYPE_STRING,
    }
  }
}

impl Debug for ScValue {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    match self {
      Self::Address(inner) => inner.to_string().fmt(f),
      Self::AgentId(inner) => inner.to_string().fmt(f),
      Self::Bytes(inner) => inner.fmt(f),
      Self::ChainId(inner) => inner.to_string().fmt(f),
      Self::Color(inner) => inner.to_string().fmt(f),
      Self::Hash(inner) => inner.to_string().fmt(f),
      Self::Hname(inner) => inner.to_string().fmt(f),
      Self::Int64(inner) => inner.fmt(f),
      Self::RequestId(inner) => inner.to_string().fmt(f),
      Self::String(inner) => inner.fmt(f),
    }
  }
}

impl From<ScAddress> for ScValue {
  fn from(other: ScAddress) -> Self {
    Self::Address(other)
  }
}

impl From<ScAgentId> for ScValue {
  fn from(other: ScAgentId) -> Self {
    Self::AgentId(other)
  }
}

impl From<ScBytes> for ScValue {
  fn from(other: ScBytes) -> Self {
    Self::Bytes(other)
  }
}

impl From<ScChainId> for ScValue {
  fn from(other: ScChainId) -> Self {
    Self::ChainId(other)
  }
}

impl From<ScColor> for ScValue {
  fn from(other: ScColor) -> Self {
    Self::Color(other)
  }
}

impl From<ScHash> for ScValue {
  fn from(other: ScHash) -> Self {
    Self::Hash(other)
  }
}

impl From<ScHname> for ScValue {
  fn from(other: ScHname) -> Self {
    Self::Hname(other)
  }
}

impl From<ScInt64> for ScValue {
  fn from(other: ScInt64) -> Self {
    Self::Int64(other)
  }
}

impl From<ScRequestId> for ScValue {
  fn from(other: ScRequestId) -> Self {
    Self::RequestId(other)
  }
}

impl From<ScString> for ScValue {
  fn from(other: ScString) -> Self {
    Self::String(other)
  }
}
