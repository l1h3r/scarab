use wasmlib::ScMutableInt64;

/// Unchecked math operations. You don't want this.
pub trait UnsafeMath {
  fn inc_value(&self, amount: i64);
  fn dec_value(&self, amount: i64);
}

impl UnsafeMath for ScMutableInt64 {
  fn inc_value(&self, amount: i64) {
    self.set_value(self.value().saturating_add(amount));
  }

  fn dec_value(&self, amount: i64) {
    self.set_value(self.value().saturating_sub(amount));
  }
}
