use wasmlib::BytesDecoder;
use wasmlib::BytesEncoder;
use wasmlib::ScAgentId;

use crate::traits::Encode;

/// Emitted when the allowance of a spender changes.
#[derive(Clone, PartialEq)]
pub struct Approval {
  pub owner: ScAgentId,
  pub spender: ScAgentId,
  pub value: i64,
}

impl Approval {
  pub const fn new(owner: ScAgentId, spender: ScAgentId, value: i64) -> Self {
    Self {
      owner,
      spender,
      value,
    }
  }
}

impl Encode for Approval {
  fn decode(decoder: &mut BytesDecoder) -> Self {
    Self {
      owner: decoder.agent_id(),
      spender: decoder.agent_id(),
      value: decoder.int64(),
    }
  }

  fn encode(&self, encoder: &mut BytesEncoder) {
    encoder.agent_id(&self.owner);
    encoder.agent_id(&self.spender);
    encoder.int64(self.value);
  }
}

// =============================================================================
// =============================================================================

/// Emitted when tokens are transferred, including zero value transfers.
#[derive(Clone, PartialEq)]
pub struct Transfer {
  pub from: ScAgentId,
  pub to: ScAgentId,
  pub value: i64,
}

impl Transfer {
  pub const fn new(from: ScAgentId, to: ScAgentId, value: i64) -> Self {
    Self { from, to, value }
  }
}

impl Encode for Transfer {
  fn decode(decoder: &mut BytesDecoder) -> Self {
    Self {
      from: decoder.agent_id(),
      to: decoder.agent_id(),
      value: decoder.int64(),
    }
  }

  fn encode(&self, encoder: &mut BytesEncoder) {
    encoder.agent_id(&self.from);
    encoder.agent_id(&self.to);
    encoder.int64(self.value);
  }
}
