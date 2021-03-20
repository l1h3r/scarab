#![feature(or_patterns)]

extern crate proc_macro;

mod decode;
mod encode;
mod internal;

use proc_macro::TokenStream;
use syn::parse_macro_input;
use syn::DeriveInput;

#[proc_macro_derive(Encode, attributes(scarab))]
pub fn derive_encode(input: TokenStream) -> TokenStream {
  encode::derive(parse_macro_input!(input as DeriveInput))
    .unwrap_or_else(|error| error.to_compile_error())
    .into()
}

#[proc_macro_derive(Decode, attributes(scarab))]
pub fn derive_decode(input: TokenStream) -> TokenStream {
  decode::derive(parse_macro_input!(input as DeriveInput))
    .unwrap_or_else(|error| error.to_compile_error())
    .into()
}
