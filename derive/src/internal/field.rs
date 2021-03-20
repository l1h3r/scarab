use syn::Ident;
use syn::Member;

#[cfg_attr(feature = "extra-traits", derive(Debug))]
pub struct Field<'a> {
  pub index: usize,
  pub ident: Option<&'a Ident>,
  pub member: Member,
}

impl<'a> Field<'a> {
  pub fn from_ast(field: &'a syn::Field, index: usize) -> Self {
    // TODO: Handle Field Attributes

    let ident: Option<&Ident> = field.ident.as_ref();

    let member: Member = match ident {
      Some(ident) => Member::Named(ident.clone()),
      None => Member::Unnamed(index.into()),
    };

    Self { index, ident, member }
  }
}
