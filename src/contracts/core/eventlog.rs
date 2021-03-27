use core::num::NonZeroI64;
use wasmlib::ScHname;
use wasmlib::ScImmutableBytesArray;
use wasmlib::ScMutableMap;
use wasmlib::ScViewContext;
use wasmlib::CORE_EVENTLOG;
use wasmlib::CORE_EVENTLOG_PARAM_CONTRACT_HNAME;
use wasmlib::CORE_EVENTLOG_PARAM_FROM_TS;
use wasmlib::CORE_EVENTLOG_PARAM_MAX_LAST_RECORDS;
use wasmlib::CORE_EVENTLOG_PARAM_TO_TS;
use wasmlib::CORE_EVENTLOG_VIEW_GET_NUM_RECORDS;
use wasmlib::CORE_EVENTLOG_VIEW_GET_RECORDS;

use crate::consts::*;
use crate::contracts::core::Contract;
use crate::traits::ContainerArray;
use crate::traits::MapExt;
use crate::types::ScBytes;

/// A simple wrapper around the core [eventlog][SPEC] contract.
///
/// [SPEC]: https://github.com/iotaledger/wasp/blob/master/docs/tutorial/eventlog.md
#[derive(Clone, PartialEq)]
pub struct EventLog(ScHname);

impl EventLog {
  /// Creates a new `EventLog` for the specified `contract`.
  pub fn new(contract: &str) -> Self {
    Self::from_hname(ScHname::new(contract))
  }

  /// Creates a new `EventLog` for the contract specified by `hname`.
  pub const fn from_hname(hname: ScHname) -> Self {
    Self(hname)
  }

  /// Returns the total number of events recorded by the smart contract
  pub fn count(&self, ctx: &ScViewContext) -> i64 {
    ctx
      .call(CORE_EVENTLOG, CORE_EVENTLOG_VIEW_GET_NUM_RECORDS, self.params().into())
      .get_value(CORE_EVENTLOG_PARAM_NUM_RECORDS)
  }

  /// Returns a list of events matching the `filter` conditions.
  pub fn search(&self, ctx: &ScViewContext, filter: EventFilter) -> Vec<ScBytes> {
    let params: ScMutableMap = self.filter(filter);

    ctx
      .call(CORE_EVENTLOG, CORE_EVENTLOG_VIEW_GET_RECORDS, params.into())
      .get::<_, ScImmutableBytesArray>(CORE_EVENTLOG_PARAM_RECORDS)
      .to_vec()
  }

  fn params(&self) -> ScMutableMap {
    map!(CORE_EVENTLOG_PARAM_CONTRACT_HNAME => &self.0)
  }

  fn filter(&self, filter: EventFilter) -> ScMutableMap {
    let params: ScMutableMap = self.params();

    if let Some(value) = filter.count.map(NonZeroI64::get) {
      params.set(CORE_EVENTLOG_PARAM_MAX_LAST_RECORDS, &value);
    }

    if let Some(value) = filter.ttime.map(NonZeroI64::get) {
      params.set(CORE_EVENTLOG_PARAM_TO_TS, &value);
    }

    if let Some(value) = filter.ftime.map(NonZeroI64::get) {
      params.set(CORE_EVENTLOG_PARAM_FROM_TS, &value);
    }

    params
  }
}

impl Contract for EventLog {
  const NAME: &'static str = "eventlog";
  const DESC: &'static str = "Event log Contract";
}

// =============================================================================
// =============================================================================

/// A set of conditions used for event filtering.
#[derive(Clone, Copy, Debug, Default)]
pub struct EventFilter {
  ftime: Option<NonZeroI64>,
  ttime: Option<NonZeroI64>,
  count: Option<NonZeroI64>,
}

impl EventFilter {
  /// Creates a new `EventFilter`.
  pub const fn new() -> Self {
    Self {
      ftime: None,
      ttime: None,
      count: None,
    }
  }

  /// Sets the `from` timestamp (default: 0).
  pub const fn from(self, value: i64) -> Self {
    Self {
      ftime: NonZeroI64::new(value),
      ..self
    }
  }

  /// Sets the `to` timestamp (default: now).
  pub const fn to(self, value: i64) -> Self {
    Self {
      ttime: NonZeroI64::new(value),
      ..self
    }
  }

  /// Sets the maximum number of records to return (default: 50).
  pub const fn count(self, value: i64) -> Self {
    Self {
      count: NonZeroI64::new(value),
      ..self
    }
  }
}
