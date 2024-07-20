use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::ExprStruct;

pub fn new(input: ExprStruct) -> TokenStream {
    let mut name = input.path;

    if let Some(last) = name.segments.last_mut() {
        last.ident = format_ident!("{}Builder", last.ident);
    }

    let fields = input
        .fields
        .into_iter()
        .map(|field| {
            let ident = field.member.clone();
            let val = field.expr;
            quote! {
                .#ident(#val)
            }
        })
        .collect::<Vec<_>>();

    quote! {
       #name::new()
           #( #fields )*
           .build()

    }
}

#[test]
fn test_new() {
    let input = syn::parse_quote! {
        Foo {
            a: 1,
            b,
        }
    };

    let output = new(input);

    let expected = quote! {
        FooBuilder {
            .a(1)
            .b(b)
            .build()
        }
    };

    assert_eq!(output.to_string(), expected.to_string());
}
