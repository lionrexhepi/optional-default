use std::collections::HashSet;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Error, ExprArray, Ident};

pub fn check_required(input: CheckInput) -> TokenStream {
    let required_set = array_to_set(input.required);
    let present_set = array_to_set(input.present);

    let missing = required_set.difference(&present_set).collect::<Vec<_>>();

    if !missing.is_empty() {
        let sp = Span::mixed_site();
        let missing_fields = missing
            .into_iter()
            .map(|ident| ident.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        Error::new(sp, &format!("Missing required fields: {}", missing_fields)).to_compile_error()
    } else {
        quote!()
    }
}

fn array_to_set(array: ExprArray) -> HashSet<Ident> {
    array
        .elems
        .into_iter()
        .map(|expr| match expr {
            syn::Expr::Path(path) => path
                .path
                .require_ident()
                .expect("Field initializers cannot be paths")
                .clone(),
            _ => panic!("Expected path"),
        })
        .collect()
}

pub struct CheckInput {
    required: ExprArray,
    present: ExprArray,
}

impl syn::parse::Parse for CheckInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let required = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let present = input.parse()?;
        Ok(CheckInput { required, present })
    }
}
