use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;
use syn::Error;
use syn::Ident;
use syn::Result;

use crate::internal::Container;
use crate::internal::Data;
use crate::internal::Field;
use crate::internal::Style;

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
  let container: Container = Container::from_ast(&input)?;

  // TODO: Check Attributes
  // TODO: Support Generics

  let ident: &Ident = container.ident;
  let dummy: Ident = Ident::new(&format!("_IMPL_ENCODE_FOR_{}", ident), Span::call_site());
  let tokens: TokenStream = expand_container(&container)?;

  Ok(quote! {
    #[allow(non_upper_case_globals)]
    const #dummy: () = {
      impl ::scarab::export::Encode for #ident {
        fn encode(&self, __ENCODER: &mut ::scarab::export::BytesEncoder) {
          #tokens
        }
      }
    };
  })
}

fn expand_container(container: &Container) -> Result<TokenStream> {
  match container.data {
    Data::Struct(Style::Struct, ref fields) => Ok(expand_fields(fields)),
    Data::Struct(Style::Tuple, ref fields) => Ok(expand_fields(fields)),
    Data::Struct(Style::Unit, _) => Err(Error::new(Span::call_site(), "Can't derive Encode for unit structs")),
    Data::Enum(_) => Err(Error::new(Span::call_site(), "Can't derive Encode for enums")),
  }
}

fn expand_fields(fields: &[Field]) -> TokenStream {
  let stmts: _ = fields
    .iter()
    .map(|field| &field.member)
    .map(|field| quote!(::scarab::export::Encode::encode(&self.#field, __ENCODER)));

  quote! {
    #(#stmts;)*
  }
}
