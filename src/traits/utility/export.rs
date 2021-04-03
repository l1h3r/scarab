use wasmlib::ScExports;

pub trait Export {
  fn register(exports: &ScExports);

  fn export() {
    Self::register(&ScExports::new())
  }
}
