use wasmlib::*;

type ScBytes = Vec<u8>;
type ScInt64 = i64;
type ScString = String;

/// A common interface for smart contract values.
pub trait Value {
  type Primitive: ?Sized;

  type Container: Container<Value = Self>;
  type ContainerMut: ContainerMut<Value = Self>;
  type ContainerArray: ContainerArray<Value = Self::Container>;
  type ContainerArrayMut: ContainerArrayMut<Value = Self::ContainerMut>;
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

// =============================================================================
// =============================================================================

/// A common interface for mutable value containers.
pub trait ContainerMut: Container {
  /// Sets a new inner [`value`][`Container::Value`].
  fn set(&self, value: &Self::Value);
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

// =============================================================================
// =============================================================================

/// A common interface for mutable arrays of value containers.
pub trait ContainerArrayMut: ContainerArray {
  fn erase(&self);
}

// =============================================================================
// =============================================================================

macro_rules! impl_ScValue {
  (@value, $ident:ident, $primitive:ty) => {
    paste::paste! {
      impl Value for [<Sc $ident>] {
        type Primitive = $primitive;
        type Container = [<ScImmutable $ident>];
        type ContainerMut = [<ScMutable $ident>];
        type ContainerArray = [<ScImmutable $ident Array>];
        type ContainerArrayMut = [<ScMutable $ident Array>];
      }
    }
  };
  (@container, $ident:ident, $value:ty) => {
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
  (@container_mut, $ident:ident, $value:ty) => {
    impl ContainerMut for $ident {
      fn set(&self, value: &Self::Value) {
        self.set_value(value);
      }
    }
  };
  (@container_array, $ident:ident, $value:ty, $fn:ident) => {
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
  (@container_array_mut, $ident:ident) => {
    impl ContainerArrayMut for $ident {
      fn erase(&self) {
        self.clear();
      }
    }
  };
  (@core, $ident:ident, $primitive:ty, $getter:ident) => {
    impl_ScValue!(@value, $ident, $primitive);

    paste::paste! {
      impl_ScValue!(@container, [<ScImmutable $ident>], [<Sc $ident>]);
      impl_ScValue!(@container, [<ScMutable $ident>], [<Sc $ident>]);
      // impl_ScValue!(@container_mut, [<ScMutable $ident>], [<Sc $ident>]);
      impl_ScValue!(@container_array, [<ScImmutable $ident Array>], [<ScImmutable $ident>], $getter);
      impl_ScValue!(@container_array, [<ScMutable $ident Array>], [<ScMutable $ident>], $getter);
      impl_ScValue!(@container_array_mut, [<ScMutable $ident Array>]);
    }
  };
  (Hname, $primitive:ty, $getter:ident) => {
    impl_ScValue!(@core, Hname, $primitive, $getter);
  };
  (Int64, $primitive:ty, $getter:ident) => {
    impl_ScValue!(@core, Int64, $primitive, $getter);
  };
  ($ident:ident, $primitive:ty, $getter:ident) => {
    impl_ScValue!(@core, $ident, $primitive, $getter);

    paste::paste! {
      impl_ScValue!(@container_mut, [<ScMutable $ident>], [<Sc $ident>]);
    }
  };
  ($(($ident:ident, $primitive:ty, $getter:ident),)+) => {
    $(
      impl_ScValue!($ident, $primitive, $getter);
    )+
  };
}

// TODO: Support maps
impl_ScValue! {
  (Address, [u8; 33], get_address),
  (AgentId, [u8; 37], get_agent_id),
  (Bytes, Vec<u8>, get_bytes),
  (ChainId, [u8; 33], get_chain_id),
  (Color, [u8; 32], get_color),
  (ContractId, [u8; 37], get_contract_id),
  (Hash, [u8; 32], get_hash),
  (Hname, u32, get_hname),
  (Int64, i64, get_int64),
  (RequestId, [u8; 34], get_request_id),
  (String, String, get_string),
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
