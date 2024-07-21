use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse_quote, punctuated::Punctuated, DeriveInput, Expr, ExprAssign, FieldValue, Fields, Ident,
    ItemMacro, Meta, Token,
};

pub fn partial_default(input: DeriveInput) -> TokenStream {
    let struct_name = input.ident;
    let fields = match input.data {
        syn::Data::Struct(data) => extract_field_info(&data.fields),
        _ => panic!("PartialDefault only works with structs"),
    };

    let rules = generate_rules(struct_name, fields);

    rules.to_token_stream()
}

fn generate_rules(name: Ident, fields: Vec<FieldInfo>) -> ItemMacro {
    let required_fields = fields
        .iter()
        .filter_map(|FieldInfo { ident, default }| {
            if default.is_none() {
                Some(ident.clone())
            } else {
                None
            }
        })
        .collect::<Punctuated<_, Token![,]>>();

    let rest = fields
        .into_iter()
        .map(|FieldInfo { ident, default }| -> FieldValue {
            let default = default.unwrap_or_else(|| {
                // Since the field may not have a default impl, we need to use a zeroed value instead This value may not be valid, but it should not cause issues as long as we insure that the invalid value from the rest strut does not actually get assigned to any field, i. e, as long as the required field check is present and insures that all fields without a default value are assigned by the user, meaning they don't get read from the rest struct.

                parse_quote!({
                    #[allow(invalid_value)]
                    unsafe {
                        ::std::mem::MaybeUninit::zeroed().assume_init()
                    }
                })
            });
            parse_quote! {
                #ident: #default
            }
        })
        .collect::<Punctuated<_, Token![,]>>();
    parse_quote! {
        macro_rules! #name {
            ($($field:ident : $value: expr),*) => {
                #name!( $($field : $value),* , )
            };
            ( $($field:ident : $value:expr),*, ) => {
                {
                    ::partial_default::check_required!([#required_fields], [$($field),*]);
                    #name {
                       $($field: $value,)*
                       ..#name {
                            #rest
                       }
                    }
                }
            };
        }
    }
}

fn extract_field_info(fields: &Fields) -> Vec<FieldInfo> {
    fields
        .iter()
        .map(|field| {
            let ident = field.ident.as_ref().unwrap().clone();
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
                default,
            }
        })
        .collect()
}

struct FieldInfo {
    ident: Ident,
    default: Option<Expr>,
}

#[test]
fn test() {
    let input = parse_quote! {
        struct Something {
            field1: i32,
            #[optional(default = 42)]
            field2: i32,
            #[optional]
            field3: i32,
        }
    };

    let output = partial_default(input);
    println!("{}", output);
}
