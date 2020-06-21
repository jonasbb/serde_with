//! Module for [`SerializeAs`][] implementations
//!
//! The module contains the [`SerializeAs`][] trait and helper code.
//! Additionally, it contains implementations of [`SerializeAs`][] for types defined in the Rust Standard Library or this crate.
//!
//! You can find more details on how to implement this trait for your types in the documentation of the [`SerializeAs`][] trait and details about the usage in the [user guide][].
//!
//! [user guide]: serde_with::guide

mod impls;

use super::*;

/// A **data structure** that can be serialized into any data format supported by Serde, analoge to [`Serialize`].
///
/// The trait is analoge to the [`serde::Serialize`][`Serialize`] trait, with the same meaning of input and output arguments.
/// It can and should the implemented using the same code structure as the [`Serialize`] trait.
/// As such, the same advice for [implementing `Serialize`][impl-serialize] applies here.
///
/// # Differences to [`Deserialize`]
///
/// The trait is only required for container-like types or types implementing specific conversion functions.
/// Container-like types are [`Vec`][], [`BTreeMap`][], but also [`Option`][] and [`Box`][].
/// Conversion types serialize into a different serde data type.
/// For example, [`DisplayFromStr`] uses the [`Display`] trait to serialize a String and [`DurationSeconds`] converts a [`Duration`] into either String or integer values.
///
/// This code shows how to implement [`Serialize`] for [`Box`]:
///
/// ```rust,ignore
/// impl<T> Serialize for Box<T>
/// where
///     T: Serialize,
/// {
///     #[inline]
///     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
///     where
///         S: Serializer,
///     {
///         (**self).serialize(serializer)
///     }
/// }
/// ```
///
/// and this code shows how to do the same using [`SerializeAs`][]:
///
/// ```rust,ignore
/// impl<T, U> SerializeAs<Box<T>> for Box<U>
/// where
///     U: SerializeAs<T>,
/// {
///     fn serialize_as<S>(source: &Box<T>, serializer: S) -> Result<S::Ok, S::Error>
///     where
///         S: Serializer,
///     {
///         SerializeAsWrap::<T, U>::new(source).serialize(serializer)
///     }
/// }
/// ```
///
/// It uses two type parameters, `T` and `U` instead of only one and performs the serialization step using the `SerialzieAsWrap` type.
/// The `T` type is the on the Rust side before serialization, whereas the `U` type determines how the value will be serialized.
/// These two changes are usually enough to make a container type implement [`SerializeAs`][].
///
/// [`BTreeMap`]: std::collections::BTreeMap
/// [`Deserialize`]: serde::Deserialize
/// [`Display`]: std::fmt::Display
/// [`Duration`]: std::time::Duration
/// [impl-serialize]: https://serde.rs/impl-serialize.html
pub trait SerializeAs<T> {
    /// Serialize this value into the given Serde serializer.
    fn serialize_as<S>(source: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer;
}

/// Helper type to implement [`SerializeAs`] for container-like types.
#[derive(Debug)]
pub struct SerializeAsWrap<'a, T, U> {
    value: &'a T,
    marker: PhantomData<U>,
}

impl<'a, T, U> SerializeAsWrap<'a, T, U> {
    /// Create new instance with provided value.
    pub fn new(value: &'a T) -> Self {
        Self {
            value,
            marker: PhantomData,
        }
    }
}

impl<'a, T, U> Serialize for SerializeAsWrap<'a, T, U>
where
    U: SerializeAs<T>,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        U::serialize_as(self.value, serializer)
    }
}

impl<'a, T, U> From<&'a T> for SerializeAsWrap<'a, T, U>
where
    U: SerializeAs<T>,
{
    fn from(value: &'a T) -> Self {
        Self::new(value)
    }
}
