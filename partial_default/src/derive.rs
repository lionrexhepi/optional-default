use core::panic;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse_quote, punctuated::Punctuated, spanned::Spanned, DeriveInput, Expr, ExprAssign,
    FieldValue, Fields, GenericArgument, GenericParam, Ident, ItemImpl, ItemStruct, Meta, Token,
};

pub fn partial_default(input: DeriveInput) -> TokenStream {
    let struct_name = input.ident;
    let builder_name = Ident::new(&format!("{}Builder", struct_name), struct_name.span());
    let fields = match input.data {
        syn::Data::Struct(ref data) => extract_field_info(&data.fields),
        _ => panic!("PartialDefault only works with structs"),
    };

    let builder = generate_builder(&builder_name, &fields);

    let setter_impl = generate_setter_impl(&builder_name, &fields);
    let new_impl = generate_new_impl(&builder_name, &fields);

    let build_impl = generate_build_impl(&struct_name, &builder_name, &fields);

    quote! {
        #builder
        #setter_impl
        #new_impl
        #build_impl
    }
}

fn generate_builder(builder_name: &Ident, fields: &Vec<FieldInfo>) -> ItemStruct {
    let option_fields = fields
        .iter()
        .map(|field| {
            let ident = &field.ident;
            let ty = &field.ty;
            quote! {
                #ident: Option<#ty>
            }
        })
        .collect::<Punctuated<_, Token![,]>>();

    let flags = fields
        .iter()
        .map(|FieldInfo { flag, .. }| quote! (const #flag: bool))
        .collect::<Punctuated<_, Token![,]>>();

    parse_quote! {
        #[derive(Default)]
        struct #builder_name<#flags> {
            #option_fields
        }
    }
}

fn generate_setter_impl(builder_name: &Ident, fields: &Vec<FieldInfo>) -> ItemImpl {
    let flags = fields
        .iter()
        .map(|FieldInfo { flag, .. }| flag)
        .collect::<Vec<_>>();
    let setters = fields.iter().map(|field| {
        let ident = &field.ident;
        let ty = &field.ty;

        let flag_setter = fields
            .iter()
            .map(
                |FieldInfo {
                     ident: other, flag, ..
                 }| {
                    if other == ident {
                        quote! { true }
                    } else {
                        quote! { #flag }
                    }
                },
            )
            .collect::<Punctuated<_, Token![,]>>();

        let field_setters = fields
            .iter()
            .filter_map(|FieldInfo { ident, .. }| {
                if ident != &field.ident {
                    Some(quote! {
                        #ident: self. #ident
                    })
                } else {
                    None
                }
            })
            .collect::<Punctuated<_, Token![,]>>();

        quote! {
            pub fn #ident(self, value: #ty) -> #builder_name<#flag_setter> {
                #builder_name {
                    #ident: Some(value),
                    #field_setters
                }
            }
        }
    });

    parse_quote! {
        impl<#(const #flags:bool,)*> #builder_name<#(#flags,)*> {
            #(#setters)*
        }
    }
}

fn generate_new_impl(builder_name: &Ident, fields: &Vec<FieldInfo>) -> ItemImpl {
    let flags = (0..fields.len())
        .map(|_| quote!(false))
        .collect::<Punctuated<_, Token![,]>>();

    let setters = fields
        .iter()
        .map(|FieldInfo { ident, .. }| -> FieldValue {
            parse_quote! { #ident: None }
        })
        .collect::<Punctuated<_, Token![,]>>();

    parse_quote! {
        impl #builder_name<#flags> {
            pub fn new() -> Self {
                #builder_name {
                    #setters
                }
            }
        }
    }
}

fn generate_build_impl(
    struct_name: &Ident,
    builder_name: &Ident,
    fields: &Vec<FieldInfo>,
) -> ItemImpl {
    let impl_generics = fields
        .iter()
        .filter_map(|FieldInfo { flag, default, .. }| -> Option<GenericParam> {
            default.as_ref().map(|_| parse_quote! { const #flag:bool })
        })
        .collect::<Punctuated<_, Token![,]>>();

    let builder_generics = fields
        .iter()
        .map(|FieldInfo { flag, default, .. }| -> GenericArgument {
            if default.is_some() {
                parse_quote! { #flag }
            } else {
                parse_quote! { true }
            }
        })
        .collect::<Punctuated<_, Token![,]>>();

    let fields = fields
        .iter()
        .map(|FieldInfo { ident, default, .. }| -> FieldValue {
            match default {
                Some(default) => parse_quote! { #ident: self. #ident .unwrap_or(#default) },
                None => parse_quote! { #ident: self.#ident.unwrap() },
            }
        })
        .collect::<Punctuated<_, Token![,]>>();

    parse_quote! {
        impl<#impl_generics> #builder_name <#builder_generics> {
            pub fn build(self) -> #struct_name {
                #struct_name {
                    #fields
                }
            }
        }
    }
}

fn extract_field_info(fields: &Fields) -> Vec<FieldInfo> {
    fields
        .iter()
        .map(|field| {
            let ident = field.ident.as_ref().unwrap().clone();
            let ty = field.ty.clone();
            let flag_name = ident.to_string().to_ascii_uppercase();
            let flag = Ident::new(&format!("{}_IS_SET", flag_name), ident.span());
            let default = field
                .attrs
                .iter()
                .find(|attr| attr.path().is_ident("optional"))
                .map(|attr| match &attr.meta {
                    Meta::Path(_) => parse_quote!(::std::default::Default::default()),
                    Meta::List(_) => {
                        let args = attr.parse_args::<ExprAssign>().expect("Invalid args");
                        assert_eq!(args.left.to_token_stream().to_string(), "default");
                        *args.right.clone()
                    },
                    _ => panic!("Invalid attribute syntax. The correct syntax is #[optional] or #[optional(default = <expr>)]"),
                });
            FieldInfo {
                ident,
                flag,
                ty,
                default,
            }
        })
        .collect()
}

struct FieldInfo {
    ident: Ident,
    flag: Ident,
    ty: syn::Type,
    default: Option<Expr>,
}

#[test]
fn test() {
    let input = parse_quote! {
        struct MyStruct {
            a: i32,
            #[optional(default = "abc".to_string())]
            b: String,
        }
    };

    let output = partial_default(input);
    println!("{}", output);
}
