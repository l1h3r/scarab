use wasmlib::ScAgentId;
use wasmlib::ScBaseContext;
use wasmlib::ScExports;
use wasmlib::ScFuncContext;
use wasmlib::ScMutableMap;
use wasmlib::ScViewContext;

use crate::contracts::erc20::Approval;
use crate::contracts::erc20::IERC20Burnable;
use crate::contracts::erc20::Transfer;
use crate::contracts::erc20::IERC20;
use crate::traits::ContextExt;
use crate::traits::Export;
use crate::traits::UnsafeMath;
use crate::traits::Zero;
use crate::utils::call_self;
use crate::utils::emit_event;

// =============================================================================
// ERC20 Configuration
// =============================================================================

fn __func_noop(_: &ScFuncContext) {}
fn __view_noop(_: &ScViewContext) {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Token {
  name: &'static str,
  symbol: &'static str,
  decimals: u8,
  meta: bool,
  burn: bool,
  mint: bool,
}

impl Token {
  const DECIMALS: u8 = 18;

  pub const fn new(name: &'static str, symbol: &'static str) -> Self {
    Self {
      name,
      symbol,
      decimals: Self::DECIMALS,
      meta: true,
      burn: false,
      mint: false,
    }
  }

  pub const fn decimals(self, decimals: u8) -> Self {
    Self { decimals, ..self }
  }

  pub const fn meta(self, meta: bool) -> Self {
    Self { meta, ..self }
  }

  pub const fn burn(self, burn: bool) -> Self {
    Self { burn, ..self }
  }

  pub const fn mint(self, mint: bool) -> Self {
    Self { mint, ..self }
  }
}

// =============================================================================
// Generic ERC20 Implementation
// =============================================================================

#[derive(Clone, Copy, Debug)]
pub struct ERC20<const TOKEN: Token>;

impl<const TOKEN: Token> ERC20<TOKEN> {
  /// Returns the name of the token.
  fn name(ctx: &ScViewContext) {
    ctx.trace("ERC20::name [>]");
    ctx.result("name", ctx.state().get_string("name").value());
    ctx.trace("ERC20::name [<]");
  }

  /// Returns the symbol of the token.
  fn symbol(ctx: &ScViewContext) {
    ctx.trace("ERC20::symbol [>]");
    ctx.result("symbol", ctx.state().get_string("symbol").value());
    ctx.trace("ERC20::symbol [<]");
  }

  /// Returns the number of decimals the token uses.
  fn decimals(ctx: &ScViewContext) {
    ctx.trace("ERC20::decimals [>]");
    ctx.result("decimals", ctx.state().get_int64("decimals").value());
    ctx.trace("ERC20::decimals [<]");
  }

  fn __init_meta(ctx: &ScFuncContext) {
    if TOKEN.meta {
      ctx.state().get_string("name").set_value(TOKEN.name);
      ctx.state().get_string("symbol").set_value(TOKEN.symbol);
      ctx.state().get_int64("decimals").set_value(TOKEN.decimals as i64);
    }
  }

  fn __init_fixed(ctx: &ScFuncContext) {
    ctx.trace("ERC20::init [>]");

    let owner: ScAgentId = ctx.get_required_param("owner");
    let total: i64 = ctx.get_required_param("total");

    ctx.require(total > 0, "invalid param: `total`");

    // Set token metadata
    Self::__init_meta(ctx);

    // Mint initial tokens; transfer all to `owner`
    Self::__mint(ctx, &owner, total);

    ctx.trace("ERC20::init [<]");
  }

  fn __mint(ctx: &ScFuncContext, to: &ScAgentId, value: i64) {
    ctx.require(!to.is_zero(), "ERC20: mint to the zero address");

    Self::__before_token_transfer(ctx, &ScAgentId::zero(), to, value);

    ctx.state().get_int64("supply").inc_value(value);
    ctx.state().get_map("balances").get_int64(to).inc_value(value);

    emit_event(ctx, Transfer::new(ScAgentId::zero(), to.clone(), value));
  }

  fn __before_token_transfer(ctx: &ScFuncContext, from: &ScAgentId, to: &ScAgentId, value: i64) {
    call_self(ctx, "_beforeTokenTransfer", |params| {
      params.get_agent_id("from").set_value(from);
      params.get_agent_id("to").set_value(to);
      params.get_int64("value").set_value(value);
    });
  }
}

impl<const TOKEN: Token> IERC20 for ERC20<TOKEN> {
  fn total_supply(ctx: &ScViewContext) -> i64 {
    ctx.state().get_int64("supply").value()
  }

  fn balance_of(ctx: &ScViewContext, owner: &ScAgentId) -> i64 {
    ctx.state().get_map("balances").get_int64(owner).value()
  }

  fn transfer(ctx: &ScFuncContext, to: &ScAgentId, value: i64) {
    Self::__transfer(ctx, &ctx.caller(), to, value);
  }

  fn transfer_from(ctx: &ScFuncContext, from: &ScAgentId, to: &ScAgentId, value: i64) {
    Self::__transfer(ctx, from, to, value);

    let caller: ScAgentId = ctx.caller();
    let allowance: i64 = Self::allowance(ctx.view(), from, &caller);

    ctx.require(allowance >= value, "ERC20: transfer amount exceeds allowance");

    Self::__approve(ctx, from, &caller, allowance - value);
  }

  fn approve(ctx: &ScFuncContext, spender: &ScAgentId, value: i64) {
    Self::__approve(ctx, &ctx.caller(), spender, value);
  }

  fn allowance(ctx: &ScViewContext, owner: &ScAgentId, spender: &ScAgentId) -> i64 {
    ctx
      .state()
      .get_map("allowances")
      .get_map(owner)
      .get_int64(spender)
      .value()
  }

  fn __transfer(ctx: &ScFuncContext, owner: &ScAgentId, to: &ScAgentId, value: i64) {
    ctx.require(!owner.is_zero(), "ERC20: transfer from the zero address");
    ctx.require(!to.is_zero(), "ERC20: transfer to the zero address");

    Self::__before_token_transfer(ctx, owner, to, value);

    let balances: ScMutableMap = ctx.state().get_map("balances");
    let transfer: i64 = balances.get_int64(owner).value();

    ctx.require(transfer >= value, "ERC20: transfer amount exceeds balance");

    balances.get_int64(owner).dec_value(value);
    balances.get_int64(to).inc_value(value);

    emit_event(ctx, Transfer::new(owner.clone(), to.clone(), value));
  }

  fn __approve(ctx: &ScFuncContext, owner: &ScAgentId, spender: &ScAgentId, value: i64) {
    ctx.require(!owner.is_zero(), "ERC20: approve from the zero address");
    ctx.require(!spender.is_zero(), "ERC20: approve to the zero address");

    ctx
      .state()
      .get_map("allowances")
      .get_map(owner)
      .get_int64(spender)
      .set_value(value);

    emit_event(ctx, Approval::new(owner.clone(), spender.clone(), value));
  }
}

impl<const TOKEN: Token> IERC20Burnable for ERC20<TOKEN> {
  fn burn(ctx: &ScFuncContext, from: &ScAgentId, value: i64) {
    ctx.require(!from.is_zero(), "ERC20: burn from the zero address");

    Self::__before_token_transfer(ctx, from, &ScAgentId::zero(), value);

    let balance: i64 = Self::balance_of(ctx.view(), from);

    ctx.require(balance >= value, "ERC20: burn amount exceeds balances");

    ctx.state().get_map("balances").get_int64(from).dec_value(value);
    ctx.state().get_int64("supply").dec_value(value);
  }
}

impl<const TOKEN: Token> Export for ERC20<TOKEN> {
  fn register(exports: &ScExports) {
    // Enable core ERC20 functions
    <Self as IERC20>::register(exports);

    // Enable optional ERC20 functions
    if TOKEN.meta {
      exports.add_view("name", Self::name);
      exports.add_view("symbol", Self::symbol);
      exports.add_view("decimals", Self::decimals);
    }

    // Enable `burn/burnFrom` functions
    if TOKEN.burn {
      <Self as IERC20Burnable>::register(exports);
    }

    // Enable contract initialization function
    //
    // Either fixed-supply or setup roles for minting
    if TOKEN.mint {
      todo!("Init mint API")
    } else {
      exports.add_func("init", Self::__init_fixed);
    }

    // TODO: FIXME
    exports.add_func("_beforeTokenTransfer", __func_noop);
  }
}
