use wasmlib::MapKey;
use wasmlib::ScBaseContext;
use wasmlib::ScFuncContext;
use wasmlib::ScImmutableMap;
use wasmlib::ScMutableMap;
use wasmlib::ScViewContext;

use crate::traits::core::MapGet;
use crate::traits::core::MapSet;
use crate::traits::core::Proxy;
use crate::traits::core::Value;
use crate::traits::extension::MapExt;

mod private {
  pub struct Private;
  pub trait Sealed: super::ScBaseContext {}
}

fn intkey<T: MapKey + ?Sized>(key: &T) -> i32 {
  key.get_key_id().0
}

/// Extensions for [ScViewContext] and [ScFuncContext].
pub trait ContextExt: private::Sealed {
  type State: MapExt;

  // ===========================================================================
  // Params
  // ===========================================================================

  fn get_param_proxy<T, U>(&self, key: &T) -> U
  where
    T: MapKey + ?Sized,
    U: MapGet<ScImmutableMap>,
  {
    self.params().get(key)
  }

  fn get_param<T, U>(&self, key: &T) -> U
  where
    T: MapKey + ?Sized,
    U: Value,
    U::Proxy: MapGet<ScImmutableMap>,
  {
    self.params().get_value(key)
  }

  fn get_required_param_proxy<T, U>(&self, key: &T) -> U
  where
    T: MapKey + ?Sized,
    U: Proxy + MapGet<ScImmutableMap>,
  {
    let this: U = self.params().get(key);

    if !this.has() {
      self.panic(&format!("missing required param: Key32({})", intkey(key)));
    }

    this
  }

  fn get_required_param<T, U>(&self, key: &T) -> U
  where
    T: MapKey + ?Sized,
    U: Value,
    U::Proxy: MapGet<ScImmutableMap>,
  {
    self.get_required_param_proxy::<T, U::Proxy>(key).get()
  }

  // ===========================================================================
  // State
  // ===========================================================================

  fn get_state_proxy<T, U>(&self, key: &T) -> U
  where
    T: MapKey + ?Sized,
    U: MapGet<Self::State>,
  {
    self.state(private::Private).get(key)
  }

  fn get_state<T, U>(&self, key: &T) -> U
  where
    T: MapKey + ?Sized,
    U: Value,
    U::Proxy: MapGet<Self::State>,
  {
    self.state(private::Private).get_value(key)
  }

  fn get_required_state_proxy<T, U>(&self, key: &T) -> U
  where
    T: MapKey + ?Sized,
    U: Proxy + MapGet<Self::State>,
  {
    let this: U = self.state(private::Private).get(key);

    if !this.has() {
      self.panic(&format!("missing required param: Key32({})", intkey(key)));
    }

    this
  }

  fn get_required_state<T, U>(&self, key: &T) -> U
  where
    T: MapKey + ?Sized,
    U: Value,
    U::Proxy: MapGet<Self::State>,
  {
    self.get_required_state_proxy::<T, U::Proxy>(key).get()
  }

  // ===========================================================================
  // Misc. Helpers
  // ===========================================================================

  fn result<T, U>(&self, key: &T, value: U)
  where
    T: MapKey + ?Sized,
    U: MapSet<ScMutableMap>,
  {
    value.map_set(&self.results(), key);
  }

  fn view(&self) -> &ScViewContext;

  #[doc(hidden)]
  fn state(&self, _: private::Private) -> Self::State;
}

impl private::Sealed for ScViewContext {}
impl private::Sealed for ScFuncContext {}

impl ContextExt for ScViewContext {
  type State = ScImmutableMap;

  #[inline(always)]
  fn view(&self) -> &ScViewContext {
    self
  }

  #[doc(hidden)]
  #[inline(always)]
  fn state(&self, _: private::Private) -> Self::State {
    self.state()
  }
}

impl ContextExt for ScFuncContext {
  type State = ScMutableMap;

  #[inline(always)]
  fn view(&self) -> &ScViewContext {
    &ScViewContext {}
  }

  #[doc(hidden)]
  #[inline(always)]
  fn state(&self, _: private::Private) -> Self::State {
    self.state()
  }
}
