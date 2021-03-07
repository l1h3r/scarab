use wasmlib::MapKey;
use wasmlib::ScBaseContext;
use wasmlib::ScFuncContext;
use wasmlib::ScImmutableMap;
use wasmlib::ScViewContext;

use crate::traits::Container;
use crate::traits::MapExt;
use crate::traits::MapGet;
use crate::traits::MapSet;

pub trait ContextExt: ScBaseContext {
  fn get_param<T, U>(&self, key: &T) -> U
  where
    T: MapKey + ?Sized,
    ScImmutableMap: MapGet<U>;

  fn get_required_param<T>(&self, key: &str) -> T
  where
    ScImmutableMap: MapGet<T>,
    T: Container,
  {
    let this: T = self.get_param(key);

    if !this.has() {
      self.panic(&format!("missing required param: {:?}", key));
    }

    this
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
  fn get_param<T, U>(&self, key: &T) -> U
  where
    T: MapKey + ?Sized,
    ScImmutableMap: MapGet<U>,
  {
    self.params().get(key)
  }

  fn view(&self) -> &ScViewContext {
    self
  }
}

impl ContextExt for ScFuncContext {
  fn get_param<T, U>(&self, key: &T) -> U
  where
    T: MapKey + ?Sized,
    ScImmutableMap: MapGet<U>,
  {
    self.params().get(key)
  }

  fn view(&self) -> &ScViewContext {
    &ScViewContext {}
  }
}
