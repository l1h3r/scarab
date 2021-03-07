use wasmlib::ScAgentId;
use wasmlib::ScBaseContext;
use wasmlib::ScExports;
use wasmlib::ScFuncContext;
use wasmlib::ScViewContext;

use crate::traits::ContextExt;

/// EIP-20: ERC-20 Token Standard
///
/// [Reference](https://eips.ethereum.org/EIPS/eip-20)
pub trait IERC20 {
  /// Returns the total token supply.
  fn total_supply(ctx: &ScViewContext) -> i64;

  /// Returns the account balance of another account with address `owner`.
  fn balance_of(ctx: &ScViewContext, owner: &ScAgentId) -> i64;

  /// Transfers `value` amount of tokens from address `from` to address `to`, and fires the Transfer event.
  fn transfer(ctx: &ScFuncContext, to: &ScAgentId, value: i64);

  /// Transfers `value` amount of tokens from address `from` to address `to`, and fires the Transfer event.
  fn transfer_from(ctx: &ScFuncContext, from: &ScAgentId, to: &ScAgentId, value: i64);

  /// Allows `spender` to withdraw from your account multiple times, up to the `value` amount.
  fn approve(ctx: &ScFuncContext, spender: &ScAgentId, value: i64);

  /// Returns the amount which `spender` is still allowed to withdraw from `owner`.
  fn allowance(ctx: &ScViewContext, owner: &ScAgentId, spender: &ScAgentId) -> i64;

  //
  // Registration
  //

  fn register(exports: &ScExports) {
    exports.add_view("totalSupply", Self::view_total_supply);
    exports.add_view("balanceOf", Self::view_balance_of);
    exports.add_func("transfer", Self::func_transfer);
    exports.add_func("transferFrom", Self::func_transfer_from);
    exports.add_func("approve", Self::func_approve);
    exports.add_view("allowance", Self::view_allowance);
  }

  //
  // Misc Hooks
  //

  fn __transfer(_ctx: &ScFuncContext, _owner: &ScAgentId, _recipient: &ScAgentId, _value: i64) {}

  fn __approve(_ctx: &ScFuncContext, _owner: &ScAgentId, _spender: &ScAgentId, _value: i64) {}

  //
  // IOTA SC Bridge
  //

  #[doc(hidden)]
  fn view_total_supply(ctx: &ScViewContext) {
    ctx.trace("ERC20::totalSupply [>]");

    ctx.result("supply", Self::total_supply(ctx));

    ctx.trace("ERC20::totalSupply [<]");
  }

  #[doc(hidden)]
  fn view_balance_of(ctx: &ScViewContext) {
    ctx.trace("ERC20::balanceOf [>]");

    let owner: ScAgentId = ctx.get_required_param("owner");
    let value: i64 = Self::balance_of(ctx, &owner);

    ctx.result("balance", value);
    ctx.trace("ERC20::balanceOf [<]");
  }

  #[doc(hidden)]
  fn func_transfer(ctx: &ScFuncContext) {
    ctx.trace("ERC20::transfer [>]");

    let to: ScAgentId = ctx.get_required_param("to");
    let value: i64 = ctx.get_required_param("value");

    Self::transfer(ctx, &to, value);

    ctx.trace("ERC20::transfer [<]");
  }

  #[doc(hidden)]
  fn func_transfer_from(ctx: &ScFuncContext) {
    ctx.trace("ERC20::transferFrom [>]");

    let from: ScAgentId = ctx.get_required_param("from");
    let to: ScAgentId = ctx.get_required_param("to");
    let value: i64 = ctx.get_required_param("value");

    Self::transfer_from(ctx, &from, &to, value);

    ctx.trace("ERC20::transferFrom [<]");
  }

  #[doc(hidden)]
  fn func_approve(ctx: &ScFuncContext) {
    ctx.trace("ERC20::approve [>]");

    let spender: ScAgentId = ctx.get_required_param("spender");
    let value: i64 = ctx.get_required_param("value");

    Self::approve(ctx, &spender, value);

    ctx.trace("ERC20::approve [<]");
  }

  #[doc(hidden)]
  fn view_allowance(ctx: &ScViewContext) {
    ctx.trace("ERC20::allowance [>]");

    let owner: ScAgentId = ctx.get_required_param("owner");
    let spender: ScAgentId = ctx.get_required_param("spender");
    let value: i64 = Self::allowance(ctx, &owner, &spender);

    ctx.result("allowance", value);
    ctx.trace("ERC20::allowance [<]");
  }
}

// =============================================================================
// =============================================================================

/// ERC20 extension that allows the destruction of tokens
/// by holders and approved agents.
pub trait IERC20Burnable: IERC20 {
  /// Destroys `value` amount of tokens from the account `from`.
  fn burn(ctx: &ScFuncContext, from: &ScAgentId, value: i64);

  //
  // Registration
  //

  fn register(exports: &ScExports) {
    exports.add_func("burn", Self::func_burn);
    exports.add_func("burnFrom", Self::func_burn_from);
  }

  //
  // IOTA SC Bridge
  //

  #[doc(hidden)]
  fn func_burn(ctx: &ScFuncContext) {
    ctx.trace("ERC20::burn [>]");

    let value: i64 = ctx.get_required_param("value");
    let caller: ScAgentId = ctx.caller();

    Self::burn(ctx, &caller, value);

    ctx.trace("ERC20::burn [<]");
  }

  #[doc(hidden)]
  fn func_burn_from(ctx: &ScFuncContext) {
    ctx.trace("ERC20::burnFrom [>]");

    let from: ScAgentId = ctx.get_required_param("from");
    let value: i64 = ctx.get_required_param("value");
    let caller: ScAgentId = ctx.caller();
    let allowance: i64 = Self::allowance(ctx.view(), &from, &caller);

    ctx.require(allowance >= value, "ERC20: burn amount exceeds allowance");

    Self::__approve(ctx, &from, &caller, allowance - value);
    Self::burn(ctx, &from, value);

    ctx.trace("ERC20::burnFrom [<]");
  }
}
