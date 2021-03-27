use wasmlib::MapKey;
use wasmlib::ScBaseContext;
use wasmlib::ScFuncContext;
use wasmlib::ScHash;
use wasmlib::ScImmutableBytes;
use wasmlib::ScImmutableMap;
use wasmlib::ScMutableMap;
use wasmlib::ScViewContext;
use wasmlib::CORE_BLOB;
use wasmlib::CORE_BLOB_FUNC_STORE_BLOB;
use wasmlib::CORE_BLOB_PARAM_FIELD;
use wasmlib::CORE_BLOB_PARAM_HASH;
use wasmlib::CORE_BLOB_VIEW_GET_BLOB_FIELD;
use wasmlib::CORE_BLOB_VIEW_GET_BLOB_INFO;
use wasmlib::CORE_BLOB_VIEW_LIST_BLOBS;

use crate::consts::*;
use crate::contracts::core::Contract;
use crate::traits::MapExt;
use crate::traits::ToUint;
use crate::types::ScBytes;
use crate::types::ScString;

const WASMTIMEVM: &str = "wasmtimevm";

/// A simple wrapper around the core [blob][SPEC] contract.
///
/// [SPEC]: https://github.com/iotaledger/wasp/blob/master/docs/tutorial/blob.md
#[derive(Clone, Copy, Debug)]
pub struct Blob;

impl Blob {
  /// Returns a map of `{hash => size}` for all blobs in the registry.
  pub fn list(ctx: &ScViewContext) -> BlobSizes {
    ctx.call(CORE_BLOB, CORE_BLOB_VIEW_LIST_BLOBS, None).into()
  }

  /// Stores the given `blob` and returns a hash identifying the contents.
  pub fn store(ctx: &ScFuncContext, blob: ScMutableMap) -> ScHash {
    ctx.require(
      !blob.contains_mut::<_, ScBytes>(CORE_BLOB_FIELD_PROGRAM_BINARY),
      "reserved field: `p`",
    );

    ctx.require(
      !blob.contains_mut::<_, ScString>(CORE_BLOB_FIELD_PROGRAM_DESCRIPTION),
      "reserved field: `d`",
    );

    ctx.require(
      !blob.contains_mut::<_, ScString>(CORE_BLOB_FIELD_VM_TYPE),
      "reserved field: `v`",
    );

    Self::store_unchecked(ctx, blob)
  }

  /// Stores the given `blob` without performing validation checks.
  pub fn store_unchecked(ctx: &ScFuncContext, blob: ScMutableMap) -> ScHash {
    ctx
      .call(CORE_BLOB, CORE_BLOB_FUNC_STORE_BLOB, blob.into(), None)
      .get_value(CORE_BLOB_PARAM_HASH)
  }

  /// Retrieves the data chunk of the specified blob field.
  pub fn field(ctx: &ScViewContext, hash: &ScHash, field: &[u8]) -> ScBytes {
    let params: ScMutableMap = map! {
      CORE_BLOB_PARAM_HASH => hash,
      CORE_BLOB_PARAM_FIELD => field,
    };

    ctx
      .call(CORE_BLOB, CORE_BLOB_VIEW_GET_BLOB_FIELD, params.into())
      .get_value(CORE_BLOB_PARAM_BYTES)
  }

  /// Returns a map of `{field => size}` for all fields in the specified blob.
  pub fn info(ctx: &ScViewContext, hash: &ScHash) -> FieldSizes {
    let params: ScMutableMap = map!(CORE_BLOB_PARAM_HASH => hash);

    ctx.call(CORE_BLOB, CORE_BLOB_VIEW_GET_BLOB_INFO, params.into()).into()
  }

  /// Stores the given `binary` as a loadable WebAssembly blob.
  pub fn wasm_put(ctx: &ScFuncContext, binary: &[u8]) -> ScHash {
    let params: ScMutableMap = map! {
      CORE_BLOB_FIELD_VM_TYPE => WASMTIMEVM,
      CORE_BLOB_FIELD_PROGRAM_BINARY => binary,
    };

    Self::store_unchecked(ctx, params)
  }

  /// Returns the raw data of a stored WebAssembly binary.
  pub fn wasm_get(ctx: &ScViewContext, hash: &ScHash) -> ScBytes {
    ctx.require(Self::is_wasm(ctx, hash), "invalid wasm binary");
    Self::field(ctx, hash, CORE_BLOB_FIELD_PROGRAM_BINARY.as_bytes())
  }

  /// Returns true if the specified blob is a WebAssembly binary.
  pub fn is_wasm(ctx: &ScViewContext, hash: &ScHash) -> bool {
    Self::field(ctx, hash, CORE_BLOB_FIELD_VM_TYPE.as_bytes()) == WASMTIMEVM.as_bytes()
  }
}

impl Contract for Blob {
  const NAME: &'static str = "blob";
  const DESC: &'static str = "Blob Contract";
}

// =============================================================================
// =============================================================================

/// A map of blob sizes.
pub struct BlobSizes(ScImmutableMap);

impl BlobSizes {
  /// Returns the size of the specified blob.
  pub fn get(&self, key: &ScHash) -> Option<u32> {
    self.0.get::<_, ScImmutableBytes>(key).to_uint()
  }
}

impl From<ScImmutableMap> for BlobSizes {
  fn from(other: ScImmutableMap) -> Self {
    Self(other)
  }
}

// =============================================================================
// =============================================================================

/// A map of blob field sizes.
pub struct FieldSizes(ScImmutableMap);

impl FieldSizes {
  /// Returns the size of the specified blob field.
  pub fn get<T>(&self, key: &T) -> Option<u32>
  where
    T: MapKey + ?Sized,
  {
    self.0.get::<_, ScImmutableBytes>(key).to_uint()
  }
}

impl From<ScImmutableMap> for FieldSizes {
  fn from(other: ScImmutableMap) -> Self {
    Self(other)
  }
}
