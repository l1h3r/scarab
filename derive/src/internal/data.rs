use proc_macro2::Span;
use syn::punctuated::Punctuated;
use syn::DataEnum;
use syn::DataStruct;
use syn::Error;
use syn::Fields;
use syn::FieldsNamed;
use syn::FieldsUnnamed;
use syn::Result;
use syn::Token;

use crate::internal::Field;
use crate::internal::Style;
use crate::internal::Variant;

#[cfg_attr(feature = "extra-traits", derive(Debug))]
pub enum Data<'a> {
  Struct(Style, Vec<Field<'a>>),
  Enum(Vec<Variant<'a>>),
}

impl<'a> Data<'a> {
  pub fn from_ast(data: &'a syn::Data) -> Result<Self> {
    match data {
      syn::Data::Struct(DataStruct { ref fields, .. }) => {
        let (style, fields): (Style, Vec<Field<'a>>) = Self::expand_struct(fields);
        Ok(Self::Struct(style, fields))
      }
      syn::Data::Enum(DataEnum { ref variants, .. }) => Ok(Self::Enum(Self::expand_enum(variants))),
      syn::Data::Union(_) => Err(Error::new(Span::call_site(), "Cannot derive Encode for unions")),
    }
  }

  pub fn expand_struct(fields: &Fields) -> (Style, Vec<Field<'_>>) {
    match fields {
      Fields::Named(FieldsNamed { ref named, .. }) => (Style::Struct, Self::expand_fields(named)),
      Fields::Unnamed(FieldsUnnamed { ref unnamed, .. }) => (Style::Tuple, Self::expand_fields(unnamed)),
      Fields::Unit => (Style::Unit, Vec::new()),
    }
  }

  pub fn expand_enum(variants: &Punctuated<syn::Variant, Token![,]>) -> Vec<Variant<'_>> {
    variants.iter().map(|variant| Variant::from_ast(variant)).collect()
  }

  pub fn expand_fields(fields: &Punctuated<syn::Field, Token![,]>) -> Vec<Field<'_>> {
    fields
      .iter()
      .enumerate()
      .map(|(index, field)| Field::from_ast(field, index))
      .collect()
  }
}
