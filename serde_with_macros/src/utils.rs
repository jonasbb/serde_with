use darling::FromDeriveInput;
use proc_macro::TokenStream;
use std::iter::Iterator;
use syn::{Error, Path};

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
#[derive(FromDeriveInput, Debug)]
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
