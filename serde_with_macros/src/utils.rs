use std::iter::Iterator;
use syn::Error;

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
