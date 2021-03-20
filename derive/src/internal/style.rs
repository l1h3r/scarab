#[derive(Clone, Copy)]
#[cfg_attr(feature = "extra-traits", derive(Debug))]
pub enum Style {
  Struct,
  Tuple,
  Unit,
}
