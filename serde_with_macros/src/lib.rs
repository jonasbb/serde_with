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
use syn::{
    parse::Parser, Attribute, Error, Field, Fields, ItemEnum, ItemStruct, Meta, NestedMeta, Path,
    Type,
};

#[proc_macro_attribute]
pub fn skip_serializing_null(_args: TokenStream, input: TokenStream) -> TokenStream {
    let res = match skip_serializing_null_do(input) {
        Ok(res) => res,
        Err(msg) => {
            let span = Span::call_site();
            Error::new(span, msg).to_compile_error()
        }
    };
    TokenStream::from(res)
}

fn skip_serializing_null_do(input: TokenStream) -> Result<proc_macro2::TokenStream, String> {
    // For each field in the struct given by `input`, add the `skip_serializing_if` attribute,
    // if and only if, it is of type `Option`
    if let Ok(mut input) = syn::parse::<ItemStruct>(input.clone()) {
        skip_serializing_null_handle_fields(&mut input.fields)?;
        Ok(quote!(#input))
    } else if let Ok(mut input) = syn::parse::<ItemEnum>(input.clone()) {
        input
            .variants
            .iter_mut()
            .map(|variant| skip_serializing_null_handle_fields(&mut variant.fields))
            .collect::<Result<(), _>>()?;
        Ok(quote!(#input))
    } else {
        Err("The attribute can only be applied to struct or enum definitions.".into())
    }
}

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

/// Determine if the `field` has an attribute with given `namespace` and `name`
///
/// On the example of
/// `#[serde(skip_serializing_if = "Option::is_none")]`
//
/// * `serde` is the outermost path, here namespace
/// * it contains a Meta::List
/// * which contains in another Meta a Meta::NameValue
/// * with the name being `skip_serializing_if`
#[cfg_attr(feature = "cargo-clippy", allow(cmp_owned))]
fn field_has_attribute(field: &Field, namespace: &str, name: &str) -> bool {
    // On the example of
    // #[serde(skip_serializing_if = "Option::is_none")]
    //
    // `serde` is the outermost path, here namespace
    // it contains a Meta::List
    // which contains in another Meta a Meta::NameValue
    // with the name being `skip_serializing_if`

    for attr in &field.attrs {
        if attr.path.is_ident(namespace) {
            // Ignore non parsable attributes, as these are not important for us
            if let Ok(expr) = attr.parse_meta() {
                if let Meta::List(expr) = expr {
                    for expr in expr.nested {
                        if let NestedMeta::Meta(Meta::NameValue(expr)) = expr {
                            if expr.ident.to_string() == name {
                                return true;
                            }
                        }
                    }
                }
            }
        }
    }
    false
}

/// Add the skip_serializing_if annotation to each field of the struct
fn skip_serializing_null_add_attr_to_field<'a>(
    fields: impl IntoIterator<Item = &'a mut Field>,
) -> Result<(), String> {
    fields.into_iter().map(|field| ->Result<(), String> {
        if let Type::Path(path) = &field.ty.clone() {
            if is_std_option(&path.path) {
                let has_skip_serializing_if =
                    field_has_attribute(&field, "serde", "skip_serializing_if");

                // Remove the `serialize_always` attribute
                let mut has_always_attr = false;
                field.attrs.retain(|attr| {
                    let has_attr = attr.path.is_ident("serialize_always");
                    has_always_attr |= has_attr;
                    !has_attr
                });

                // Error on conflicting attributes
                if has_always_attr && has_skip_serializing_if {
                    let mut msg = r#"The attributes `serialize_always` and `serde(skip_serializing_if = "...")` cannot be used on the same field"#.to_string();
                    if let Some(ident) = &field.ident {
                       msg += ": `";
                       msg += &ident.to_string();
                       msg += "`";
                    }
                        msg +=".";
                    return Err(msg);
                }

                // Do nothing if `skip_serializing_if` or `serialize_always` is already present
                if has_skip_serializing_if || has_always_attr {
                    return Ok(());
                }

                // Add the `skip_serializing_if` attribute
                let attr_tokens = quote!(
                    #[serde(skip_serializing_if = "Option::is_none")]
                );
                let parser = Attribute::parse_outer;
                let attrs = parser
                    .parse2(attr_tokens)
                    .expect("Static attr tokens should not panic");
                field.attrs.extend(attrs);
            } else {
                // Warn on use of `serialize_always` on non-Option fields
                let has_attr= field.attrs.iter().any(|attr| {
                    attr.path.is_ident("serialize_always")
                });
                if has_attr  {
                    return Err("`serialize_always` may only be used on fields of type `Option`.".into());
                }
            }
        }
        Ok(())
    }).collect()
}

/// Handle a single struct or a single enum variant
fn skip_serializing_null_handle_fields(fields: &mut Fields) -> Result<(), String> {
    match fields {
        // simple, no fields, do nothing
        Fields::Unit => Ok(()),
        Fields::Named(ref mut fields) => {
            skip_serializing_null_add_attr_to_field(fields.named.iter_mut())
        }
        Fields::Unnamed(ref mut fields) => {
            skip_serializing_null_add_attr_to_field(fields.unnamed.iter_mut())
        }
    }
}
