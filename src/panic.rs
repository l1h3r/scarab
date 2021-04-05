use core::intrinsics::abort;

mod private {
  pub trait Sealed {}
}

pub trait Unwrap<T>: private::Sealed {
  fn unwrap_abort(self) -> T;
}

impl<T> private::Sealed for Option<T> {}
impl<T, E> private::Sealed for Result<T, E> {}

impl<T> Unwrap<T> for Option<T> {
  fn unwrap_abort(self) -> T {
    match self {
      Some(item) => item,
      None => abort(),
    }
  }
}

impl<T, E> Unwrap<T> for Result<T, E> {
  fn unwrap_abort(self) -> T {
    match self {
      Ok(item) => item,
      Err(_) => abort(),
    }
  }
}
