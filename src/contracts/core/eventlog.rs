use core::num::NonZeroI64;
use wasmlib::ScHname;
use wasmlib::ScImmutableBytesArray;
use wasmlib::ScMutableMap;
use wasmlib::ScViewContext;
use wasmlib::CORE_EVENTLOG;
use wasmlib::CORE_EVENTLOG_PARAM_CONTRACT_HNAME;
use wasmlib::CORE_EVENTLOG_PARAM_FROM_TS;
use wasmlib::CORE_EVENTLOG_PARAM_MAX_LAST_RECORDS;
use wasmlib::CORE_EVENTLOG_PARAM_NUM_RECORDS;
use wasmlib::CORE_EVENTLOG_PARAM_RECORDS;
use wasmlib::CORE_EVENTLOG_PARAM_TO_TS;
use wasmlib::CORE_EVENTLOG_VIEW_GET_NUM_RECORDS;
use wasmlib::CORE_EVENTLOG_VIEW_GET_RECORDS;

use crate::contracts::core::Contract;
use crate::traits::MapExt;

/// A simple wrapper around the core [`eventlog`][SPEC] contract.
///
/// [SPEC]: https://github.com/iotaledger/wasp/blob/master/docs/tutorial/eventlog.md
#[derive(Clone, PartialEq)]
pub struct EventLog(ScHname);

impl EventLog {
  /// Creates a new [`EventLog`] for the specified `contract`.
  pub fn new(contract: &str) -> Self {
    Self::from_hname(ScHname::new(contract))
  }

  /// Creates a new [`EventLog`] for the specified contract `hname`.
  pub const fn from_hname(hname: ScHname) -> Self {
    Self(hname)
  }

  /// Returns the total number of events recorded by the smart contract
  pub fn count(&self, ctx: &ScViewContext) -> i64 {
    ctx
      .call(CORE_EVENTLOG, CORE_EVENTLOG_VIEW_GET_NUM_RECORDS, self.params().into())
      .get_value(CORE_EVENTLOG_PARAM_NUM_RECORDS)
  }

  pub fn search(&self, ctx: &ScViewContext, filter: EventFilter) -> Vec<Vec<u8>> {
    let params: ScMutableMap = self.filter(filter);

    let output: ScImmutableBytesArray = ctx
      .call(CORE_EVENTLOG, CORE_EVENTLOG_VIEW_GET_RECORDS, params.into())
      .get(CORE_EVENTLOG_PARAM_RECORDS);

    (0..output.length())
      .map(|index| output.get_bytes(index).value())
      .collect()
  }

  fn params(&self) -> ScMutableMap {
    map! {
      CORE_EVENTLOG_PARAM_CONTRACT_HNAME => &self.0,
    }
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

#[derive(Clone, Copy, Debug, Default)]
pub struct EventFilter {
  ftime: Option<NonZeroI64>,
  ttime: Option<NonZeroI64>,
  count: Option<NonZeroI64>,
}

impl EventFilter {
  pub const fn new() -> Self {
    Self {
      ftime: None,
      ttime: None,
      count: None,
    }
  }

  pub const fn from(self, value: i64) -> Self {
    Self {
      ftime: NonZeroI64::new(value),
      ..self
    }
  }

  pub const fn to(self, value: i64) -> Self {
    Self {
      ttime: NonZeroI64::new(value),
      ..self
    }
  }

  pub const fn count(self, value: i64) -> Self {
    Self {
      count: NonZeroI64::new(value),
      ..self
    }
  }
}
