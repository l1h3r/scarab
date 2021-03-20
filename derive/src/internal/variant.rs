use syn::Ident;

use crate::internal::Data;
use crate::internal::Field;
use crate::internal::Style;

#[cfg_attr(feature = "extra-traits", derive(Debug))]
pub struct Variant<'a> {
  pub ident: &'a Ident,
  pub style: Style,
  pub fields: Vec<Field<'a>>,
}

impl<'a> Variant<'a> {
  pub fn from_ast(variant: &'a syn::Variant) -> Self {
    // TODO: Handle Variant Attributes

    let ident: &Ident = &variant.ident;
    let (style, fields): _ = Data::expand_struct(&variant.fields);

    Self { ident, style, fields }
  }
}
