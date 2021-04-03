use wasmlib::*;

use crate::traits::core::MapGet;
use crate::traits::core::MapSet;
use crate::traits::core::Proxy;
use crate::traits::core::Value;

mod private {
  pub trait Sealed {}
}

/// Extensions for [ScImmutableMap] and [ScMutableMap].
pub trait MapExt: private::Sealed {
  /// Returns the value specified by `key`.
  fn get_value<T, U>(&self, key: &T) -> U
  where
    T: MapKey + ?Sized,
    U: Value,
    U::Proxy: MapGet<Self>,
  {
    U::Proxy::map_get(self, key).get()
  }

  /// Returns the value proxy specified by `key`.
  fn get<T, U>(&self, key: &T) -> U
  where
    T: MapKey + ?Sized,
    U: MapGet<Self>,
  {
    U::map_get(self, key)
  }

  /// Sets the `value` of `key`.
  fn set<T, U>(&self, key: &T, value: &U)
  where
    T: MapKey + ?Sized,
    U: MapSet<Self> + ?Sized,
  {
    U::map_set(value, self, key);
  }

  /// Returns `true` if the map contains `key`.
  fn contains<T, U>(&self, key: &T) -> bool
  where
    T: MapKey + ?Sized,
    U: Value,
    U::Proxy: MapGet<Self>,
  {
    U::Proxy::map_get(self, key).has()
  }
}

impl private::Sealed for ScImmutableMap {}
impl private::Sealed for ScMutableMap {}

impl MapExt for ScImmutableMap {}
impl MapExt for ScMutableMap {}
