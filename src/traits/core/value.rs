use crate::traits::core::Array;
use crate::traits::core::ArrayMut;
use crate::traits::core::Proxy;
use crate::traits::core::ProxyMut;

/// A common interface for smart contract values.
pub trait Value: Sized {
  /// The primitive type of this smart contract value.
  type Primitive: ?Sized;

  /// The immutable proxy for this value.
  type Proxy: Proxy<Value = Self>;

  /// The mutable proxy for this value.
  type ProxyMut: ProxyMut<Value = Self>;

  /// The immutable array proxy for this value.
  type Array: Array<Value = Self::Proxy>;

  /// The mutable array proxy for this value.
  type ArrayMut: ArrayMut<Value = Self::ProxyMut>;
}
