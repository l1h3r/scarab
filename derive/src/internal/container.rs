use syn::DeriveInput;
use syn::Generics;
use syn::Ident;
use syn::Result;

use crate::internal::Data;

#[cfg_attr(feature = "extra-traits", derive(Debug))]
pub struct Container<'a> {
  pub ident: &'a Ident,
  pub data: Data<'a>,
  pub generics: &'a Generics,
}

impl<'a> Container<'a> {
  pub fn from_ast(input: &'a DeriveInput) -> Result<Self> {
    // TODO: Handle Container Attributes

    Ok(Self {
      data: Data::from_ast(&input.data)?,
      ident: &input.ident,
      generics: &input.generics,
    })
  }
}
