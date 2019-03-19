#![deny(
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    variant_size_differences
)]
#![cfg_attr(feature = "cargo-clippy", allow(renamed_and_removed_lints))]
#![doc(html_root_url = "https://docs.rs/serde_with_macros/1.0.0")]

extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parser, parse_macro_input, Attribute, Fields, ItemStruct, Path, Type};

/// Return `true`, if the type path refers to `std::option::Option`
///
/// Accepts
///
/// * `Option`
/// * `std::option::Option`, with or without leading `::`
/// * `core::option::Option`, with or without leading `::`
fn is_std_option(path: &Path) -> bool {
    (path.leading_colon.is_none() && path.segments.len() == 1 && path.segments[0].ident == "Option")
        || (path.segments.len() == 3
            && (path.segments[0].ident == "std" || path.segments[0].ident == "core")
            && path.segments[1].ident == "option"
            && path.segments[2].ident == "Option")
}

#[proc_macro_attribute]
pub fn skip_serializing_null(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as ItemStruct);

    // For each field in the struct given by `input`, add the `skip_serializing_if` attribute,
    // if and only if, it is of type `Option`
    match &mut input.fields {
        // simple, no fields, do nothing
        Fields::Unit => {}

        Fields::Named(ref mut fields) => {
            fields.named.iter_mut().for_each(|field| {
                if let Type::Path(path) = &field.ty {
                    if is_std_option(&path.path) {
                        // FIXME if skip_serializing_if already exists, do not add it again
                        // Add the `skip_serializing_if` attribute
                        let attr_tokens = quote!(
                            #[serde(skip_serializing_if = "Option::is_none")]
                        );
                        let parser = Attribute::parse_outer;
                        let attrs = parser
                            .parse2(attr_tokens)
                            .expect("Static attr tokens should not panic");
                        field.attrs.extend(attrs);
                    }
                }
            })
        }

        Fields::Unnamed(ref mut _fields) => unimplemented!("Unnamed"),
    };

    // Hand the modified input back to the compiler
    TokenStream::from(quote!(#input))
}
