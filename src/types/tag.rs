use core::hint::unreachable_unchecked;
use wasmlib::host;

/// Represents any valid smart contract type id.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum ScTag {
  /// Represents a smart contract address.
  Address = host::TYPE_ADDRESS,
  /// Represents a smart contract agent id.
  AgentId = host::TYPE_AGENT_ID,
  /// Represents a smart contract byte vector.
  Bytes = host::TYPE_BYTES,
  /// Represents a smart contract chain id.
  ChainId = host::TYPE_CHAIN_ID,
  /// Represents a smart contract color.
  Color = host::TYPE_COLOR,
  /// Represents a smart contract hash.
  Hash = host::TYPE_HASH,
  /// Represents a smart contract hash name.
  Hname = host::TYPE_HNAME,
  /// Represents a smart contract integer.
  Int64 = host::TYPE_INT64,
  /// Represents a smart contract request id.
  RequestId = host::TYPE_REQUEST_ID,
  /// Represents a smart contract string.
  String = host::TYPE_STRING,
}

impl ScTag {
  const TOTAL: usize = 10;

  const TYPES: [i32; Self::TOTAL] = [
    host::TYPE_ADDRESS,
    host::TYPE_AGENT_ID,
    host::TYPE_BYTES,
    host::TYPE_CHAIN_ID,
    host::TYPE_COLOR,
    host::TYPE_HASH,
    host::TYPE_HNAME,
    host::TYPE_INT64,
    host::TYPE_REQUEST_ID,
    host::TYPE_STRING,
  ];

  const VARIANTS: [Self; Self::TOTAL] = [
    Self::Address,
    Self::AgentId,
    Self::Bytes,
    Self::ChainId,
    Self::Color,
    Self::Hash,
    Self::Hname,
    Self::Int64,
    Self::RequestId,
    Self::String,
  ];

  /// Decodes an `ScTag` from the given type id.
  pub const fn from_type_id(value: i32) -> Option<Self> {
    let mut index: usize = 0;

    while index < Self::TOTAL {
      if Self::TYPES[index] == value {
        return Some(Self::VARIANTS[index]);
      }

      index += 1;
    }

    None
  }

  /// Decodes an `ScTag` from the given type id.
  ///
  /// # Safety
  ///
  /// This must be guaranteed safe by the caller.
  pub const unsafe fn from_type_id_unchecked(value: i32) -> Self {
    match value {
      host::TYPE_ADDRESS => Self::Address,
      host::TYPE_AGENT_ID => Self::AgentId,
      host::TYPE_BYTES => Self::Bytes,
      host::TYPE_CHAIN_ID => Self::ChainId,
      host::TYPE_COLOR => Self::Color,
      host::TYPE_HASH => Self::Hash,
      host::TYPE_HNAME => Self::Hname,
      host::TYPE_INT64 => Self::Int64,
      host::TYPE_REQUEST_ID => Self::RequestId,
      host::TYPE_STRING => Self::String,
      _ => unreachable_unchecked(),
    }
  }

  /// Returns the host type id of the value.
  pub const fn type_id(&self) -> i32 {
    match self {
      Self::Address => host::TYPE_ADDRESS,
      Self::AgentId => host::TYPE_AGENT_ID,
      Self::Bytes => host::TYPE_BYTES,
      Self::ChainId => host::TYPE_CHAIN_ID,
      Self::Color => host::TYPE_COLOR,
      Self::Hash => host::TYPE_HASH,
      Self::Hname => host::TYPE_HNAME,
      Self::Int64 => host::TYPE_INT64,
      Self::RequestId => host::TYPE_REQUEST_ID,
      Self::String => host::TYPE_STRING,
    }
  }
}
