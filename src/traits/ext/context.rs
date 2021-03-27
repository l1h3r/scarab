use wasmlib::MapKey;
use wasmlib::ScBaseContext;
use wasmlib::ScFuncContext;
use wasmlib::ScImmutableMap;
use wasmlib::ScMutableMap;
use wasmlib::ScViewContext;

use crate::traits::Container;
use crate::traits::MapExt;
use crate::traits::MapGet;
use crate::traits::MapSet;
use crate::traits::Value;

mod private {
  pub trait Sealed {}
}

/// Extensions for [ScViewContext] and [ScFuncContext].
pub trait ContextExt: private::Sealed + ScBaseContext {
  type State;

  //
  // Params
  //

  fn get_param_container<T, U>(&self, key: &T) -> U
  where
    T: MapKey + ?Sized,
    ScImmutableMap: MapGet<U>,
  {
    self.params().get(key)
  }

  fn get_param<T, U>(&self, key: &T) -> U
  where
    T: MapKey + ?Sized,
    U: Value,
    ScImmutableMap: MapGet<U::Container>,
  {
    self.params().get_value(key)
  }

  fn get_required_param_container<T>(&self, key: &str) -> T
  where
    T: Container,
    ScImmutableMap: MapGet<T>,
  {
    let this: T = self.params().get(key);

    if !this.has() {
      self.panic(&format!("missing required param: {:?}", key));
    }

    this
  }

  fn get_required_param<T>(&self, key: &str) -> T
  where
    T: Value,
    ScImmutableMap: MapGet<T::Container>,
  {
    self.get_required_param_container(key).get()
  }

  //
  // State
  //

  fn get_state_container<T, U>(&self, key: &T) -> U
  where
    T: MapKey + ?Sized,
    Self::State: MapGet<U> + MapExt,
  {
    self.__state().get(key)
  }

  fn get_state<T, U>(&self, key: &T) -> U
  where
    T: MapKey + ?Sized,
    U: Value,
    Self::State: MapGet<U::Container> + MapExt,
  {
    self.__state().get_value(key)
  }

  fn get_required_state_container<T>(&self, key: &str) -> T
  where
    T: Container,
    Self::State: MapGet<T> + MapExt,
  {
    let this: T = self.__state().get(key);

    if !this.has() {
      self.panic(&format!("missing required param: {:?}", key));
    }

    this
  }

  fn get_required_state<T>(&self, key: &str) -> T
  where
    T: Value,
    Self::State: MapGet<T::Container> + MapExt,
  {
    self.get_required_state_container(key).get()
  }

  //
  // Misc
  //

  fn result<T, U>(&self, key: &T, value: U)
  where
    T: MapKey + ?Sized,
    U: MapSet<ScMutableMap>,
  {
    value.mset(&self.results(), key);
  }

  fn emit<E>(&self, _: E) {
    // TODO
  }

  fn view(&self) -> &ScViewContext;

  #[doc(hidden)]
  fn __state(&self) -> Self::State;
}

impl private::Sealed for ScViewContext {}
impl private::Sealed for ScFuncContext {}

impl ContextExt for ScViewContext {
  type State = ScImmutableMap;

  fn view(&self) -> &ScViewContext {
    self
  }

  #[doc(hidden)]
  fn __state(&self) -> Self::State {
    self.state()
  }
}

impl ContextExt for ScFuncContext {
  type State = ScMutableMap;

  fn view(&self) -> &ScViewContext {
    &ScViewContext {}
  }

  #[doc(hidden)]
  fn __state(&self) -> Self::State {
    self.state()
  }
}
