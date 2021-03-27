#![allow(non_camel_case_types, non_snake_case)]

use wasmlib::ScAgentId;
use wasmlib::ScExports;
use wasmlib::ScFuncContext;
use wasmlib::ScViewContext;

use crate::traits::ContextExt;
use crate::Decode;
use crate::Encode;

pub type U256 = i64;

mod private {
  pub struct Private;
}

// =============================================================================
// ERC-20 Interface
// =============================================================================

pub trait IERC20 {
  /// Emitted when the allowance of a spender changes.
  fn Approval(owner: ScAgentId, spender: ScAgentId, value: U256) -> Approval {
    Approval::new(owner, spender, value)
  }

  /// Emitted when tokens are transferred, including zero value transfers.
  fn Transfer(from: ScAgentId, to: ScAgentId, value: U256) -> Transfer {
    Transfer::new(from, to, value)
  }

  /// Returns the name of the token.
  fn name(ctx: &ScViewContext) -> String;

  /// Returns the symbol of the token.
  fn symbol(ctx: &ScViewContext) -> String;

  /// Returns the number of decimals the token uses.
  fn decimals(ctx: &ScViewContext) -> u8;

  /// Returns the total token supply.
  fn totalSupply(ctx: &ScViewContext) -> U256;

  /// Returns the account balance of another account with address `owner`.
  fn balanceOf(ctx: &ScViewContext, owner: &ScAgentId) -> U256;

  /// Returns the amount which `spender` is still allowed to withdraw from `owner`.
  fn allowance(ctx: &ScViewContext, owner: &ScAgentId, spender: &ScAgentId) -> U256;

  /// Allows `spender` to withdraw from your account multiple times, up to the `value` amount.
  fn approve(ctx: &ScFuncContext, spender: &ScAgentId, value: &U256) -> bool;

  /// Transfers `value` amount of tokens to address `to`, and MUST fire the Transfer event.
  fn transfer(ctx: &ScFuncContext, to: &ScAgentId, value: &U256) -> bool;

  /// Transfers `value` amount of tokens from address `from` to address `to`, and MUST fire the Transfer event.
  fn transferFrom(ctx: &ScFuncContext, from: &ScAgentId, to: &ScAgentId, value: &U256) -> bool;
}

// =============================================================================
// ERC-20 Events
// =============================================================================

/// Emitted when the allowance of a spender changes.
#[derive(Encode, Decode)]
pub struct Approval {
  pub owner: ScAgentId,
  pub spender: ScAgentId,
  pub value: U256,
}

impl Approval {
  pub const fn new(owner: ScAgentId, spender: ScAgentId, value: U256) -> Approval {
    Self { owner, spender, value }
  }
}

/// Emitted when tokens are transferred, including zero value transfers.
#[derive(Encode, Decode)]
pub struct Transfer {
  pub from: ScAgentId,
  pub to: ScAgentId,
  pub value: U256,
}

impl Transfer {
  pub const fn new(from: ScAgentId, to: ScAgentId, value: U256) -> Transfer {
    Self { from, to, value }
  }
}

// =============================================================================
// Bridge to IOTA Smart Contract API
// =============================================================================

pub trait IERC20__Bridge: IERC20 {
  fn register(exports: &ScExports) {
    Self::export(exports, private::Private);
  }

  #[doc(hidden)]
  fn export(exports: &ScExports, _: private::Private);

  #[doc(hidden)]
  fn view_name(ctx: &ScViewContext) {
    trace!("ERC20.name [>]");

    ctx.result("name", Self::name(ctx));

    trace!("ERC20.name [<]");
  }

  #[doc(hidden)]
  fn view_symbol(ctx: &ScViewContext) {
    trace!("ERC20.symbol [>]");

    ctx.result("symbol", Self::symbol(ctx));

    trace!("ERC20.symbol [<]");
  }

  #[doc(hidden)]
  fn view_decimals(ctx: &ScViewContext) {
    trace!("ERC20.decimals [>]");

    ctx.result("decimals", Self::decimals(ctx) as i64);

    trace!("ERC20.decimals [<]");
  }

  #[doc(hidden)]
  fn view_totalSupply(ctx: &ScViewContext) {
    trace!("ERC20.totalSupply [>]");

    ctx.result("totalSupply", Self::totalSupply(ctx));

    trace!("ERC20.totalSupply [<]");
  }

  #[doc(hidden)]
  fn view_balanceOf(ctx: &ScViewContext) {
    trace!("ERC20.balanceOf [>]");

    let owner: ScAgentId = ctx.get_required_param("owner");

    ctx.result("balance", Self::balanceOf(ctx, &owner));

    trace!("ERC20.balanceOf [<]");
  }

  #[doc(hidden)]
  fn view_allowance(ctx: &ScViewContext) {
    trace!("ERC20.allowance [>]");

    let owner: ScAgentId = ctx.get_required_param("owner");
    let spender: ScAgentId = ctx.get_required_param("spender");

    ctx.result("remaining", Self::allowance(ctx, &owner, &spender));

    trace!("ERC20.allowance [<]");
  }

  #[doc(hidden)]
  fn func_approve(ctx: &ScFuncContext) {
    trace!("ERC20.approve [>]");

    let spender: ScAgentId = ctx.get_required_param("spender");
    let value: U256 = ctx.get_required_param("value");

    ctx.result("success", Self::approve(ctx, &spender, &value));

    trace!("ERC20.approve [<]");
  }

  #[doc(hidden)]
  fn func_transfer(ctx: &ScFuncContext) {
    trace!("ERC20.transfer [>]");

    let to: ScAgentId = ctx.get_required_param("to");
    let value: U256 = ctx.get_required_param("value");

    ctx.result("success", Self::transfer(ctx, &to, &value));

    trace!("ERC20.transfer [<]");
  }

  #[doc(hidden)]
  fn func_transferFrom(ctx: &ScFuncContext) {
    trace!("ERC20.transferFrom [>]");

    let from: ScAgentId = ctx.get_required_param("from");
    let to: ScAgentId = ctx.get_required_param("to");
    let value: U256 = ctx.get_required_param("value");

    ctx.result("success", Self::transferFrom(ctx, &from, &to, &value));

    trace!("ERC20.transferFrom [<]");
  }
}

impl<T: IERC20> IERC20__Bridge for T {
  fn export(exports: &ScExports, _: private::Private) {
    exports.add_view("name", T::view_name);
    exports.add_view("symbol", T::view_symbol);
    exports.add_view("decimals", T::view_decimals);
    exports.add_view("totalSupply", T::view_totalSupply);
    exports.add_view("balanceOf", T::view_balanceOf);
    exports.add_view("allowance", T::view_allowance);
    exports.add_func("approve", T::func_approve);
    exports.add_func("transfer", T::func_transfer);
    exports.add_func("transferFrom", T::func_transferFrom);
  }
}
