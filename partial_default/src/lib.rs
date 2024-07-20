use proc_macro::TokenStream;
use syn::{DeriveInput, ExprStruct};

mod derive;
mod init;

#[proc_macro_derive(PartialDefault, attributes(optional))]
pub fn partial_default(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    derive::partial_default(input).into()
}

#[proc_macro]
pub fn new(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as ExprStruct);
    init::new(input).into()
}
