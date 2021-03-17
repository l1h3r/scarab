use wasmlib::*;

/// A common interface for smart contract values.
pub trait Value {
  type Primitive: ?Sized;

  type Container: Container<Value = Self>;
  type ContainerMut: ContainerMut<Value = Self>;
  type ContainerArray: ContainerArray<Value = Self::Container>;
  type ContainerArrayMut: ContainerArrayMut<Value = Self::ContainerMut>;
}

macro_rules! impl_Value {
  (
    $ident:ty,
    $primitive:ty,
    $container:ty,
    $container_mut:ty,
    $container_array:ty,
    $container_array_mut:ty
  ) => {
    impl Value for $ident {
      type Primitive = $primitive;

      type Container = $container;
      type ContainerMut = $container_mut;
      type ContainerArray = $container_array;
      type ContainerArrayMut = $container_array_mut;
    }
  };
  ($((
    $ident:ty,
    $primitive:ty,
    $container:ty,
    $container_mut:ty,
    $container_array:ty,
    $container_array_mut:ty
  ),)+) => {
    $(
      impl_Value!($ident, $primitive, $container, $container_mut, $container_array, $container_array_mut);
    )+
  };
}

impl_Value! {
  (ScAddress, [u8; 33], ScImmutableAddress, ScMutableAddress, ScImmutableAddressArray, ScMutableAddressArray),
  (ScAgentId, [u8; 37], ScImmutableAgentId, ScMutableAgentId, ScImmutableAgentIdArray, ScMutableAgentIdArray),
  (Vec<u8>, Vec<u8>, ScImmutableBytes, ScMutableBytes, ScImmutableBytesArray, ScMutableBytesArray),
  (ScChainId, [u8; 33], ScImmutableChainId, ScMutableChainId, ScImmutableChainIdArray, ScMutableChainIdArray),
  (ScColor, [u8; 32], ScImmutableColor, ScMutableColor, ScImmutableColorArray, ScMutableColorArray),
  (ScContractId, [u8; 37], ScImmutableContractId, ScMutableContractId, ScImmutableContractIdArray, ScMutableContractIdArray),
  (ScHash, [u8; 32], ScImmutableHash, ScMutableHash, ScImmutableHashArray, ScMutableHashArray),
  (ScHname, u32, ScImmutableHname, ScMutableHname, ScImmutableHnameArray, ScMutableHnameArray),
  (i64, i64, ScImmutableInt64, ScMutableInt64, ScImmutableInt64Array, ScMutableInt64Array),
  (ScRequestId, [u8; 34], ScImmutableRequestId, ScMutableRequestId, ScImmutableRequestIdArray, ScMutableRequestIdArray),
  (String, String, ScImmutableString, ScMutableString, ScImmutableStringArray, ScMutableStringArray),
}

// =============================================================================
// =============================================================================

/// A common interface for immutable value containers.
pub trait Container {
  type Value: Value;

  /// Returns `true` if the container has a non-empty value.
  fn has(&self) -> bool;

  /// Returns the inner [`value`][`Self::Value`] of the container.
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
}

impl_Container! {
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

/// A common interface for mutable value containers.
pub trait ContainerMut: Container {
  /// Sets a new inner [`value`][`Container::Value`].
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

/// A common interface for immutable arrays of value containers.
pub trait ContainerArray {
  type Value: Container;

  #[doc(hidden)]
  fn __get(&self, index: usize) -> Self::Value;

  fn len(&self) -> usize;

  fn is_empty(&self) -> bool {
    self.len() == 0
  }

  fn get(&self, index: usize) -> <Self::Value as Container>::Value {
    self.__get(index).get()
  }

  fn to_vec(&self) -> Vec<<Self::Value as Container>::Value> {
    (0..self.len()).map(|index| self.get(index)).collect()
  }
}

macro_rules! impl_ContainerArray {
  ($ident:ident, $value:ty, $fn:ident) => {
    impl ContainerArray for $ident {
      type Value = $value;

      fn __get(&self, index: usize) -> Self::Value {
        self.$fn(index as i32)
      }

      fn len(&self) -> usize {
        self.length() as usize
      }
    }
  };
  ($(($ident:ident, $value:ty, $fn:ident),)+) => {
    $(
      impl_ContainerArray!($ident, $value, $fn);
    )+
  };
}

// TODO: Support maps
impl_ContainerArray! {
  (ScImmutableAddressArray, ScImmutableAddress, get_address),
  (ScImmutableAgentIdArray, ScImmutableAgentId, get_agent_id),
  (ScImmutableBytesArray, ScImmutableBytes, get_bytes),
  (ScImmutableChainIdArray, ScImmutableChainId, get_chain_id),
  (ScImmutableColorArray, ScImmutableColor, get_color),
  (ScImmutableContractIdArray, ScImmutableContractId, get_contract_id),
  (ScImmutableHashArray, ScImmutableHash, get_hash),
  (ScImmutableHnameArray, ScImmutableHname, get_hname),
  (ScImmutableInt64Array, ScImmutableInt64, get_int64),
  (ScImmutableRequestIdArray, ScImmutableRequestId, get_request_id),
  (ScImmutableStringArray, ScImmutableString, get_string),
}

// TODO: Support maps
impl_ContainerArray! {
  (ScMutableAddressArray, ScMutableAddress, get_address),
  (ScMutableAgentIdArray, ScMutableAgentId, get_agent_id),
  (ScMutableBytesArray, ScMutableBytes, get_bytes),
  (ScMutableChainIdArray, ScMutableChainId, get_chain_id),
  (ScMutableColorArray, ScMutableColor, get_color),
  (ScMutableContractIdArray, ScMutableContractId, get_contract_id),
  (ScMutableHashArray, ScMutableHash, get_hash),
  (ScMutableHnameArray, ScMutableHname, get_hname),
  (ScMutableInt64Array, ScMutableInt64, get_int64),
  (ScMutableRequestIdArray, ScMutableRequestId, get_request_id),
  (ScMutableStringArray, ScMutableString, get_string),
}

// =============================================================================
// =============================================================================

pub trait ContainerArrayMut: ContainerArray {
  fn erase(&self);
}

macro_rules! impl_ContainerArrayMut {
  ($ident:ident) => {
    impl ContainerArrayMut for $ident {
      fn erase(&self) {
        self.clear();
      }
    }
  };
  ($($ident:ident,)+) => {
    $(
      impl_ContainerArrayMut!($ident);
    )+
  };
}

// TODO: Support maps
impl_ContainerArrayMut! {
  ScMutableAddressArray,
  ScMutableAgentIdArray,
  ScMutableBytesArray,
  ScMutableChainIdArray,
  ScMutableColorArray,
  ScMutableContractIdArray,
  ScMutableHashArray,
  ScMutableHnameArray,
  ScMutableInt64Array,
  ScMutableRequestIdArray,
  ScMutableStringArray,
}
