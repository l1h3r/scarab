use crate::traits::core::Proxy;
use crate::traits::core::ProxyMut;

/// A common interface for immutable array proxies.
pub trait Array: Sized {
  /// The inner value stored in the array.
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
    (0..self.len()).map(|index| self.get(index)).collect()
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
}

// =============================================================================
// =============================================================================

/// A by-reference [array][Array] iterator.
pub struct Iter<'a, T> {
  index: usize,
  scope: &'a T,
}

impl<'a, T> Iter<'a, T> {
  const fn new(scope: &'a T) -> Self {
    Self { index: 0, scope }
  }
}

impl<'a, T> Iterator for Iter<'a, T>
where
  T: Array,
{
  type Item = <T::Value as Proxy>::Value;

  fn next(&mut self) -> Option<Self::Item> {
    if self.index >= self.scope.len() {
      return None;
    }

    let next: Self::Item = self.scope.get(self.index);

    self.index = self.index.checked_add(1)?;

    Some(next)
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
  index: usize,
  scope: T,
}

impl<T> IntoIter<T> {
  const fn new(scope: T) -> Self {
    Self { index: 0, scope }
  }
}

impl<T> Iterator for IntoIter<T>
where
  T: Array,
{
  type Item = <T::Value as Proxy>::Value;

  fn next(&mut self) -> Option<Self::Item> {
    if self.index >= self.scope.len() {
      return None;
    }

    let next: Self::Item = self.scope.get(self.index);

    self.index = self.index.checked_add(1)?;

    Some(next)
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
