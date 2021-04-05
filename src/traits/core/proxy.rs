use crate::traits::core::Value;

/// A common interface for immutable value proxies.
pub trait Proxy: Sized {
  /// The value stored in the proxy.
  type Value: Value;

  /// Returns `true` if the proxy contains a [value][Self::Value].
  fn has(&self) -> bool;

  /// Returns the stored [value][Self::Value].
  fn get(&self) -> Self::Value;

  /// Returns the stored [value][Self::Value] if present, otherwise [None].
  fn opt(&self) -> Option<Self::Value> {
    if self.has() {
      Some(self.get())
    } else {
      None
    }
  }
}

/// A common interface for mutable value proxies.
pub trait ProxyMut: Proxy {
  /// Sets the stored [value][Proxy::Value].
  fn set(&self, value: &Self::Value);
}
