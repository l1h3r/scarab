use wasmlib::*;

use crate::types::ScBytes;
use crate::types::ScInt64;
use crate::types::ScString;

/// A common interface for retrieving values from a map.
pub trait MapGet<Map: ?Sized> {
  /// Returns the value specified by `key`.
  fn map_get<Key: MapKey + ?Sized>(map: &Map, key: &Key) -> Self;
}

/// A common interface for values that can be applied to maps.
pub trait MapSet<Map: ?Sized> {
  /// Sets the value of `key` in `map`.
  fn map_set<Key: MapKey + ?Sized>(&self, map: &Map, key: &Key);
}

macro_rules! impl_MapValue {
  (@get, $map:ident, $ident:ty, $fn:ident) => {
    impl MapGet<$map> for $ident {
      fn map_get<Key: MapKey + ?Sized>(map: &$map, key: &Key) -> Self {
        map.$fn(key)
      }
    }
  };
  (@set, $map:ident, ScHname, $fn:ident) => {
    impl MapSet<$map> for ScHname {
      fn map_set<Key: MapKey + ?Sized>(&self, map: &$map, key: &Key) {
        map.$fn(key).set_value(self.clone());
      }
    }
  };
  (@set, $map:ident, ScInt64, $fn:ident) => {
    impl MapSet<$map> for ScInt64 {
      fn map_set<Key: MapKey + ?Sized>(&self, map: &$map, key: &Key) {
        map.$fn(key).set_value(*self);
      }
    }
  };
  (@set, $map:ident, $ident:ty, $fn:ident) => {
    impl MapSet<$map> for $ident {
      fn map_set<Key: MapKey + ?Sized>(&self, map: &$map, key: &Key) {
        map.$fn(key).set_value(self);
      }
    }
  };
  ($ident:ident, $fn:ident) => {
    paste::paste! {
      impl_MapValue!(@get, ScImmutableMap, [<ScImmutable $ident>], $fn);
      impl_MapValue!(@get, ScImmutableMap, [<ScImmutable $ident Array>], [<$fn _array>]);

      impl_MapValue!(@get, ScMutableMap, [<ScMutable $ident>], $fn);
      impl_MapValue!(@get, ScMutableMap, [<ScMutable $ident Array>], [<$fn _array>]);

      impl_MapValue!(@set, ScMutableMap, [<Sc $ident>], $fn);
    }
  };
  ($(($ident:ident, $fn:ident),)+) => {
    $(
      impl_MapValue!($ident, $fn);
    )+
  };
}

impl_MapValue! {
  (Address, get_address),
  (AgentId, get_agent_id),
  (Bytes, get_bytes),
  (ChainId, get_chain_id),
  (Color, get_color),
  (Hash, get_hash),
  (Hname, get_hname),
  (Int64, get_int64),
  (RequestId, get_request_id),
  (String, get_string),
}

impl_MapValue!(@set, ScMutableMap, [u8], get_bytes);
impl_MapValue!(@set, ScMutableMap, str, get_string);

// Allow retrieving maps from maps.
impl MapGet<ScImmutableMap> for ScImmutableMap {
  fn map_get<Key: MapKey + ?Sized>(map: &ScImmutableMap, key: &Key) -> Self {
    map.get_map(key)
  }
}

// Allow retrieving maps from maps.
impl MapGet<ScMutableMap> for ScMutableMap {
  fn map_get<Key: MapKey + ?Sized>(map: &ScMutableMap, key: &Key) -> Self {
    map.get_map(key)
  }
}

// Allow retrieving immutable values from mutable maps
impl<T> MapGet<ScMutableMap> for T
where
  T: MapGet<ScImmutableMap>,
{
  fn map_get<Key: MapKey + ?Sized>(map: &ScMutableMap, key: &Key) -> Self {
    Self::map_get(&map.immutable(), key)
  }
}
