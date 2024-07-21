//!# partial-default
//!A Helper macro to allow specifying default values for some fields of a rust struct while requiring some to be initialized manually.
//!
//!## Usage
//!
//!Add `partial-default` to your crate's dependencies: `cargo add partial-default`
//!
//!1. Annotate your struct with the `PartialDefault` derive macro.
//!2. Annotate any optional fields with `#[optional]`.
//!3. If the field should have a default value other than `Default::default()`, or its type does not implement the `Default` trait, you can specify your own default value within the `#[optional(default = <value>)]`.
//!4. The macro will generate a second macro with the same name as your struct. Use this macro to initialize the struct with your specified default values
//!
//!### Example
//!```rust
//!use partial_default::PartialDefault;
//!
//!#[derive(Debug, PartialDefault)]
//!struct Example {
//!    foo: i32, // Required field
//!    #[optional]
//!    bar: i32 // Optional, default = i32::default() = 0
//!    #[optional(default = 10)]
//!    baz: i32, // Optional, default = 10
//!    
//!}
//!
//!fn example() {
//!    // Use the macro as if it was a struct declaration
//!    let example1 = Example! {
//!        foo: 1
//!        // The other fields are set to their default values
//!    };
//!
//!    println!("{:?}", example1); // Example { foo:1, bar: 0, baz: 10 }
//!
//!    let example2 = Example! {
//!        foo: 1,
//!        bar: 5
//!    };
//!
//!    println!("{:?}", example2); // Example { foo:1, bar: 5, baz: 10 }
//!
//!    let example3 = Example! {
//!        foo: 20,
//!        baz: 0 // You can override the default values
//!    };
//!
//!    println!("{:?}", example1); // Example { foo:1, bar: 0, baz: 20 }
//!
//!    let does_not_work = Example! {
//!        baz: 0  
//!    }; // Error: missing required field foo
//!
//!}
//!```
//!
//!## Limitations
//!Currently, the macro can only be placed on structs. While it would be possible to implement this approach for enums as well, the initialisation syntax would be inconsistent with regular enum initialisations as `Enum::Variant` would not be a valid macro name.
//!
//!
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
