use wasmlib::ScAgentId;
use wasmlib::ScBaseContext;
use wasmlib::ScColor;
use wasmlib::ScFuncContext;
use wasmlib::ScImmutableMap;
use wasmlib::ScMutableMap;
use wasmlib::ScTransfers;
use wasmlib::ScViewContext;
use wasmlib::CORE_ACCOUNTS;
use wasmlib::CORE_ACCOUNTS_FUNC_DEPOSIT;
use wasmlib::CORE_ACCOUNTS_FUNC_WITHDRAW_TO_ADDRESS;
use wasmlib::CORE_ACCOUNTS_FUNC_WITHDRAW_TO_CHAIN;
use wasmlib::CORE_ACCOUNTS_PARAM_AGENT_ID;
use wasmlib::CORE_ACCOUNTS_VIEW_ACCOUNTS;
use wasmlib::CORE_ACCOUNTS_VIEW_BALANCE;
use wasmlib::CORE_ACCOUNTS_VIEW_TOTAL_ASSETS;

use crate::contracts::core::Contract;
use crate::traits::MapExt;

/// A simple wrapper around the core [`accounts`][SPEC] contract.
///
/// [SPEC]: https://github.com/iotaledger/wasp/blob/master/docs/tutorial/accounts.md
#[derive(Clone, Copy, Debug)]
pub struct Accounts;

impl Accounts {
  /// Moves `transfers` (tokens) to the on-chain account of `agent`.
  pub fn deposit(ctx: &ScFuncContext, transfers: ScTransfers, agent: &ScAgentId) -> ScImmutableMap {
    let params: ScMutableMap = map! {
      CORE_ACCOUNTS_PARAM_AGENT_ID => agent,
    };

    ctx.call(
      CORE_ACCOUNTS,
      CORE_ACCOUNTS_FUNC_DEPOSIT,
      params.into(),
      transfers.into(),
    )
  }

  /// Returns a map of the assets controller by the specified `agent`.
  pub fn balance(ctx: &ScViewContext, agent: &ScAgentId) -> Balances {
    let params: ScMutableMap = map! {
      CORE_ACCOUNTS_PARAM_AGENT_ID => agent,
    };

    ctx
      .call(CORE_ACCOUNTS, CORE_ACCOUNTS_VIEW_BALANCE, params.into())
      .into()
  }

  /// Returns a map of the total assets on the chain.
  pub fn assets(ctx: &ScViewContext) -> Balances {
    ctx.call(CORE_ACCOUNTS, CORE_ACCOUNTS_VIEW_TOTAL_ASSETS, None).into()
  }

  /// Returns a list of all non-empty accounts on the chain.
  pub fn accounts(ctx: &ScViewContext) -> ScImmutableMap {
    ctx.call(CORE_ACCOUNTS, CORE_ACCOUNTS_VIEW_ACCOUNTS, None)
  }
}

impl Contract for Accounts {
  const NAME: &'static str = "accounts";
  const DESC: &'static str = "Chain account ledger contract";
}

// =============================================================================
// =============================================================================

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Location {
  /// Sends all smart contract funds to the caller (an address)
  ///
  /// Note: Only valid if requested by an address (not a smart contract)
  Address,
  /// Sends all smart contract funds to the contract's native chain account.
  ///
  /// Note: Only valid if requested by the smart contract (not an address)
  Chain,
}

// =============================================================================
// =============================================================================

pub struct Balances(ScImmutableMap);

impl Balances {
  pub fn get(&self, color: &ScColor) -> i64 {
    self.0.get_value(color)
  }
}

impl From<ScImmutableMap> for Balances {
  fn from(other: ScImmutableMap) -> Self {
    Self(other)
  }
}
