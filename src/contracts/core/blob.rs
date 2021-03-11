use core::convert::TryInto;
use wasmlib::MapKey;
use wasmlib::ScFuncContext;
use wasmlib::ScHash;
use wasmlib::ScImmutableBytes;
use wasmlib::ScImmutableMap;
use wasmlib::ScMutableMap;
use wasmlib::ScViewContext;
use wasmlib::CORE_BLOB;
use wasmlib::CORE_BLOB_FUNC_STORE_BLOB;
use wasmlib::CORE_BLOB_PARAM_BYTES;
use wasmlib::CORE_BLOB_PARAM_FIELD;
use wasmlib::CORE_BLOB_PARAM_HASH;
use wasmlib::CORE_BLOB_VIEW_GET_BLOB_FIELD;
use wasmlib::CORE_BLOB_VIEW_GET_BLOB_INFO;
use wasmlib::CORE_BLOB_VIEW_LIST_BLOBS;

use crate::contracts::core::Contract;
use crate::traits::MapExt;

/// A simple wrapper around the core [`blob`][SPEC] contract.
///
/// [SPEC]: https://github.com/iotaledger/wasp/blob/master/docs/tutorial/blob.md
#[derive(Clone, Copy, Debug)]
pub struct Blob;

impl Blob {
  /// Returns a map of `hash => blobsize` for all blobs in the registry.
  pub fn list(ctx: &ScViewContext) -> BlobSizes {
    ctx.call(CORE_BLOB, CORE_BLOB_VIEW_LIST_BLOBS, None).into()
  }

  pub fn store(ctx: &ScFuncContext, blob: ScMutableMap) -> ScHash {
    ctx
      .call(CORE_BLOB, CORE_BLOB_FUNC_STORE_BLOB, blob.into(), None)
      .get_value(CORE_BLOB_PARAM_HASH)
  }

  /// Retrieves the data chunk of the specified blob field.
  pub fn field(ctx: &ScViewContext, hash: &ScHash, field: &[u8]) -> Vec<u8> {
    let params: ScMutableMap = map! {
      CORE_BLOB_PARAM_HASH => hash,
      CORE_BLOB_PARAM_FIELD => &field.to_vec(),
    };

    ctx
      .call(CORE_BLOB, CORE_BLOB_VIEW_GET_BLOB_FIELD, params.into())
      .get_value(CORE_BLOB_PARAM_BYTES)
  }

  /// Returns a map of `field => chunksize` for all fields in the specified blob.
  pub fn info(ctx: &ScViewContext, hash: &ScHash) -> FieldSizes {
    let params: ScMutableMap = map! {
      CORE_BLOB_PARAM_HASH => hash,
    };

    ctx.call(CORE_BLOB, CORE_BLOB_VIEW_GET_BLOB_INFO, params.into()).into()
  }
}

impl Contract for Blob {
  const NAME: &'static str = "blob";
  const DESC: &'static str = "Blob Contract";
}

// =============================================================================
// =============================================================================

pub struct BlobSizes(ScImmutableMap);

impl BlobSizes {
  pub fn get(&self, key: &ScHash) -> Option<u32> {
    decode_size(self.0.get(key))
  }
}

impl From<ScImmutableMap> for BlobSizes {
  fn from(other: ScImmutableMap) -> Self {
    Self(other)
  }
}

// =============================================================================
// =============================================================================

pub struct FieldSizes(ScImmutableMap);

impl FieldSizes {
  pub fn get<T>(&self, key: &T) -> Option<u32>
  where
    T: MapKey + ?Sized,
  {
    decode_size(self.0.get(key))
  }
}

impl From<ScImmutableMap> for FieldSizes {
  fn from(other: ScImmutableMap) -> Self {
    Self(other)
  }
}

// =============================================================================
// =============================================================================

fn decode_size(data: ScImmutableBytes) -> Option<u32> {
  if data.exists() {
    decode_u32(data.value())
  } else {
    None
  }
}

fn decode_u32(data: Vec<u8>) -> Option<u32> {
  data.try_into().ok().map(u32::from_le_bytes)
}
