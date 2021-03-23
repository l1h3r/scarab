//! Random number generation.

use rand_core::impls;
use wasmlib::ScBaseContext;
use wasmlib::ScViewContext;

#[doc(inline)]
pub use rand_core::Error;

#[doc(inline)]
pub use rand_core::RngCore;

static __VIEW: &ScViewContext = &ScViewContext {};

/// A random number generator that retrieves randomness from Smart Contract
/// Transaction IDs.
///
/// This is a zero-sized struct. It can be freely constructed with `ScRng`.
#[derive(Clone, Copy, Debug, Default)]
pub struct ScRng;

impl RngCore for ScRng {
  fn next_u32(&mut self) -> u32 {
    __VIEW.utility().random(i64::from(u32::MAX)) as u32
  }

  fn next_u64(&mut self) -> u64 {
    impls::next_u64_via_u32(self)
  }

  fn fill_bytes(&mut self, dest: &mut [u8]) {
    impls::fill_bytes_via_next(self, dest);
  }

  fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
    self.fill_bytes(dest);
    Ok(())
  }
}
