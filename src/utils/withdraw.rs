use wasmlib::ScAgentId;
use wasmlib::ScBalances;
use wasmlib::ScBaseContext;
use wasmlib::ScColor;
use wasmlib::ScFuncContext;
use wasmlib::ScTransfers;

use crate::traits::core::Array;
use crate::traits::extension::ColorExt;

/// Withdraw all L2 tokens of the specified `color` to the caller's L1 address.
///
/// * The caller **must** be the contract creator
/// * The caller **must** be an address
pub fn token(ctx: &ScFuncContext, color: &ScColor) {
  trace!("utils::withdraw::token({}) [>]", color.name());

  let creator: ScAgentId = contract_creator(ctx);
  let balance: i64 = ctx.balances().balance(color);
  let transfer: ScTransfers = ScTransfers::new(color, balance);

  if balance > 0 {
    ctx.transfer_to_address(&creator.address(), transfer);
  }

  trace!("utils::withdraw::token({}) [<]", color.name());
}

/// Withdraw all L2 tokens to the caller's L1 address.
///
/// * The caller **must** be the contract creator
/// * The caller **must** be an address
pub fn tokens(ctx: &ScFuncContext) {
  trace!("utils::withdraw::tokens [>]");

  let creator: ScAgentId = contract_creator(ctx);
  let balances: ScBalances = ctx.balances();

  for color in balances.colors().into_iter() {
    let balance: i64 = balances.balance(&color);
    let transfer: ScTransfers = ScTransfers::new(&color, balance);

    if balance > 0 {
      ctx.transfer_to_address(&creator.address(), transfer);
    }
  }

  trace!("utils::withdraw::tokens [<]");
}

fn contract_creator(ctx: &ScFuncContext) -> ScAgentId {
  let caller: ScAgentId = ctx.caller();
  let creator: ScAgentId = ctx.contract_creator();

  ctx.require(creator == caller, "withdraw: unauthorized");
  ctx.require(creator.is_address(), "withdraw: bad address");

  creator
}
