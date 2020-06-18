#![deny(
    missing_copy_implementations,
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    variant_size_differences
)]
#![warn(rust_2018_idioms)]
#![doc(test(attr(deny(
    missing_copy_implementations,
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    variant_size_differences,
))))]
#![doc(test(attr(warn(rust_2018_idioms))))]
// Not needed for 2018 edition and conflicts with `rust_2018_idioms`
#![doc(test(no_crate_inject))]
#![doc(html_root_url = "https://docs.rs/serde_with_macros/1.1.0")]

//! proc-macro extensions for [`serde_with`]
//!
//! This crate should not be used alone, but through the [`serde_with`] crate.
//!
//! [`serde_with`]: https://crates.io/crates/serde_with/

#[allow(unused_extern_crates)]
extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    parse::Parser, Attribute, Error, Field, Fields, ItemEnum, ItemStruct, Meta, NestedMeta, Path,
    Type,
};

/// Apply function on every field of structs or enums
fn apply_function_to_struct_and_enum_fields<F>(
    input: TokenStream,
    function: F,
) -> Result<proc_macro2::TokenStream, String>
where
    F: Copy,
    F: Fn(&mut Field) -> Result<(), String>,
{
    /// Handle a single struct or a single enum variant
    fn apply_on_fields<F>(fields: &mut Fields, function: F) -> Result<(), String>
    where
        F: Fn(&mut Field) -> Result<(), String>,
    {
        match fields {
            // simple, no fields, do nothing
            Fields::Unit => Ok(()),
            Fields::Named(ref mut fields) => fields.named.iter_mut().map(function).collect(),
            Fields::Unnamed(ref mut fields) => fields.unnamed.iter_mut().map(function).collect(),
        }
    }

    // For each field in the struct given by `input`, add the `skip_serializing_if` attribute,
    // if and only if, it is of type `Option`
    if let Ok(mut input) = syn::parse::<ItemStruct>(input.clone()) {
        apply_on_fields(&mut input.fields, function)?;
        Ok(quote!(#input))
    } else if let Ok(mut input) = syn::parse::<ItemEnum>(input) {
        input
            .variants
            .iter_mut()
            .map(|variant| apply_on_fields(&mut variant.fields, function))
            .collect::<Result<(), _>>()?;
        Ok(quote!(#input))
    } else {
        Err("The attribute can only be applied to struct or enum definitions.".into())
    }
}

/// Add `skip_serializing_if` annotations to [`Option`] fields.
///
/// The attribute can be added to structs and enums.
///
/// Import this attribute with `use serde_with::skip_serializing_none;`.
///
/// # Example
///
/// JSON APIs sometimes have many optional values.
/// Missing values should not be serialized, to keep the serialized format smaller.
/// Such a data type might look like:
///
/// ```rust
/// # use serde::Serialize;
/// #
/// #[derive(Serialize)]
/// struct Data {
///     #[serde(skip_serializing_if = "Option::is_none")]
///     a: Option<String>,
///     #[serde(skip_serializing_if = "Option::is_none")]
///     b: Option<u64>,
///     #[serde(skip_serializing_if = "Option::is_none")]
///     c: Option<String>,
///     #[serde(skip_serializing_if = "Option::is_none")]
///     d: Option<bool>,
/// }
/// ```
///
/// The `skip_serializing_if` annotation is repetitive and harms readability.
/// Instead the same struct can be written as:
///
/// ```rust
/// # use serde::Serialize;
/// # use serde_with_macros::skip_serializing_none;
/// #[skip_serializing_none]
/// #[derive(Serialize)]
/// struct Data {
///     a: Option<String>,
///     b: Option<u64>,
///     c: Option<String>,
///     d: Option<bool>,
/// }
/// ```
///
/// Existing `skip_serializing_if` annotations will not be altered.
///
/// If some values should always be serialized, then the `serialize_always` can be used.
///
/// # Limitations
///
/// The `serialize_always` cannot be used together with a manual `skip_serializing_if` annotations, as these conflict in their meaning.
/// A compile error will be generated if this occurs.
///
/// The `skip_serializing_none` only works if the type is called [`Option`], [`std::option::Option`], or [`core::option::Option`].
/// Type aliasing an [`Option`] and giving it another name, will cause this field to be ignored.
/// This cannot be supported, as proc-macros run before type checking, thus it is not possible to determine if a type alias refers to an [`Option`].
///
/// ```rust,ignore
/// # use serde::Serialize;
/// # use serde_with_macros::skip_serializing_none;
/// type MyOption<T> = Option<T>;
///
/// #[skip_serializing_none]
/// #[derive(Serialize)]
/// struct Data {
///     a: MyOption<String>, // This field will not be skipped
/// }
/// ```
///
/// Likewise, if you import a type and name it `Option`, the `skip_serializing_if` attributes will be added and compile errors will occur, if `Option::is_none` is not a valid function.
/// Here the function `Vec::is_none` does not exist and therefore the example fails to compile.
///
/// ```rust,compile_fail
/// # use serde::Serialize;
/// # use serde_with_macros::skip_serializing_none;
/// use std::vec::Vec as Option;
///
/// #[skip_serializing_none]
/// #[derive(Serialize)]
/// struct Data {
///     a: Option<String>,
/// }
/// ```
///
#[proc_macro_attribute]
pub fn skip_serializing_none(_args: TokenStream, input: TokenStream) -> TokenStream {
    let res = match apply_function_to_struct_and_enum_fields(
        input,
        skip_serializing_none_add_attr_to_field,
    ) {
        Ok(res) => res,
        Err(msg) => {
            let span = Span::call_site();
            Error::new(span, msg).to_compile_error()
        }
    };
    TokenStream::from(res)
}

/// Add the skip_serializing_if annotation to each field of the struct
fn skip_serializing_none_add_attr_to_field(field: &mut Field) -> Result<(), String> {
    if let Type::Path(path) = &field.ty {
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
                msg += ".";
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
            let has_attr = field
                .attrs
                .iter()
                .any(|attr| attr.path.is_ident("serialize_always"));
            if has_attr {
                return Err(
                    "`serialize_always` may only be used on fields of type `Option`.".into(),
                );
            }
        }
    }
    Ok(())
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
///
/// * `serde` is the outermost path, here namespace
/// * it contains a Meta::List
/// * which contains in another Meta a Meta::NameValue
/// * with the name being `skip_serializing_if`
#[allow(clippy::cmp_owned)]
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
                            if let Some(ident) = expr.path.get_ident() {
                                if ident.to_string() == name {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    false
}
