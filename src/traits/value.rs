use wasmlib::*;

// =============================================================================
// =============================================================================

pub trait Value {
  type Primitive;
}

macro_rules! impl_Value {
  ($ident:ty, $primitive:ty) => {
    impl Value for $ident {
      type Primitive = $primitive;
    }
  };
  ($(($ident:ty, $primitive:ty),)+) => {
    $(
      impl_Value!($ident, $primitive);
    )+
  };
}

impl_Value! {
  (ScAddress, [u8; 33]),
  (ScAgentId, [u8; 37]),
  (Vec<u8>, Vec<u8>), // no ScBytes
  (ScChainId, [u8; 33]),
  (ScColor, [u8; 32]),
  (ScContractId, [u8; 37]),
  (ScHash, [u8; 32]),
  (ScHname, u32),
  (i64, i64), // no ScInt64
  (ScRequestId, [u8; 34]),
  (String, String), // no ScString
}

// =============================================================================
// =============================================================================

pub trait Container {
  type Value: Value;

  fn has(&self) -> bool;
  fn get(&self) -> Self::Value;
}

macro_rules! impl_Container {
  ($ident:ident, $value:ty) => {
    impl Container for $ident {
      type Value = $value;

      fn has(&self) -> bool {
        self.exists()
      }

      fn get(&self) -> Self::Value {
        self.value()
      }
    }
  };
  ($(($ident:ident, $value:ty),)+) => {
    $(
      impl_Container!($ident, $value);
    )+
  };
}

impl_Container! {
  (ScImmutableAddress, ScAddress),
  (ScImmutableAgentId, ScAgentId),
  (ScImmutableBytes, Vec<u8>),
  (ScImmutableChainId, ScChainId),
  (ScImmutableColor, ScColor),
  (ScImmutableContractId, ScContractId),
  (ScImmutableHash, ScHash),
  (ScImmutableHname, ScHname),
  (ScImmutableInt64, i64),
  (ScImmutableRequestId, ScRequestId),
  (ScImmutableString, String),

  (ScMutableAddress, ScAddress),
  (ScMutableAgentId, ScAgentId),
  (ScMutableBytes, Vec<u8>),
  (ScMutableChainId, ScChainId),
  (ScMutableColor, ScColor),
  (ScMutableContractId, ScContractId),
  (ScMutableHash, ScHash),
  (ScMutableHname, ScHname),
  (ScMutableInt64, i64),
  (ScMutableRequestId, ScRequestId),
  (ScMutableString, String),
}

// =============================================================================
// =============================================================================

pub trait ContainerMut: Container {
  fn set(&self, value: &Self::Value);
}

macro_rules! impl_ContainerMut {
  ($ident:ident, $value:ty) => {
    impl ContainerMut for $ident {
      fn set(&self, value: &Self::Value) {
        self.set_value(value);
      }
    }
  };
  ($(($ident:ident, $value:ty),)+) => {
    $(
      impl_ContainerMut!($ident, $value);
    )+
  };
}

impl_ContainerMut! {
  (ScMutableAddress, ScAddress),
  (ScMutableAgentId, ScAgentId),
  (ScMutableBytes, Vec<u8>),
  (ScMutableChainId, ScChainId),
  (ScMutableColor, ScColor),
  (ScMutableContractId, ScContractId),
  (ScMutableHash, ScHash),
  // (ScMutableHname, ScHname),
  // (ScMutableInt64, i64),
  (ScMutableRequestId, ScRequestId),
  (ScMutableString, String),
}

impl ContainerMut for ScMutableHname {
  fn set(&self, value: &Self::Value) {
    self.set_value(value.clone());
  }
}

impl ContainerMut for ScMutableInt64 {
  fn set(&self, value: &Self::Value) {
    self.set_value(*value);
  }
}

// =============================================================================
// =============================================================================

pub trait MapGet<T> {
  fn mget<U>(&self, key: &U) -> T
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
  (ScImmutableMap, ScImmutableAddressArray, get_address_array),
  (ScImmutableMap, ScImmutableAgentId, get_agent_id),
  (ScImmutableMap, ScImmutableAgentIdArray, get_agent_id_array),
  (ScImmutableMap, ScImmutableBytes, get_bytes),
  (ScImmutableMap, ScImmutableBytesArray, get_bytes_array),
  (ScImmutableMap, ScImmutableChainId, get_chain_id),
  (ScImmutableMap, ScImmutableChainIdArray, get_chain_id_array),
  (ScImmutableMap, ScImmutableColor, get_color),
  (ScImmutableMap, ScImmutableColorArray, get_color_array),
  (ScImmutableMap, ScImmutableContractId, get_contract_id),
  (ScImmutableMap, ScImmutableContractIdArray, get_contract_id_array),
  (ScImmutableMap, ScImmutableHash, get_hash),
  (ScImmutableMap, ScImmutableHashArray, get_hash_array),
  (ScImmutableMap, ScImmutableHname, get_hname),
  (ScImmutableMap, ScImmutableHnameArray, get_hname_array),
  (ScImmutableMap, ScImmutableInt64, get_int64),
  (ScImmutableMap, ScImmutableInt64Array, get_int64_array),
  (ScImmutableMap, ScImmutableMap, get_map),
  (ScImmutableMap, ScImmutableMapArray, get_map_array),
  (ScImmutableMap, ScImmutableRequestId, get_request_id),
  (ScImmutableMap, ScImmutableRequestIdArray, get_request_id_array),
  (ScImmutableMap, ScImmutableString, get_string),
  (ScImmutableMap, ScImmutableStringArray, get_string_array),
}

impl_MapGet! {
  (ScMutableMap, ScMutableAddress, get_address),
  (ScMutableMap, ScMutableAddressArray, get_address_array),
  (ScMutableMap, ScMutableAgentId, get_agent_id),
  (ScMutableMap, ScMutableAgentIdArray, get_agent_id_array),
  (ScMutableMap, ScMutableBytes, get_bytes),
  (ScMutableMap, ScMutableBytesArray, get_bytes_array),
  (ScMutableMap, ScMutableChainId, get_chain_id),
  (ScMutableMap, ScMutableChainIdArray, get_chain_id_array),
  (ScMutableMap, ScMutableColor, get_color),
  (ScMutableMap, ScMutableColorArray, get_color_array),
  (ScMutableMap, ScMutableContractId, get_contract_id),
  (ScMutableMap, ScMutableContractIdArray, get_contract_id_array),
  (ScMutableMap, ScMutableHash, get_hash),
  (ScMutableMap, ScMutableHashArray, get_hash_array),
  (ScMutableMap, ScMutableHname, get_hname),
  (ScMutableMap, ScMutableHnameArray, get_hname_array),
  (ScMutableMap, ScMutableInt64, get_int64),
  (ScMutableMap, ScMutableInt64Array, get_int64_array),
  (ScMutableMap, ScMutableMap, get_map),
  (ScMutableMap, ScMutableMapArray, get_map_array),
  (ScMutableMap, ScMutableRequestId, get_request_id),
  (ScMutableMap, ScMutableRequestIdArray, get_request_id_array),
  (ScMutableMap, ScMutableString, get_string),
  (ScMutableMap, ScMutableStringArray, get_string_array),
}

// =============================================================================
// =============================================================================

pub trait MapSet {
  fn mset<T>(&self, map: &ScMutableMap, key: &T)
  where
    T: MapKey + ?Sized;
}

macro_rules! impl_MapSet {
  ($ident:ty, $fn:ident) => {
    impl MapSet for $ident {
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
  (ScContractId, get_contract_id),
  (ScHash, get_hash),
  // (ScHname, get_hname),
  // (i64, get_int64),
  (ScRequestId, get_request_id),
  (String, get_string),
  (str, get_string),
}

impl MapSet for ScHname {
  fn mset<T>(&self, map: &ScMutableMap, key: &T)
  where
    T: MapKey + ?Sized,
  {
    map.get_hname(key).set_value(self.clone());
  }
}

impl MapSet for i64 {
  fn mset<T>(&self, map: &ScMutableMap, key: &T)
  where
    T: MapKey + ?Sized,
  {
    map.get_int64(key).set_value(*self);
  }
}

impl MapSet for bool {
  fn mset<T>(&self, map: &ScMutableMap, key: &T)
  where
    T: MapKey + ?Sized,
  {
    map.get_int64(key).set_value(*self as i64);
  }
}

// =============================================================================
// =============================================================================

pub trait MapExt: Sized {
  fn get<T, U>(&self, key: &T) -> U
  where
    T: MapKey + ?Sized,
    Self: MapGet<U>,
  {
    self.mget(key)
  }
}

impl MapExt for ScImmutableMap {}

impl MapExt for ScMutableMap {}
