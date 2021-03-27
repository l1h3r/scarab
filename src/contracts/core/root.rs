use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result;
use wasmlib::ScAgentId;
use wasmlib::ScBaseContext;
use wasmlib::ScChainId;
use wasmlib::ScColor;
use wasmlib::ScFuncContext;
use wasmlib::ScHash;
use wasmlib::ScHname;
use wasmlib::ScImmutableBytes;
use wasmlib::ScImmutableMap;
use wasmlib::ScMutableMap;
use wasmlib::ScViewContext;
use wasmlib::CORE_ROOT;
use wasmlib::CORE_ROOT_FUNC_CLAIM_CHAIN_OWNERSHIP;
use wasmlib::CORE_ROOT_FUNC_DELEGATE_CHAIN_OWNERSHIP;
use wasmlib::CORE_ROOT_FUNC_GRANT_DEPLOY_PERMISSION;
use wasmlib::CORE_ROOT_FUNC_REVOKE_DEPLOY_PERMISSION;
use wasmlib::CORE_ROOT_FUNC_SET_CONTRACT_FEE;
use wasmlib::CORE_ROOT_FUNC_SET_DEFAULT_FEE;
use wasmlib::CORE_ROOT_PARAM_CHAIN_OWNER;
use wasmlib::CORE_ROOT_PARAM_DEPLOYER;
use wasmlib::CORE_ROOT_PARAM_HNAME;
use wasmlib::CORE_ROOT_PARAM_OWNER_FEE;
use wasmlib::CORE_ROOT_PARAM_VALIDATOR_FEE;
use wasmlib::CORE_ROOT_VIEW_FIND_CONTRACT;
use wasmlib::CORE_ROOT_VIEW_GET_CHAIN_INFO;
use wasmlib::CORE_ROOT_VIEW_GET_FEE_INFO;

use crate::consts::*;
use crate::contracts::core::Contract;
use crate::traits::ColorExt;
use crate::traits::Decode as _;
use crate::traits::MapExt;
use crate::traits::ToUint;
use crate::Decode;
use crate::Encode;

/// A simple wrapper around the core [root][SPEC] contract.
///
/// [SPEC]: https://github.com/iotaledger/wasp/blob/master/docs/tutorial/root.md
#[derive(Clone, Copy, Debug)]
pub struct Root;

impl Root {
  /// Returns general information about the contract chain.
  pub fn chain(ctx: &ScViewContext) -> Chain {
    ctx.call(CORE_ROOT, CORE_ROOT_VIEW_GET_CHAIN_INFO, None).into()
  }

  /// Returns a contract-specific subset of the API.
  pub fn contract(contract: &ScHname) -> ContractAPI<'_> {
    ContractAPI::new(contract)
  }

  /// Sets the default fee for the contract chain.
  ///
  /// Note: panics if `fee` is empty.
  pub fn set_default_fee(ctx: &ScFuncContext, fee: Fee) {
    if fee.is_empty() {
      return ctx.panic("invalid default fee: empty");
    }

    ctx.call(CORE_ROOT, CORE_ROOT_FUNC_SET_DEFAULT_FEE, fee.params().into(), None);
  }

  /// Deploys a new smart contract to the chain.
  ///
  /// Note: The caller must have permission to deploy contracts.
  pub fn deploy(ctx: &ScFuncContext, deploy: Deploy<'_>) {
    ctx.deploy(
      deploy.program,
      deploy.name,
      deploy.description.unwrap_or("N/A"),
      deploy.init_params,
    );
  }

  /// Offers to delegate ownership of the contract chain to the specified `owner`.
  ///
  /// Note: Ownership is not transferred until claimed by the recipient.
  pub fn offer_ownership(ctx: &ScFuncContext, owner: &ScAgentId) {
    ctx.call(
      CORE_ROOT,
      CORE_ROOT_FUNC_DELEGATE_CHAIN_OWNERSHIP,
      map!(CORE_ROOT_PARAM_CHAIN_OWNER => owner).into(),
      None,
    );
  }

  /// Claims ownership of the contract chain.
  ///
  /// Note: Ownership must be offered to the caller.
  pub fn claim_ownership(ctx: &ScFuncContext) {
    ctx.call(CORE_ROOT, CORE_ROOT_FUNC_CLAIM_CHAIN_OWNERSHIP, None, None);
  }

  /// Grants permission to deploy contracts.
  pub fn grant_deploy_permission(ctx: &ScFuncContext, deployer: &ScAgentId) {
    ctx.call(
      CORE_ROOT,
      CORE_ROOT_FUNC_GRANT_DEPLOY_PERMISSION,
      map!(CORE_ROOT_PARAM_DEPLOYER => deployer).into(),
      None,
    );
  }

  /// Revokes permission to deploy contracts.
  pub fn revoke_deploy_permission(ctx: &ScFuncContext, deployer: &ScAgentId) {
    ctx.call(
      CORE_ROOT,
      CORE_ROOT_FUNC_REVOKE_DEPLOY_PERMISSION,
      map!(CORE_ROOT_PARAM_DEPLOYER => deployer).into(),
      None,
    );
  }
}

impl Contract for Root {
  const NAME: &'static str = "root";
  const DESC: &'static str = "Root Contract";
}

// =============================================================================
// =============================================================================

/// Contract-specific functionality of the [root][Root] contract.
pub struct ContractAPI<'a> {
  contract: &'a ScHname,
}

impl<'a> ContractAPI<'a> {
  fn new(contract: &'a ScHname) -> Self {
    Self { contract }
  }

  /// Returns general information about the fees of the smart contract.
  pub fn fees(&self, ctx: &ScViewContext) -> ContractFees {
    ctx
      .call(CORE_ROOT, CORE_ROOT_VIEW_GET_FEE_INFO, self.params().into())
      .into()
  }

  /// Returns the on-chain record of the smart contract.
  pub fn record(&self, ctx: &ScViewContext) -> ContractRecord {
    let data: Vec<u8> = ctx
      .call(CORE_ROOT, CORE_ROOT_VIEW_FIND_CONTRACT, self.params().into())
      .get_value(CORE_ROOT_PARAM_DATA);

    ContractRecord::from_bytes(&data)
  }

  /// Sets the fee values for the smart contract.
  ///
  /// Note: panics if `fee` is empty.
  pub fn set_fee(&self, ctx: &ScFuncContext, fee: Fee) {
    if fee.is_empty() {
      return ctx.panic("invalid contract fee: empty");
    }

    let params: ScMutableMap = self.params();

    fee.extend(&params);
    ctx.call(CORE_ROOT, CORE_ROOT_FUNC_SET_CONTRACT_FEE, params.into(), None);
  }

  fn params(&self) -> ScMutableMap {
    map!(CORE_ROOT_PARAM_HNAME => self.contract)
  }
}

// =============================================================================
// =============================================================================

pub struct Deploy<'a> {
  program: &'a ScHash,
  name: &'a str,
  description: Option<&'a str>,
  init_params: Option<ScMutableMap>,
}

impl<'a> Deploy<'a> {
  pub fn new(program: &'a ScHash, name: &'a str) -> Self {
    Self {
      program,
      name,
      description: None,
      init_params: None,
    }
  }

  pub fn description(mut self, value: &'a str) -> Self {
    self.description = Some(value);
    self
  }

  pub fn init_params(mut self, value: ScMutableMap) -> Self {
    self.init_params = Some(value);
    self
  }
}

// =============================================================================
// =============================================================================

/// Fees applied to a contract chain.
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

#[derive(Encode, Decode)]
pub struct ContractRecord {
  program_hash: ScHash,
  description: String,
  name: String,
  owner_fee: i64,
  validator_fee: i64,
  creator: ScAgentId,
}

impl ContractRecord {
  pub const fn program_hash(&self) -> &ScHash {
    &self.program_hash
  }

  pub fn description(&self) -> &str {
    &self.description
  }

  pub fn name(&self) -> &str {
    &self.name
  }

  pub const fn owner_fee(&self) -> i64 {
    self.owner_fee
  }

  pub const fn validator_fee(&self) -> i64 {
    self.validator_fee
  }

  pub const fn creator(&self) -> &ScAgentId {
    &self.creator
  }
}

// =============================================================================
// =============================================================================

/// Fees associated with a smart contract.
pub struct ContractFees(ScImmutableMap);

impl ContractFees {
  /// Returns the color of fees used by the contract.
  pub fn color(&self) -> ScColor {
    self.0.get_value(CORE_ROOT_PARAM_FEE_COLOR)
  }

  /// Returns the fee charged by the contract owner.
  pub fn owner_fee(&self) -> Option<u64> {
    self.0.get::<_, ScImmutableBytes>(CORE_ROOT_PARAM_OWNER_FEE).to_uint()
  }

  /// Returns the fee charged by the contract validator.
  pub fn validator_fee(&self) -> Option<u64> {
    self
      .0
      .get::<_, ScImmutableBytes>(CORE_ROOT_PARAM_VALIDATOR_FEE)
      .to_uint()
  }
}

impl From<ScImmutableMap> for ContractFees {
  fn from(other: ScImmutableMap) -> Self {
    Self(other)
  }
}

impl Debug for ContractFees {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.debug_struct("ContractFees")
      .field("color", &self.color().name())
      .field("owner_fee", &self.owner_fee())
      .field("validator_fee", &self.validator_fee())
      .finish()
  }
}

// =============================================================================
// =============================================================================

/// Information regarding the configuration of the contract chain.
pub struct Chain(ScImmutableMap);

impl Chain {
  /// Returns the chain id.
  pub fn id(&self) -> ScChainId {
    self.0.get_value(CORE_ROOT_VAR_CHAIN_ID)
  }

  /// Returns the agent id of the contract chain owner.
  pub fn owner_id(&self) -> ScAgentId {
    self.0.get_value(CORE_ROOT_VAR_CHAIN_OWNER_ID)
  }

  /// Returns the decription of the contract chain.
  pub fn description(&self) -> String {
    self.0.get_value(CORE_ROOT_VAR_DESCRIPTION)
  }

  /// Returns the color of fees used by the chain.
  pub fn fee_color(&self) -> ScColor {
    self.0.get_value(CORE_ROOT_VAR_FEE_COLOR)
  }

  /// Returns the default fee charged by the contract owner.
  pub fn default_owner_fee(&self) -> i64 {
    self.0.get_value(CORE_ROOT_VAR_DEFAULT_OWNER_FEE)
  }

  /// Returns the default fee charged by the contract validator.
  pub fn default_validator_fee(&self) -> i64 {
    self.0.get_value(CORE_ROOT_VAR_DEFAULT_VALIDATOR_FEE)
  }

  /// Returns the contract registry.
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

/// A registry of on-chain contracts.
pub struct Contracts(ScImmutableMap);

impl Contracts {
  /// Returns the raw binary of the specified contract.
  pub fn get(&self, key: &ScHname) -> Vec<u8> {
    self.0.get_value(key)
  }
}

impl From<ScImmutableMap> for Contracts {
  fn from(other: ScImmutableMap) -> Self {
    Self(other)
  }
}
