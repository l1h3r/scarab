use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result;
use wasmlib::ScAddress;
use wasmlib::ScAgentId;
use wasmlib::ScBaseContext;
use wasmlib::ScChainId;
use wasmlib::ScColor;
use wasmlib::ScFuncContext;
use wasmlib::ScHname;
use wasmlib::ScImmutableMap;
use wasmlib::ScMutableMap;
use wasmlib::ScViewContext;
use wasmlib::CORE_ROOT;
use wasmlib::CORE_ROOT_FUNC_CLAIM_CHAIN_OWNERSHIP;
use wasmlib::CORE_ROOT_FUNC_DELEGATE_CHAIN_OWNERSHIP;
use wasmlib::CORE_ROOT_FUNC_DEPLOY_CONTRACT;
use wasmlib::CORE_ROOT_FUNC_GRANT_DEPLOY_PERMISSION;
use wasmlib::CORE_ROOT_FUNC_REVOKE_DEPLOY_PERMISSION;
use wasmlib::CORE_ROOT_FUNC_SET_CONTRACT_FEE;
use wasmlib::CORE_ROOT_FUNC_SET_DEFAULT_FEE;
use wasmlib::CORE_ROOT_PARAM_CHAIN_OWNER;
use wasmlib::CORE_ROOT_PARAM_DATA;
use wasmlib::CORE_ROOT_PARAM_DEPLOYER;
use wasmlib::CORE_ROOT_PARAM_DESCRIPTION;
use wasmlib::CORE_ROOT_PARAM_FEE_COLOR;
use wasmlib::CORE_ROOT_PARAM_HNAME;
use wasmlib::CORE_ROOT_PARAM_NAME;
use wasmlib::CORE_ROOT_PARAM_OWNER_FEE;
use wasmlib::CORE_ROOT_PARAM_PROGRAM_HASH;
use wasmlib::CORE_ROOT_PARAM_VALIDATOR_FEE;
use wasmlib::CORE_ROOT_VIEW_FIND_CONTRACT;
use wasmlib::CORE_ROOT_VIEW_GET_CHAIN_INFO;
use wasmlib::CORE_ROOT_VIEW_GET_FEE_INFO;

use crate::contracts::core::Contract;
use crate::traits::ColorExt;
use crate::traits::MapExt;

const CORE_ROOT_VAR_CHAIN_ID: &str = "c";
const CORE_ROOT_VAR_CHAIN_COLOR: &str = "co";
const CORE_ROOT_VAR_CHAIN_ADDRESS: &str = "ad";
const CORE_ROOT_VAR_CHAIN_OWNER_ID: &str = "o";
const CORE_ROOT_VAR_FEE_COLOR: &str = "f";
const CORE_ROOT_VAR_DEFAULT_OWNER_FEE: &str = "do";
const CORE_ROOT_VAR_DEFAULT_VALIDATOR_FEE: &str = "dv";
const CORE_ROOT_VAR_CONTRACT_REGISTRY: &str = "r";
const CORE_ROOT_VAR_DESCRIPTION: &str = "d";

/// A simple wrapper around the core [`root`][SPEC] contract.
///
/// [SPEC]: https://github.com/iotaledger/wasp/blob/master/docs/tutorial/root.md
#[derive(Clone, Copy, Debug)]
pub struct Root;

impl Root {
  /// Deploys a new smart contract to the chain.
  pub fn deploy(ctx: &ScFuncContext, config: Deploy) {
    ctx.call(CORE_ROOT, CORE_ROOT_FUNC_DEPLOY_CONTRACT, config.params().into(), None);
  }

  /// Offers to delegate ownership of the chain to the specified `owner`.
  ///
  /// Note: Ownership is not transferred until claimed by the recipient.
  pub fn offer_ownership(ctx: &ScFuncContext, owner: &ScAgentId) {
    let params: ScMutableMap = params! {
      CORE_ROOT_PARAM_CHAIN_OWNER => owner,
    };

    ctx.call(CORE_ROOT, CORE_ROOT_FUNC_DELEGATE_CHAIN_OWNERSHIP, params.into(), None);
  }

  /// Claims ownership of the chain if it was offered.
  pub fn claim_ownership(ctx: &ScFuncContext) {
    ctx.call(CORE_ROOT, CORE_ROOT_FUNC_CLAIM_CHAIN_OWNERSHIP, None, None);
  }

  /// Grants permission to deploy contracts.
  pub fn grant_deploy_permission(ctx: &ScFuncContext, deployer: &ScAgentId) {
    let params: ScMutableMap = params! {
      CORE_ROOT_PARAM_DEPLOYER => deployer,
    };

    ctx.call(CORE_ROOT, CORE_ROOT_FUNC_GRANT_DEPLOY_PERMISSION, params.into(), None);
  }

  /// Revokes permission to deploy contracts.
  pub fn revoke_deploy_permission(ctx: &ScFuncContext, deployer: &ScAgentId) {
    let params: ScMutableMap = params! {
      CORE_ROOT_PARAM_DEPLOYER => deployer,
    };

    ctx.call(CORE_ROOT, CORE_ROOT_FUNC_REVOKE_DEPLOY_PERMISSION, params.into(), None);
  }

  /// Sets the chain-wide default fee values.
  ///
  /// Note: panics if `Fee` is empty.
  pub fn set_default_fee(ctx: &ScFuncContext, fee: Fee) {
    if fee.is_empty() {
      return ctx.panic("invalid default fee: empty");
    }

    ctx.call(CORE_ROOT, CORE_ROOT_FUNC_SET_DEFAULT_FEE, fee.params().into(), None);
  }

  /// Sets the fee values for the specified smart `contract`.
  ///
  /// Note: panics if `Fee` is empty.
  pub fn set_contract_fee(ctx: &ScFuncContext, contract: &ScHname, fee: Fee) {
    if fee.is_empty() {
      return ctx.panic("invalid contract fee: empty");
    }

    let params: ScMutableMap = params! {
      CORE_ROOT_PARAM_HNAME => contract,
    };

    fee.extend(&params);
    ctx.call(CORE_ROOT, CORE_ROOT_FUNC_SET_CONTRACT_FEE, params.into(), None);
  }

  /// Returns the data of the specified smart `contract`.
  pub fn contract(ctx: &ScViewContext, contract: &ScHname) -> Vec<u8> {
    let params: ScMutableMap = params! {
      CORE_ROOT_PARAM_HNAME => contract,
    };

    ctx
      .call(CORE_ROOT, CORE_ROOT_VIEW_FIND_CONTRACT, params.into())
      .get_value(CORE_ROOT_PARAM_DATA)
  }

  /// Returns general information about the chain.
  pub fn chain(ctx: &ScViewContext) -> Chain {
    ctx.call(CORE_ROOT, CORE_ROOT_VIEW_GET_CHAIN_INFO, None).into()
  }

  /// Returns general information about the fees of the specified `contract`.
  pub fn fees(ctx: &ScViewContext, contract: &ScHname) -> Fees {
    let params: ScMutableMap = params! {
      CORE_ROOT_PARAM_HNAME => contract,
    };

    ctx.call(CORE_ROOT, CORE_ROOT_VIEW_GET_FEE_INFO, params.into()).into()
  }
}

impl Contract for Root {
  const NAME: &'static str = "root";
  const DESC: &'static str = "Root Contract";
}

// =============================================================================
// =============================================================================

#[derive(Clone, PartialEq)]
pub struct Deploy {
  contract: ScHname,
  name: String,
  description: Option<String>,
}

impl Deploy {
  pub const fn new(contract: ScHname, name: String) -> Self {
    Self {
      contract,
      name,
      description: None,
    }
  }

  pub fn description(mut self, value: String) -> Self {
    self.description = Some(value);
    self
  }

  fn params(&self) -> ScMutableMap {
    let params: ScMutableMap = params! {
      CORE_ROOT_PARAM_PROGRAM_HASH => &self.contract,
      CORE_ROOT_PARAM_NAME => &self.name,
    };

    if let Some(value) = self.description.as_ref() {
      params.set(CORE_ROOT_PARAM_DESCRIPTION, value);
    }

    params
  }
}

// =============================================================================
// =============================================================================

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub struct Fee {
  owner: Option<u32>,
  validator: Option<u32>,
}

impl Fee {
  pub const fn new() -> Self {
    Self {
      owner: None,
      validator: None,
    }
  }

  pub const fn owner(self, value: u32) -> Self {
    Self {
      owner: Some(value),
      ..self
    }
  }

  pub const fn validator(self, value: u32) -> Self {
    Self {
      validator: Some(value),
      ..self
    }
  }

  pub const fn is_empty(&self) -> bool {
    self.owner.is_none() && self.validator.is_none()
  }

  fn params(&self) -> ScMutableMap {
    let params: ScMutableMap = ScMutableMap::new();

    self.extend(&params);

    params
  }

  fn extend(&self, params: &ScMutableMap) {
    if let Some(value) = self.owner.map(i64::from) {
      params.set(CORE_ROOT_PARAM_OWNER_FEE, &value);
    }

    if let Some(value) = self.validator.map(i64::from) {
      params.set(CORE_ROOT_PARAM_OWNER_FEE, &value);
    }
  }
}

// =============================================================================
// =============================================================================

pub struct Fees(ScImmutableMap);

impl Fees {
  pub fn color(&self) -> ScColor {
    self.0.get_value(CORE_ROOT_PARAM_FEE_COLOR)
  }

  pub fn owner_fee(&self) -> i64 {
    self.0.get_value(CORE_ROOT_PARAM_OWNER_FEE)
  }

  pub fn validator_fee(&self) -> i64 {
    self.0.get_value(CORE_ROOT_PARAM_VALIDATOR_FEE)
  }
}

impl From<ScImmutableMap> for Fees {
  fn from(other: ScImmutableMap) -> Self {
    Self(other)
  }
}

impl Debug for Fees {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.debug_struct("Fees")
      .field("color", &self.color().name())
      .field("owner_fee", &self.owner_fee())
      .field("validator_fee", &self.validator_fee())
      .finish()
  }
}

// =============================================================================
// =============================================================================

pub struct Chain(ScImmutableMap);

impl Chain {
  pub fn id(&self) -> ScChainId {
    self.0.get_value(CORE_ROOT_VAR_CHAIN_ID)
  }

  pub fn owner_id(&self) -> ScAgentId {
    self.0.get_value(CORE_ROOT_VAR_CHAIN_OWNER_ID)
  }

  pub fn color(&self) -> ScColor {
    self.0.get_value(CORE_ROOT_VAR_CHAIN_COLOR)
  }

  pub fn address(&self) -> ScAddress {
    self.0.get_value(CORE_ROOT_VAR_CHAIN_ADDRESS)
  }

  pub fn description(&self) -> String {
    self.0.get_value(CORE_ROOT_VAR_DESCRIPTION)
  }

  pub fn fee_color(&self) -> ScColor {
    self.0.get_value(CORE_ROOT_VAR_FEE_COLOR)
  }

  pub fn default_owner_fee(&self) -> i64 {
    self.0.get_value(CORE_ROOT_VAR_DEFAULT_OWNER_FEE)
  }

  pub fn default_validator_fee(&self) -> i64 {
    self.0.get_value(CORE_ROOT_VAR_DEFAULT_VALIDATOR_FEE)
  }

  pub fn contracts(&self) -> Contracts {
    self.0.get::<_, ScImmutableMap>(CORE_ROOT_VAR_CONTRACT_REGISTRY).into()
  }
}

impl From<ScImmutableMap> for Chain {
  fn from(other: ScImmutableMap) -> Self {
    Self(other)
  }
}

impl Debug for Chain {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.debug_struct("Chain")
      .field("id", &self.id().to_string())
      .field("owner_id", &self.owner_id().to_string())
      .field("color", &self.color().name())
      .field("address", &self.address().to_string())
      .field("description", &self.description())
      .field("fee_color", &self.fee_color().name())
      .field("default_owner_fee", &self.default_owner_fee())
      .field("default_validator_fee", &self.default_validator_fee())
      .field("contracts", &"...")
      .finish()
  }
}

// =============================================================================
// =============================================================================

pub struct Contracts(ScImmutableMap);

impl Contracts {
  pub fn get(&self, key: &ScHname) -> Vec<u8> {
    self.0.get_value(key)
  }
}

impl From<ScImmutableMap> for Contracts {
  fn from(other: ScImmutableMap) -> Self {
    Self(other)
  }
}
