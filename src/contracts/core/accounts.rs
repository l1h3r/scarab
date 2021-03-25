use wasmlib::ScAgentId;
use wasmlib::ScBaseContext;
use wasmlib::ScColor;
use wasmlib::ScFuncContext;
use wasmlib::ScImmutableColorArray;
use wasmlib::ScImmutableMap;
use wasmlib::ScMutableMap;
use wasmlib::ScTransfers;
use wasmlib::ScViewContext;
use wasmlib::CORE_ACCOUNTS;
use wasmlib::CORE_ACCOUNTS_FUNC_DEPOSIT;
use wasmlib::CORE_ACCOUNTS_FUNC_WITHDRAW;
use wasmlib::CORE_ACCOUNTS_PARAM_AGENT_ID;
use wasmlib::CORE_ACCOUNTS_VIEW_ACCOUNTS;
use wasmlib::CORE_ACCOUNTS_VIEW_BALANCE;
use wasmlib::CORE_ACCOUNTS_VIEW_TOTAL_ASSETS;
use wasmlib::KEY_COLOR;

use crate::contracts::core::Contract;
use crate::traits::MapExt;

/// A simple wrapper around the core [`accounts`][SPEC] contract.
///
/// [SPEC]: https://github.com/iotaledger/wasp/blob/master/docs/tutorial/accounts.md
#[derive(Clone, Copy, Debug)]
pub struct Accounts;

impl Accounts {
  /// Moves `transfer` to the on-chain account of `agent`.
  pub fn deposit(ctx: &ScFuncContext, transfer: ScTransfers, agent: &ScAgentId) {
    let params: ScMutableMap = map! {
      CORE_ACCOUNTS_PARAM_AGENT_ID => agent,
    };

    ctx.call(
      CORE_ACCOUNTS,
      CORE_ACCOUNTS_FUNC_DEPOSIT,
      params.into(),
      transfer.into(),
    );
  }

  /// Moves `transfer` to the caller's L1 address.
  pub fn withdraw(ctx: &ScFuncContext, transfer: ScTransfers) {
    ctx.require(ctx.caller().is_address(), "caller must be an address");
    ctx.call(CORE_ACCOUNTS, CORE_ACCOUNTS_FUNC_WITHDRAW, None, transfer.into());
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

  pub fn colors(&self) -> ScImmutableColorArray {
    self.0.get_color_array(&KEY_COLOR)
  }
}

impl From<ScImmutableMap> for Balances {
  fn from(other: ScImmutableMap) -> Self {
    Self(other)
  }
}
