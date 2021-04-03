use crate::traits::core::Value;

/// A common interface for immutable value proxies.
pub trait Proxy: Sized {
  /// The inner value stored in the proxy.
  type Value: Value;

  /// Returns `true` if the proxy has a non-empty value.
  fn has(&self) -> bool;

  /// Returns the inner [value][Self::Value] of the proxy.
  fn get(&self) -> Self::Value;
}

/// A common interface for mutable value proxies.
pub trait ProxyMut: Proxy {
  /// Sets the contained [value][Proxy::Value].
  fn set(&self, value: &Self::Value);
}
