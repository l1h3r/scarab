use std::borrow::Cow;
use wasmlib::ScColor;

static IOTA: &ScColor = &ScColor::IOTA;
static MINT: &ScColor = &ScColor::MINT;

mod private {
  pub trait Sealed {}
}

/// Extensions for [ScColor].
pub trait ColorExt: private::Sealed {
  /// Returns `true` if the token color is [IOTA][ScColor::IOTA].
  fn is_iota(&self) -> bool {
    self.color() == IOTA
  }

  /// Returns `true` if the token color is [MINT][ScColor::MINT].
  fn is_mint(&self) -> bool {
    self.color() == MINT
  }

  /// Returns `true` if the token color is non-standard.
  fn is_custom(&self) -> bool {
    !self.is_iota() && !self.is_mint()
  }

  /// Returns a human-friendly name for the token color.
  fn name(&self) -> Cow<'_, str> {
    if self.is_iota() {
      Cow::Borrowed("IOTA")
    } else if self.is_mint() {
      Cow::Borrowed("MINT")
    } else {
      Cow::Owned(self.color().to_string())
    }
  }

  #[doc(hidden)]
  fn color(&self) -> &ScColor;
}

impl private::Sealed for ScColor {}

impl ColorExt for ScColor {
  #[doc(hidden)]
  #[inline(always)]
  fn color(&self) -> &ScColor {
    self
  }
}
