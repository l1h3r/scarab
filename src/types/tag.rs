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
  pub const fn type_id(self) -> i32 {
    self as i32
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_from_type_id() {
    assert_eq!(ScTag::from_type_id(host::TYPE_ADDRESS), Some(ScTag::Address));
    assert_eq!(ScTag::from_type_id(host::TYPE_AGENT_ID), Some(ScTag::AgentId));
    assert_eq!(ScTag::from_type_id(host::TYPE_BYTES), Some(ScTag::Bytes));
    assert_eq!(ScTag::from_type_id(host::TYPE_CHAIN_ID), Some(ScTag::ChainId));
    assert_eq!(ScTag::from_type_id(host::TYPE_COLOR), Some(ScTag::Color));
    assert_eq!(ScTag::from_type_id(host::TYPE_HASH), Some(ScTag::Hash));
    assert_eq!(ScTag::from_type_id(host::TYPE_HNAME), Some(ScTag::Hname));
    assert_eq!(ScTag::from_type_id(host::TYPE_INT64), Some(ScTag::Int64));
    assert_eq!(ScTag::from_type_id(host::TYPE_REQUEST_ID), Some(ScTag::RequestId));
    assert_eq!(ScTag::from_type_id(host::TYPE_STRING), Some(ScTag::String));

    assert_eq!(ScTag::from_type_id(i32::MAX), None);
    assert_eq!(ScTag::from_type_id(i32::MIN), None);
  }

  #[test]
  fn test_from_type_id_unchecked() {
    unsafe {
      assert_eq!(ScTag::from_type_id_unchecked(host::TYPE_ADDRESS), ScTag::Address);
      assert_eq!(ScTag::from_type_id_unchecked(host::TYPE_AGENT_ID), ScTag::AgentId);
      assert_eq!(ScTag::from_type_id_unchecked(host::TYPE_BYTES), ScTag::Bytes);
      assert_eq!(ScTag::from_type_id_unchecked(host::TYPE_CHAIN_ID), ScTag::ChainId);
      assert_eq!(ScTag::from_type_id_unchecked(host::TYPE_COLOR), ScTag::Color);
      assert_eq!(ScTag::from_type_id_unchecked(host::TYPE_HASH), ScTag::Hash);
      assert_eq!(ScTag::from_type_id_unchecked(host::TYPE_HNAME), ScTag::Hname);
      assert_eq!(ScTag::from_type_id_unchecked(host::TYPE_INT64), ScTag::Int64);
      assert_eq!(ScTag::from_type_id_unchecked(host::TYPE_REQUEST_ID), ScTag::RequestId);
      assert_eq!(ScTag::from_type_id_unchecked(host::TYPE_STRING), ScTag::String);
    }
  }

  #[test]
  fn test_type_id() {
    assert_eq!(ScTag::Address.type_id(), host::TYPE_ADDRESS);
    assert_eq!(ScTag::AgentId.type_id(), host::TYPE_AGENT_ID);
    assert_eq!(ScTag::Bytes.type_id(), host::TYPE_BYTES);
    assert_eq!(ScTag::ChainId.type_id(), host::TYPE_CHAIN_ID);
    assert_eq!(ScTag::Color.type_id(), host::TYPE_COLOR);
    assert_eq!(ScTag::Hash.type_id(), host::TYPE_HASH);
    assert_eq!(ScTag::Hname.type_id(), host::TYPE_HNAME);
    assert_eq!(ScTag::Int64.type_id(), host::TYPE_INT64);
    assert_eq!(ScTag::RequestId.type_id(), host::TYPE_REQUEST_ID);
    assert_eq!(ScTag::String.type_id(), host::TYPE_STRING);
  }
}
