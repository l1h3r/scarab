#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum ScTag {
  Address = 0,
  AgentId,
  Bytes,
  ChainId,
  Color,
  Hash,
  Hname,
  Int64,
  RequestId,
  String,
}

impl ScTag {
  pub fn from_u8(value: u8) -> Option<Self> {
    match value {
      0 => Some(Self::Address),
      1 => Some(Self::AgentId),
      2 => Some(Self::Bytes),
      3 => Some(Self::ChainId),
      4 => Some(Self::Color),
      5 => Some(Self::Hash),
      6 => Some(Self::Hname),
      7 => Some(Self::Int64),
      8 => Some(Self::RequestId),
      9 => Some(Self::String),
      _ => None,
    }
  }
}
