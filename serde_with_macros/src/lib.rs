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
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse::Parser, Attribute, Error, Field, Fields, ItemEnum, ItemStruct, Path, Type};

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

/// Add the skip_serializing_if annotation to each field of the struct
fn skip_serializing_null_add_attr_to_field<'a>(fields: impl IntoIterator<Item = &'a mut Field>) {
    fields.into_iter().for_each(|field| {
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

/// Handle a single struct or a single enum variant
fn skip_serializing_null_handle_fields(fields: &mut Fields) {
    match fields {
        // simple, no fields, do nothing
        Fields::Unit => {}
        Fields::Named(ref mut fields) => {
            skip_serializing_null_add_attr_to_field(fields.named.iter_mut())
        }
        Fields::Unnamed(ref mut fields) => {
            skip_serializing_null_add_attr_to_field(fields.unnamed.iter_mut())
        }
    };
}

#[proc_macro_attribute]
pub fn skip_serializing_null(_args: TokenStream, input: TokenStream) -> TokenStream {
    // For each field in the struct given by `input`, add the `skip_serializing_if` attribute,
    // if and only if, it is of type `Option`
    let res = if let Ok(mut input) = syn::parse::<ItemStruct>(input.clone()) {
        skip_serializing_null_handle_fields(&mut input.fields);
        quote!(#input)
    } else if let Ok(mut input) = syn::parse::<ItemEnum>(input.clone()) {
        input.variants.iter_mut().for_each(|variant| {
            skip_serializing_null_handle_fields(&mut variant.fields);
        });
        quote!(#input)
    } else {
        let span = Span::call_site();
        Error::new(
            span,
            "The attribute can only be applied to struct or enum definitions.",
        )
        .to_compile_error()
    };
    // Hand the modified input back to the compiler
    TokenStream::from(res)
}
