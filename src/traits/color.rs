use std::borrow::Cow;
use wasmlib::ScColor;

static IOTA: &ScColor = &ScColor::IOTA;
static MINT: &ScColor = &ScColor::MINT;

mod private {
  pub struct Private;
  pub trait Sealed {}
}

pub trait ColorExt: private::Sealed {
  fn is_iota(&self) -> bool {
    self.__bytes(private::Private) == IOTA.to_bytes()
  }

  fn is_mint(&self) -> bool {
    self.__bytes(private::Private) == MINT.to_bytes()
  }

  fn name(&self) -> Cow<'_, str> {
    if self.is_iota() {
      Cow::Borrowed("IOTA")
    } else if self.is_mint() {
      Cow::Borrowed("MINT")
    } else {
      Cow::Owned(self.__string(private::Private))
    }
  }

  #[doc(hidden)]
  fn __bytes(&self, _: private::Private) -> &[u8];

  #[doc(hidden)]
  fn __string(&self, _: private::Private) -> String;
}

impl private::Sealed for ScColor {}

impl ColorExt for ScColor {
  #[doc(hidden)]
  fn __bytes(&self, _: private::Private) -> &[u8] {
    self.to_bytes()
  }

  #[doc(hidden)]
  fn __string(&self, _: private::Private) -> String {
    self.to_string()
  }
}
