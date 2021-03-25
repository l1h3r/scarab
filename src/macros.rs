/// Logs an informational message.
///
/// This macro is similar to [println!] from the std library.
///
/// ## Examples
///
/// ```
/// log!("hello there!");
/// log!("format {} arguments", "some");
/// ```
#[macro_export]
macro_rules! log {
  ($($tt:tt)*) => {
    $crate::wasmlib::host::log(&format!($($tt)*))
  };
}

/// Logs a debug message.
///
/// This macro is similar to [println!] from the std library.
///
/// ## Examples
///
/// ```
/// trace!("hello there!");
/// trace!("format {} arguments", "some");
/// ```
#[macro_export]
macro_rules! trace {
  ($($tt:tt)*) => {
    $crate::wasmlib::host::trace(&format!($($tt)*))
  };
}

/// Creates a new [ScImmutableMap][wasmlib::ScImmutableMap] from a set of
/// initial values.
///
/// ```
/// let immutable = imap! {
///   "int64" => &0,
///   "hname" => &ScHname::new("something"),
/// };
/// ```
#[macro_export]
macro_rules! imap {
  ($($tt:tt)*) => {
    $crate::map!($($tt)*).immutable()
  };
}

/// Creates a new [ScMutableMap][wasmlib::ScMutableMap] from a set of
/// initial values.
///
/// ```
/// let mutable = imap! {
///   "int64" => &0,
///   "hname" => &ScHname::new("something"),
/// };
/// ```
#[macro_export]
macro_rules! map {
  ($($tt:tt)*) => {
    // Delegate to another macro to avoid showing internals in generated docs
    $crate::map_internal!($($tt)*)
  };
}

#[macro_export]
#[doc(hidden)]
macro_rules! map_internal {
  (@internal $($key:expr => $value:expr),* $(,)*) => {{
    let params: $crate::wasmlib::ScMutableMap = $crate::wasmlib::ScMutableMap::new();
    $(
      params.set($key, $value);
    )*
    params
  }};
  ($($tt:tt)*) => {
    $crate::map!(@internal $($tt)*)
  };
}

#[macro_export]
macro_rules! delegate {
  (@function, $remote:ident, $name:ident ( $($param:ident : $ty:ty),*) -> $out:ty) => {
    fn $name ( $($param : $ty),* ) -> $out {
      $remote::$name ( $($param),* )
    }
  };
  (@function, $remote:ident, $name:ident ( $($param:ident : $ty:ty),*)) => {
    delegate! {
      @function, $remote, $name ( $($param : $ty),* ) -> ()
    }
  };
  ($local:ident implements $trait:ident through $remote:ident => {
    $(
      fn $name:ident ( $($param:ident : $ty:ty),* $(,)* )  $(-> $out:ty)* ;
    )*
  }) => {
    impl $trait for $local {
      $(
        delegate! {
          @function, $remote, $name ( $($param : $ty),* ) $(-> $out)*
        }
      )*
    }
  };
}
