use wasmlib::ScAgentId;
use wasmlib::ScBaseContext;
use wasmlib::ScColor;
use wasmlib::ScFuncContext;
use wasmlib::ScImmutableBytes;
use wasmlib::ScImmutableColorArray;
use wasmlib::ScImmutableMap;
use wasmlib::ScMutableMap;
use wasmlib::ScTransfers;
use wasmlib::ScViewContext;
use wasmlib::CORE_ACCOUNTS;
use wasmlib::CORE_ACCOUNTS_FUNC_DEPOSIT;
use wasmlib::CORE_ACCOUNTS_FUNC_WITHDRAW_TO_ADDRESS as CORE_ACCOUNTS_FUNC_WITHDRAW; // TODO: FIXME
use wasmlib::CORE_ACCOUNTS_PARAM_AGENT_ID;
use wasmlib::CORE_ACCOUNTS_VIEW_ACCOUNTS;
use wasmlib::CORE_ACCOUNTS_VIEW_BALANCE;
use wasmlib::CORE_ACCOUNTS_VIEW_TOTAL_ASSETS;
use wasmlib::KEY_COLOR;

use crate::contracts::core::Contract;
use crate::traits::extension::MapExt;
use crate::traits::math::ToInteger;

/// A simple wrapper around the core [accounts][SPEC] contract.
///
/// [SPEC]: https://github.com/iotaledger/wasp/blob/master/docs/tutorial/accounts.md
#[derive(Clone, Copy, Debug)]
pub struct Accounts;

impl Accounts {
  /// Moves `transfer` to the L2 (on-chain) account of the caller.
  pub fn deposit_caller(ctx: &ScFuncContext, transfer: ScTransfers) {
    ctx.call(CORE_ACCOUNTS, CORE_ACCOUNTS_FUNC_DEPOSIT, None, transfer.into());
  }

  /// Moves `transfer` to the L2 (on-chain) account of `agent`.
  pub fn deposit_account(ctx: &ScFuncContext, transfer: ScTransfers, agent: &ScAgentId) {
    ctx.call(
      CORE_ACCOUNTS,
      CORE_ACCOUNTS_FUNC_DEPOSIT,
      map!(CORE_ACCOUNTS_PARAM_AGENT_ID => agent).into(),
      transfer.into(),
    );
  }

  /// Moves `transfer` to the L1 (tangle) address of the caller.
  pub fn withdraw(ctx: &ScFuncContext, transfer: ScTransfers) {
    ctx.require(ctx.caller().is_address(), "caller must be an address");
    ctx.call(CORE_ACCOUNTS, CORE_ACCOUNTS_FUNC_WITHDRAW, None, transfer.into());
  }

  /// Returns a map of the assets controlled by the specified `agent`.
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
  //
  // TODO: Need a way to list the keys of `ScImmutableMap`
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

/// A map of account balances.
pub struct Balances(ScImmutableMap);

impl Balances {
  /// Returns the total balance of tokens matching the specified `color`.
  pub fn get(&self, color: &ScColor) -> Option<u64> {
    self.0.get::<_, ScImmutableBytes>(color).to_integer()
  }

  /// Returns an array of all token colors
  pub fn colors(&self) -> ScImmutableColorArray {
    self.0.get_color_array(&KEY_COLOR)
  }
}

impl From<ScImmutableMap> for Balances {
  fn from(other: ScImmutableMap) -> Self {
    Self(other)
  }
}
