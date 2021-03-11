#[macro_export]
macro_rules! log {
  ($ctx:expr, $($tt:tt)*) => {
    ::wasmlib::ScBaseContext::log($ctx, &format!($($tt)*))
  };
}

#[macro_export]
macro_rules! trace {
  ($ctx:expr, $($tt:tt)*) => {
    ::wasmlib::ScBaseContext::trace($ctx, &format!($($tt)*))
  };
}

#[macro_export]
macro_rules! imap {
  ($($tt:tt)*) => {
    map!($($tt)*).immutable()
  };
}

#[macro_export]
macro_rules! map {
  (@internal $($key:expr => $value:expr),* $(,)*) => {{
    let params: ::wasmlib::ScMutableMap = ::wasmlib::ScMutableMap::new();
    $(
      params.set($key, $value);
    )*
    params
  }};
  ($($tt:tt)*) => {
    map!(@internal $($tt)*)
  };
}
