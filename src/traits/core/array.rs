use crate::traits::core::Proxy;
use crate::traits::core::ProxyMut;

/// A common interface for immutable array proxies.
pub trait Array: Sized {
  /// The value stored in the array.
  type Value: Proxy;

  /// Returns the number of values in the array.
  fn len(&self) -> usize;

  /// Returns a reference to the proxy object at the given `index`.
  fn proxy(&self, index: usize) -> Self::Value;

  /// Returns `true` if the array contains no values.
  fn is_empty(&self) -> bool {
    self.len() == 0
  }

  /// Returns a reference to the value at the given `index`.
  fn get(&self, index: usize) -> <Self::Value as Proxy>::Value {
    self.proxy(index).get()
  }

  /// Returns a newly allocated vector containing all values in the array.
  fn to_vec(&self) -> Vec<<Self::Value as Proxy>::Value> {
    self.iter().collect()
  }

  /// Returns an iterator over the array.
  fn iter(&self) -> Iter<'_, Self> {
    Iter::new(self)
  }

  /// Returns a by-value iterator over the array.
  fn into_iter(self) -> IntoIter<Self> {
    IntoIter::new(self)
  }
}

/// A common interface for mutable array proxies.
pub trait ArrayMut: Array
where
  Self::Value: ProxyMut,
{
  /// Clears the array, removing all values.
  fn erase(&self);

  /// Appends a value to the back of the array.
  fn push(&self, value: <Self::Value as Proxy>::Value) {
    self.proxy(self.len()).set(&value);
  }

  /// Appends a value to the back of the array.
  fn push_proxy(&self, value: Self::Value) {
    self.push(value.get());
  }
}

// =============================================================================
// =============================================================================

/// A by-reference [array][Array] iterator.
pub struct Iter<'a, T> {
  index: Option<usize>,
  scope: &'a T,
}

impl<'a, T> Iter<'a, T> {
  const fn new(scope: &'a T) -> Self {
    Self { index: Some(0), scope }
  }

  fn step(&mut self) {
    self.index = self.index.and_then(|index| index.checked_add(1));
  }
}

impl<'a, T> Iterator for Iter<'a, T>
where
  T: Array,
{
  type Item = <T::Value as Proxy>::Value;

  fn next(&mut self) -> Option<Self::Item> {
    let index: usize = self.index?;

    if index >= self.scope.len() {
      None
    } else {
      self.step();
      Some(self.scope.get(index))
    }
  }
}

impl<'a, T> Clone for Iter<'a, T> {
  fn clone(&self) -> Self {
    Self {
      index: self.index,
      scope: self.scope,
    }
  }
}

// =============================================================================
// =============================================================================

/// A by-value [array][Array] iterator.
pub struct IntoIter<T> {
  index: Option<usize>,
  scope: T,
}

impl<T> IntoIter<T> {
  const fn new(scope: T) -> Self {
    Self { index: Some(0), scope }
  }

  fn step(&mut self) {
    self.index = self.index.and_then(|index| index.checked_add(1));
  }
}

impl<T> Iterator for IntoIter<T>
where
  T: Array,
{
  type Item = <T::Value as Proxy>::Value;

  fn next(&mut self) -> Option<Self::Item> {
    let index: usize = self.index?;

    if index >= self.scope.len() {
      None
    } else {
      self.step();
      Some(self.scope.get(index))
    }
  }
}

impl<T: Clone> Clone for IntoIter<T> {
  fn clone(&self) -> Self {
    Self {
      index: self.index,
      scope: self.scope.clone(),
    }
  }
}
