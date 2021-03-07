use wasmlib::ScBaseContext;
use wasmlib::ScFuncContext;
use wasmlib::ScHname;
use wasmlib::ScImmutableMap;
use wasmlib::ScMutableMap;

use crate::traits::Encode;

pub fn emit_event<E>(ctx: &ScFuncContext, event: E)
where
  E: Encode,
{
  ctx.event(&ctx.utility().base58_encode(&event.to_bytes()));
}

pub fn call_self<F>(ctx: &ScFuncContext, name: &str, f: F) -> ScImmutableMap
where
  F: Fn(&ScMutableMap),
{
  let hname: ScHname = ScHname::new(name);
  let params: ScMutableMap = ScMutableMap::new();

  f(&params);

  ctx.call_self(hname, Some(params), None)
}
