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
macro_rules! params {
  (@internal $($key:expr => $value:expr),* $(,)*) => {{
    let params: ScMutableMap = ScMutableMap::new();
    $(
      params.set($key, $value);
    )*
    params
  }};
  ($($tt:tt)*) => {
    params!(@internal $($tt)*)
  };
}
