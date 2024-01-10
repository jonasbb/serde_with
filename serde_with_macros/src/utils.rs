use core::iter::Iterator;
use darling::FromDeriveInput;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::{parse_quote, punctuated::Punctuated, Error, Generics, Path, TypeGenerics};

/// Merge multiple [`syn::Error`] into one.
pub(crate) trait IteratorExt {
    fn collect_error(self) -> Result<(), Error>
    where
        Self: Iterator<Item = Result<(), Error>> + Sized,
    {
        let accu = Ok(());
        self.fold(accu, |accu, error| match (accu, error) {
            (Ok(()), error) => error,
            (accu, Ok(())) => accu,
            (Err(mut err), Err(error)) => {
                err.combine(error);
                Err(err)
            }
        })
    }
}
impl<I> IteratorExt for I where I: Iterator<Item = Result<(), Error>> + Sized {}

/// Attributes usable for derive macros
#[derive(FromDeriveInput)]
#[darling(attributes(serde_with))]
pub(crate) struct DeriveOptions {
    /// Path to the crate
    #[darling(rename = "crate", default)]
    pub(crate) alt_crate_path: Option<Path>,
}

impl DeriveOptions {
    pub(crate) fn from_derive_input(input: &syn::DeriveInput) -> Result<Self, TokenStream> {
        match <Self as FromDeriveInput>::from_derive_input(input) {
            Ok(v) => Ok(v),
            Err(e) => Err(TokenStream::from(e.write_errors())),
        }
    }

    pub(crate) fn get_serde_with_path(&self) -> Path {
        self.alt_crate_path
            .clone()
            .unwrap_or_else(|| syn::parse_str("::serde_with").unwrap())
    }
}

// Inspired by https://github.com/serde-rs/serde/blob/fb2fe409c8f7ad6c95e3096e5e9ede865c8cfb49/serde_derive/src/de.rs#L3120
// Serde is also licensed Apache 2 + MIT
pub(crate) fn split_with_de_lifetime(
    generics: &Generics,
) -> (
    DeImplGenerics<'_>,
    TypeGenerics<'_>,
    Option<&syn::WhereClause>,
) {
    let de_impl_generics = DeImplGenerics(generics);
    let (_, ty_generics, where_clause) = generics.split_for_impl();
    (de_impl_generics, ty_generics, where_clause)
}

pub(crate) struct DeImplGenerics<'a>(&'a Generics);

impl<'a> ToTokens for DeImplGenerics<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let mut generics = self.0.clone();
        generics.params = Some(parse_quote!('de))
            .into_iter()
            .chain(generics.params)
            .collect();
        let (impl_generics, _, _) = generics.split_for_impl();
        impl_generics.to_tokens(tokens);
    }
}

/// Determine if there is a `#[derive(JsonSchema)]` on this struct.
pub(crate) fn has_derive_jsonschema(input: TokenStream) -> SchemaFieldConfig {
    /// Represents the macro body of a `#[cfg_attr]` attribute.
    ///
    /// ```text
    /// #[cfg_attr(feature = "things", derive(Macro))]
    ///            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    /// ```
    struct CfgAttr {
        cfg: syn::Expr,
        _comma: syn::Token![,],
        meta: syn::Meta,
    }

    impl syn::parse::Parse for CfgAttr {
        fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
            Ok(Self {
                cfg: input.parse()?,
                _comma: input.parse()?,
                meta: input.parse()?,
            })
        }
    }

    fn parse_derive_args(
        input: syn::parse::ParseStream<'_>,
    ) -> syn::Result<Punctuated<Path, syn::Token![,]>> {
        Punctuated::parse_terminated_with(input, Path::parse_mod_style)
    }

    fn eval_attribute(attr: &syn::Attribute) -> syn::Result<SchemaFieldConfig> {
        let args: CfgAttr;
        let mut meta = &attr.meta;
        let mut config = SchemaFieldConfig::Unconditional;

        if meta.path().is_ident("cfg_attr") {
            let list = meta.require_list()?;
            args = list.parse_args()?;

            meta = &args.meta;
            config = SchemaFieldConfig::Conditional(args.cfg);
        }

        let list = meta.require_list()?;
        if !list.path.is_ident("derive") {
            return Ok(SchemaFieldConfig::Disabled);
        }

        let derives = list.parse_args_with(parse_derive_args)?;
        for derive in &derives {
            let segments = &derive.segments;

            // Check for $(::)? $(schemars::)? JsonSchema
            match segments.len() {
                1 if segments[0].ident == "JsonSchema" => (),
                2 if segments[0].ident == "schemars" && segments[1].ident == "JsonSchema" => (),
                _ => continue,
            }

            return Ok(config);
        }

        Ok(SchemaFieldConfig::Disabled)
    }

    let input: syn::DeriveInput = match syn::parse(input) {
        Ok(input) => input,
        Err(_) => return SchemaFieldConfig::Disabled,
    };

    for attr in input.attrs {
        match eval_attribute(&attr) {
            Ok(SchemaFieldConfig::Disabled) => continue,
            Ok(config) => return config,
            Err(_) => continue,
        }
    }

    SchemaFieldConfig::Disabled
}

/// Enum controlling when we should emit a `#[schemars]` field attribute.
pub(crate) enum SchemaFieldConfig {
    /// Emit a `#[cfg_attr(#cfg, schemars(...))]` attribute.
    Conditional(syn::Expr),
    /// Emit a `#[schemars(...)]` attribute (or equivalent).
    Unconditional,
    /// Do not emit an attribute.
    Disabled,
}

impl SchemaFieldConfig {
    /// Get a `#[cfg]` expression suitable for emitting the `#[schemars]` attribute.
    ///
    /// If this config is `Unconditional` then it will just return `all()` which
    /// is always true.
    pub(crate) fn cfg_expr(&self) -> Option<syn::Expr> {
        match self {
            Self::Unconditional => Some(syn::parse_quote!(all())),
            Self::Conditional(cfg) => Some(cfg.clone()),
            Self::Disabled => None,
        }
    }
}
