//! Integration with [schemars v0.8](schemars_0_8).
//!
//! This module is only available if using the `schemars_0_8` feature of the crate.

use crate::prelude::{Schema as WrapSchema, *};
use ::schemars_0_8::{
    gen::SchemaGenerator,
    schema::{ArrayValidation, InstanceType, Schema, SchemaObject},
    JsonSchema,
};
use std::borrow::Cow;

/// A mirror of [`JsonSchema`] which also includes the type being serialized.
///
/// This is used by [`Schema`](crate::Schema) to implement [`JsonSchema`].
pub trait JsonSchemaFor<T: ?Sized> {
    /// The name of the generated JSON schema.
    ///
    /// See [`JsonSchema::schema_name`].
    fn schema_name() -> String {
        Self::schema_id().into_owned()
    }

    /// A unique string identifying the schema emitted by this type.
    ///
    /// See [`JsonSchema::schema_id`].
    fn schema_id() -> Cow<'static, str>;

    /// Generate a JSON schema for the combination of `Self` and `T`.
    ///
    /// See [`JsonSchema::json_schema`].
    fn json_schema(gen: &mut SchemaGenerator) -> Schema;

    /// Whether JSON schemas generated for this type should be re-used where
    /// possible using the `$ref` keyword.
    ///
    /// See [`JsonSchema::is_referenceable`].
    fn is_referenceable() -> bool;
}

impl<T, TA> JsonSchema for WrapSchema<T, TA>
where
    T: ?Sized,
    TA: JsonSchemaFor<T>,
{
    fn schema_name() -> String {
        <TA as JsonSchemaFor<T>>::schema_name()
    }

    fn schema_id() -> Cow<'static, str> {
        <TA as JsonSchemaFor<T>>::schema_id()
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        <TA as JsonSchemaFor<T>>::json_schema(gen)
    }

    fn is_referenceable() -> bool {
        <TA as JsonSchemaFor<T>>::is_referenceable()
    }
}

//===================================================================
// Macro helpers

macro_rules! forward_schema {
    ($fwd:ty) => {
        fn schema_name() -> String {
            <$fwd as JsonSchema>::schema_name()
        }

        fn schema_id() -> Cow<'static, str> {
            <$fwd as JsonSchema>::schema_id()
        }

        fn json_schema(gen: &mut SchemaGenerator) -> Schema {
            <$fwd as JsonSchema>::json_schema(gen)
        }

        fn is_referenceable() -> bool {
            <$fwd as JsonSchema>::is_referenceable()
        }
    };
}

//===================================================================
// Common definitions for various std types

impl<'a, T, TA> JsonSchemaFor<&'a T> for &'a TA
where
    T: ?Sized + 'a,
    TA: JsonSchemaFor<T> + 'a,
{
    forward_schema!(&'a WrapSchema<T, TA>);
}

impl<'a, T, TA> JsonSchemaFor<&'a mut T> for &'a mut TA
where
    T: ?Sized + 'a,
    TA: JsonSchemaFor<T> + 'a,
{
    forward_schema!(&'a mut WrapSchema<T, TA>);
}

impl<T, TA> JsonSchemaFor<Option<T>> for Option<TA>
where
    TA: JsonSchemaFor<T>,
{
    forward_schema!(Option<WrapSchema<T, TA>>);
}

impl<T, TA> JsonSchemaFor<Box<T>> for Box<TA>
where
    T: ?Sized,
    TA: JsonSchemaFor<T>,
{
    forward_schema!(Box<WrapSchema<T, TA>>);
}

impl<T, TA> JsonSchemaFor<Rc<T>> for Rc<TA>
where
    T: ?Sized,
    TA: JsonSchemaFor<T>,
{
    forward_schema!(Rc<WrapSchema<T, TA>>);
}

impl<T, TA> JsonSchemaFor<Arc<T>> for Arc<TA>
where
    T: ?Sized,
    TA: JsonSchemaFor<T>,
{
    forward_schema!(Arc<WrapSchema<T, TA>>);
}

impl<T, TA> JsonSchemaFor<Vec<T>> for Vec<TA>
where
    TA: JsonSchemaFor<T>,
{
    forward_schema!(Vec<WrapSchema<T, TA>>);
}

impl<T, TA> JsonSchemaFor<VecDeque<T>> for VecDeque<TA>
where
    TA: JsonSchemaFor<T>,
{
    forward_schema!(VecDeque<WrapSchema<T, TA>>);
}

// schemars only requires that V implement JsonSchema for BTreeMap<K, V>
impl<K, V, KA, VA> JsonSchemaFor<BTreeMap<K, V>> for BTreeMap<KA, VA>
where
    VA: JsonSchemaFor<V>,
{
    forward_schema!(BTreeMap<WrapSchema<K, KA>, WrapSchema<V, VA>>);
}

// schemars only requires that V implement JsonSchema for HashMap<K, V>
impl<K, V, H, KA, VA> JsonSchemaFor<HashMap<K, V, H>> for HashMap<KA, VA, H>
where
    VA: JsonSchemaFor<V>,
{
    forward_schema!(HashMap<WrapSchema<K, KA>, WrapSchema<V, VA>, H>);
}

impl<T, TA> JsonSchemaFor<BTreeSet<T>> for BTreeSet<TA>
where
    TA: JsonSchemaFor<T>,
{
    forward_schema!(BTreeSet<WrapSchema<T, TA>>);
}

impl<T, TA, H> JsonSchemaFor<HashSet<T, H>> for HashSet<TA, H>
where
    TA: JsonSchemaFor<T>,
{
    forward_schema!(HashSet<WrapSchema<T, TA>>);
}

impl<T, TA, const N: usize> JsonSchemaFor<[T; N]> for [TA; N]
where
    TA: JsonSchemaFor<T>,
{
    fn schema_name() -> String {
        std::format!("[{}; {}]", TA::schema_name(), N)
    }

    fn schema_id() -> Cow<'static, str> {
        std::format!("[{}; {}]", TA::schema_id(), N).into()
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        let (max, min) = match N.try_into() {
            Ok(len) => (Some(len), Some(len)),
            Err(_) => (None, Some(u32::MAX)),
        };

        SchemaObject {
            instance_type: Some(InstanceType::Array.into()),
            array: Some(Box::new(ArrayValidation {
                items: Some(gen.subschema_for::<WrapSchema<T, TA>>().into()),
                max_items: max,
                min_items: min,
                ..Default::default()
            })),
            ..Default::default()
        }
        .into()
    }

    fn is_referenceable() -> bool {
        false
    }
}

macro_rules! schema_for_tuple {
    (
        ( $( $ts:ident )+ )
        ( $( $as:ident )+ )
    ) => {
        impl<$($ts,)+ $($as,)+> JsonSchemaFor<( $( $ts, )+)> for ($($as,)+)
        where
            $( $as: JsonSchemaFor<$ts>, )+
        {
            forward_schema!(( $( WrapSchema<$ts, $as>, )+ ));
        }
    }
}

impl JsonSchemaFor<()> for () {
    forward_schema!(());
}

// schemars only implements JsonSchema for tuples up to 15 elements so we do
// the same here.
schema_for_tuple!((T0)(A0));
schema_for_tuple!((T0 T1) (A0 A1));
schema_for_tuple!((T0 T1 T2) (A0 A1 A2));
schema_for_tuple!((T0 T1 T2 T3) (A0 A1 A2 A3));
schema_for_tuple!((T0 T1 T2 T3 T4) (A0 A1 A2 A3 A4));
schema_for_tuple!((T0 T1 T2 T3 T4 T5) (A0 A1 A2 A3 A4 A5));
schema_for_tuple!((T0 T1 T2 T3 T4 T5 T6) (A0 A1 A2 A3 A4 A5 A6));
schema_for_tuple!((T0 T1 T2 T3 T4 T5 T6 T7) (A0 A1 A2 A3 A4 A5 A6 A7));
schema_for_tuple!((T0 T1 T2 T3 T4 T5 T6 T7 T8) (A0 A1 A2 A3 A4 A5 A6 A7 A8));
schema_for_tuple!((T0 T1 T2 T3 T4 T5 T6 T7 T8 T9) (A0 A1 A2 A3 A4 A5 A6 A7 A8 A9));
schema_for_tuple!((T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10) (A0 A1 A2 A3 A4 A5 A6 A7 A8 A9 A10));
schema_for_tuple!((T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11) (A0 A1 A2 A3 A4 A5 A6 A7 A8 A9 A10 A11));
schema_for_tuple!(
    (T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12)
    (A0 A1 A2 A3 A4 A5 A6 A7 A8 A9 A10 A11 A12)
);
schema_for_tuple!(
    (T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13)
    (A0 A1 A2 A3 A4 A5 A6 A7 A8 A9 A10 A11 A12 A13)
);
schema_for_tuple!(
    (T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14)
    (A0 A1 A2 A3 A4 A5 A6 A7 A8 A9 A10 A11 A12 A13 A14)
);
schema_for_tuple!(
    (T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15)
    (A0 A1 A2 A3 A4 A5 A6 A7 A8 A9 A10 A11 A12 A13 A14 A15)
);
