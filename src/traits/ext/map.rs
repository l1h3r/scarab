use wasmlib::*;

use crate::traits::Container;
use crate::traits::Value;

/// A common interface for retrieving [values][Value] from a map.
pub trait MapGet<Value> {
  /// Returns the value specified by `key`.
  fn mget<U>(&self, key: &U) -> Value
  where
    U: MapKey + ?Sized;
}

macro_rules! impl_MapGet {
  ($map:ident, $ident:ident, $fn:ident) => {
    impl MapGet<$ident> for $map {
      fn mget<U>(&self, key: &U) -> $ident
      where
        U: MapKey + ?Sized
      {
        self.$fn(key)
      }
    }
  };
  ($(($map:ident, $ident:ident, $fn:ident),)+) => {
    $(
      impl_MapGet!($map, $ident, $fn);
    )+
  };
}

impl_MapGet! {
  (ScImmutableMap, ScImmutableAddress, get_address),
  (ScImmutableMap, ScImmutableAgentId, get_agent_id),
  (ScImmutableMap, ScImmutableBytes, get_bytes),
  (ScImmutableMap, ScImmutableChainId, get_chain_id),
  (ScImmutableMap, ScImmutableColor, get_color),
  (ScImmutableMap, ScImmutableHash, get_hash),
  (ScImmutableMap, ScImmutableHname, get_hname),
  (ScImmutableMap, ScImmutableInt64, get_int64),
  (ScImmutableMap, ScImmutableMap, get_map),
  (ScImmutableMap, ScImmutableRequestId, get_request_id),
  (ScImmutableMap, ScImmutableString, get_string),

  (ScImmutableMap, ScImmutableAddressArray, get_address_array),
  (ScImmutableMap, ScImmutableAgentIdArray, get_agent_id_array),
  (ScImmutableMap, ScImmutableBytesArray, get_bytes_array),
  (ScImmutableMap, ScImmutableChainIdArray, get_chain_id_array),
  (ScImmutableMap, ScImmutableColorArray, get_color_array),
  (ScImmutableMap, ScImmutableHashArray, get_hash_array),
  (ScImmutableMap, ScImmutableHnameArray, get_hname_array),
  (ScImmutableMap, ScImmutableInt64Array, get_int64_array),
  (ScImmutableMap, ScImmutableMapArray, get_map_array),
  (ScImmutableMap, ScImmutableRequestIdArray, get_request_id_array),
  (ScImmutableMap, ScImmutableStringArray, get_string_array),
}

impl_MapGet! {
  (ScMutableMap, ScMutableAddress, get_address),
  (ScMutableMap, ScMutableAgentId, get_agent_id),
  (ScMutableMap, ScMutableBytes, get_bytes),
  (ScMutableMap, ScMutableChainId, get_chain_id),
  (ScMutableMap, ScMutableColor, get_color),
  (ScMutableMap, ScMutableHash, get_hash),
  (ScMutableMap, ScMutableHname, get_hname),
  (ScMutableMap, ScMutableInt64, get_int64),
  (ScMutableMap, ScMutableMap, get_map),
  (ScMutableMap, ScMutableRequestId, get_request_id),
  (ScMutableMap, ScMutableString, get_string),

  (ScMutableMap, ScMutableAddressArray, get_address_array),
  (ScMutableMap, ScMutableAgentIdArray, get_agent_id_array),
  (ScMutableMap, ScMutableBytesArray, get_bytes_array),
  (ScMutableMap, ScMutableChainIdArray, get_chain_id_array),
  (ScMutableMap, ScMutableColorArray, get_color_array),
  (ScMutableMap, ScMutableHashArray, get_hash_array),
  (ScMutableMap, ScMutableHnameArray, get_hname_array),
  (ScMutableMap, ScMutableInt64Array, get_int64_array),
  (ScMutableMap, ScMutableMapArray, get_map_array),
  (ScMutableMap, ScMutableRequestIdArray, get_request_id_array),
  (ScMutableMap, ScMutableStringArray, get_string_array),
}

impl<T> MapGet<T> for ScMutableMap
where
  ScImmutableMap: MapGet<T>,
{
  fn mget<U>(&self, key: &U) -> T
  where
    U: MapKey + ?Sized,
  {
    self.immutable().mget(key)
  }
}

// =============================================================================
// =============================================================================

/// A common interface for values that can be applied to maps.
pub trait MapSet<Map> {
  /// Sets the value of `key` in `map`.
  fn mset<T>(&self, map: &Map, key: &T)
  where
    T: MapKey + ?Sized;
}

macro_rules! impl_MapSet {
  ($ident:ty, $fn:ident) => {
    impl MapSet<ScMutableMap> for $ident {
      fn mset<T>(&self, map: &ScMutableMap, key: &T)
      where
        T: MapKey + ?Sized,
      {
        map.$fn(key).set_value(self);
      }
    }
  };
  ($(($ident:ty, $fn:ident),)+) => {
    $(
      impl_MapSet!($ident, $fn);
    )+
  };
}

impl_MapSet! {
  (ScAddress, get_address),
  (ScAgentId, get_agent_id),
  (Vec<u8>, get_bytes),
  ([u8], get_bytes),
  (ScChainId, get_chain_id),
  (ScColor, get_color),
  (ScHash, get_hash),
  // (ScHname, get_hname),
  // (i64, get_int64),
  (ScRequestId, get_request_id),
  (String, get_string),
  (str, get_string),
}

impl MapSet<ScMutableMap> for ScHname {
  fn mset<T>(&self, map: &ScMutableMap, key: &T)
  where
    T: MapKey + ?Sized,
  {
    map.get_hname(key).set_value(self.clone());
  }
}

impl MapSet<ScMutableMap> for i64 {
  fn mset<T>(&self, map: &ScMutableMap, key: &T)
  where
    T: MapKey + ?Sized,
  {
    map.get_int64(key).set_value(*self);
  }
}

impl MapSet<ScMutableMap> for bool {
  fn mset<T>(&self, map: &ScMutableMap, key: &T)
  where
    T: MapKey + ?Sized,
  {
    map.get_int64(key).set_value(*self as i64);
  }
}

// =============================================================================
// =============================================================================

/// Extensions for [ScImmutableMap] and [ScMutableMap].
pub trait MapExt: Sized {
  /// Returns the container value specified by `key`.
  fn get<T, U>(&self, key: &T) -> U
  where
    T: MapKey + ?Sized,
    Self: MapGet<U>,
  {
    self.mget(key)
  }

  /// Returns the value specified by `key`.
  fn get_value<T, U>(&self, key: &T) -> U
  where
    T: MapKey + ?Sized,
    U: Value,
    Self: MapGet<U::Container>,
  {
    self.mget(key).get()
  }

  /// Sets the `value` of `key`.
  fn set<T, U>(&self, key: &T, value: &U)
  where
    T: MapKey + ?Sized,
    U: MapSet<Self> + ?Sized,
  {
    value.mset(self, key);
  }

  /// Returns `true` if self contains `key`.
  fn contains<T, U>(&self, key: &T) -> bool
  where
    T: MapKey + ?Sized,
    U: Value,
    Self: MapGet<U::Container>,
  {
    self.mget(key).has()
  }

  /// Returns `true` if self contains `key`.
  fn contains_mut<T, U>(&self, key: &T) -> bool
  where
    T: MapKey + ?Sized,
    U: Value,
    Self: MapGet<U::ContainerMut>,
  {
    self.mget(key).has()
  }
}

impl MapExt for ScImmutableMap {}

impl MapExt for ScMutableMap {}
