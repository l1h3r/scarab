use wasmlib::MapKey;
use wasmlib::ScBaseContext;
use wasmlib::ScFuncContext;
use wasmlib::ScImmutableMap;
use wasmlib::ScViewContext;

use crate::traits::Container;
use crate::traits::MapExt;
use crate::traits::MapGet;
use crate::traits::MapSet;
use crate::traits::MapValue;

pub trait ContextExt: ScBaseContext {
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
    ScImmutableMap: MapGet<U::Container>,
    U: MapValue<ScImmutableMap>,
  {
    self.params().get_value(key)
  }

  fn get_required_param<T>(&self, key: &str) -> T
  where
    ScImmutableMap: MapGet<T::Container>,
    T: MapValue<ScImmutableMap>,
  {
    let this: T::Container = self.get_param_container(key);

    if !this.has() {
      self.panic(&format!("missing required param: {:?}", key));
    }

    this.get()
  }

  fn result<T, U>(&self, key: &T, value: U)
  where
    T: MapKey + ?Sized,
    U: MapSet,
  {
    value.mset(&self.results(), key);
  }

  fn view(&self) -> &ScViewContext;
}

impl ContextExt for ScViewContext {
  fn view(&self) -> &ScViewContext {
    self
  }
}

impl ContextExt for ScFuncContext {
  fn view(&self) -> &ScViewContext {
    &ScViewContext {}
  }
}
