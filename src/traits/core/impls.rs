use wasmlib::*;

use crate::traits::core::Array;
use crate::traits::core::ArrayMut;
use crate::traits::core::Proxy;
use crate::traits::core::ProxyMut;
use crate::traits::core::Value;
use crate::types::ScBytes;
use crate::types::ScInt64;
use crate::types::ScString;

macro_rules! impl_ScValue {
  (Hname, $primitive:ty, $getter:ident) => {
    impl_ScValue!(@core, Hname, $primitive, $getter);
  };
  (Int64, $primitive:ty, $getter:ident) => {
    impl_ScValue!(@core, Int64, $primitive, $getter);
  };
  ($ident:ident, $primitive:ty, $getter:ident) => {
    impl_ScValue!(@core, $ident, $primitive, $getter);

    paste::paste! {
      impl_ScValue!(@proxy_mut, [<ScMutable $ident>], [<Sc $ident>]);
    }
  };
  ($(($ident:ident, $primitive:ty, $getter:ident),)+) => {
    $(
      impl_ScValue!($ident, $primitive, $getter);
    )+
  };
  (@value, $ident:ident, $primitive:ty) => {
    paste::paste! {
      impl Value for [<Sc $ident>] {
        type Primitive = $primitive;
        type Proxy = [<ScImmutable $ident>];
        type ProxyMut = [<ScMutable $ident>];
        type Array = [<ScImmutable $ident Array>];
        type ArrayMut = [<ScMutable $ident Array>];
      }
    }
  };
  (@proxy, $ident:ident, $value:ty) => {
    impl Proxy for $ident {
      type Value = $value;

      fn has(&self) -> bool {
        self.exists()
      }

      fn get(&self) -> Self::Value {
        self.value()
      }
    }
  };
  (@proxy_mut, $ident:ident, $value:ty) => {
    impl ProxyMut for $ident {
      fn set(&self, value: &Self::Value) {
        self.set_value(value);
      }
    }
  };
  (@array, $ident:ident, $value:ty, $fn:ident) => {
    impl Array for $ident {
      type Value = $value;

      fn len(&self) -> usize {
        self.length() as usize
      }

      fn proxy(&self, index: usize) -> Self::Value {
        self.$fn(index as i32)
      }
    }
  };
  (@array_mut, $ident:ident) => {
    impl ArrayMut for $ident {
      fn erase(&self) {
        self.clear();
      }
    }
  };
  (@core, $ident:ident, $primitive:ty, $getter:ident) => {
    impl_ScValue!(@value, $ident, $primitive);

    paste::paste! {
      impl_ScValue!(@proxy, [<ScImmutable $ident>], [<Sc $ident>]);
      impl_ScValue!(@proxy, [<ScMutable $ident>], [<Sc $ident>]);
      impl_ScValue!(@array, [<ScImmutable $ident Array>], [<ScImmutable $ident>], $getter);
      impl_ScValue!(@array, [<ScMutable $ident Array>], [<ScMutable $ident>], $getter);
      impl_ScValue!(@array_mut, [<ScMutable $ident Array>]);
    }
  };
}

#[rustfmt::skip]
impl_ScValue! {
  (Address,   [u8; 33], get_address),
  (AgentId,   [u8; 37], get_agent_id),
  (Bytes,     Vec<u8>,  get_bytes),
  (ChainId,   [u8; 33], get_chain_id),
  (Color,     [u8; 32], get_color),
  (Hash,      [u8; 32], get_hash),
  (Hname,     u32,      get_hname),
  (Int64,     i64,      get_int64),
  (RequestId, [u8; 34], get_request_id),
  (String,    String,   get_string),
}

impl ProxyMut for ScMutableHname {
  fn set(&self, value: &Self::Value) {
    self.set_value(value.clone());
  }
}

impl ProxyMut for ScMutableInt64 {
  fn set(&self, value: &Self::Value) {
    self.set_value(*value);
  }
}
