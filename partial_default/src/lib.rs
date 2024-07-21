mod check_helper;
mod derive;

#[proc_macro_derive(PartialDefault, attributes(optional))]
pub fn derive_partial_default(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    derive::partial_default(input).into()
}

#[proc_macro]
pub fn check_required(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as check_helper::CheckInput);
    check_helper::check_required(input).into()
}
