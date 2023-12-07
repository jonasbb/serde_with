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

impl<'a, T: 'a, TA: 'a> JsonSchema for WrapSchema<&'a T, &'a TA>
where
    T: ?Sized,
    WrapSchema<T, TA>: JsonSchema,
{
    forward_schema!(&'a WrapSchema<T, TA>);
}

impl<'a, T: 'a, TA: 'a> JsonSchema for WrapSchema<&'a mut T, &'a mut TA>
where
    T: ?Sized,
    WrapSchema<T, TA>: JsonSchema,
{
    forward_schema!(&'a mut WrapSchema<T, TA>);
}

impl<T, TA> JsonSchema for WrapSchema<Option<T>, Option<TA>>
where
    WrapSchema<T, TA>: JsonSchema,
{
    forward_schema!(Option<WrapSchema<T, TA>>);
}

impl<T, TA> JsonSchema for WrapSchema<Box<T>, Box<TA>>
where
    T: ?Sized,
    WrapSchema<T, TA>: JsonSchema,
{
    forward_schema!(Box<WrapSchema<T, TA>>);
}

impl<T, TA> JsonSchema for WrapSchema<Rc<T>, Rc<TA>>
where
    T: ?Sized,
    WrapSchema<T, TA>: JsonSchema,
{
    forward_schema!(Rc<WrapSchema<T, TA>>);
}

impl<T, TA> JsonSchema for WrapSchema<Arc<T>, Arc<TA>>
where
    T: ?Sized,
    WrapSchema<T, TA>: JsonSchema,
{
    forward_schema!(Arc<WrapSchema<T, TA>>);
}

impl<T, TA> JsonSchema for WrapSchema<Vec<T>, Vec<TA>>
where
    WrapSchema<T, TA>: JsonSchema,
{
    forward_schema!(Vec<WrapSchema<T, TA>>);
}

impl<T, TA> JsonSchema for WrapSchema<VecDeque<T>, VecDeque<TA>>
where
    WrapSchema<T, TA>: JsonSchema,
{
    forward_schema!(VecDeque<WrapSchema<T, TA>>);
}

// schemars only requires that V implement JsonSchema for BTreeMap<K, V>
impl<K, V, KA, VA> JsonSchema for WrapSchema<BTreeMap<K, V>, BTreeMap<KA, VA>>
where
    WrapSchema<V, VA>: JsonSchema,
{
    forward_schema!(BTreeMap<WrapSchema<K, KA>, WrapSchema<V, VA>>);
}

// schemars only requires that V implement JsonSchema for HashMap<K, V>
impl<K, V, S, KA, VA> JsonSchema for WrapSchema<HashMap<K, V, S>, HashMap<KA, VA, S>>
where
    WrapSchema<V, VA>: JsonSchema,
{
    forward_schema!(HashMap<WrapSchema<K, KA>, WrapSchema<V, VA>, S>);
}

impl<T, TA> JsonSchema for WrapSchema<BTreeSet<T>, BTreeSet<TA>>
where
    WrapSchema<T, TA>: JsonSchema,
{
    forward_schema!(BTreeSet<WrapSchema<T, TA>>);
}

impl<T, TA, H> JsonSchema for WrapSchema<HashSet<T, H>, HashSet<TA, H>>
where
    WrapSchema<T, TA>: JsonSchema,
{
    forward_schema!(HashSet<WrapSchema<T, TA>, H>);
}

impl<T, TA, const N: usize> JsonSchema for WrapSchema<[T; N], [TA; N]>
where
    WrapSchema<T, TA>: JsonSchema,
{
    fn schema_name() -> String {
        std::format!("[{}; {}]", <WrapSchema<T, TA>>::schema_name(), N)
    }

    fn schema_id() -> Cow<'static, str> {
        std::format!("[{}; {}]", <WrapSchema<T, TA>>::schema_id(), N).into()
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
        impl<$($ts,)+ $($as,)+> JsonSchema for WrapSchema<($($ts,)+), ($($as,)+)>
        where
            $( WrapSchema<$ts, $as>: JsonSchema, )+
        {
            forward_schema!(( $( WrapSchema<$ts, $as>, )+ ));
        }
    }
}

impl JsonSchema for WrapSchema<(), ()> {
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

//===================================================================
// Impls for serde_with types.

impl<T: JsonSchema> JsonSchema for WrapSchema<T, Same> {
    forward_schema!(T);
}

impl<T> JsonSchema for WrapSchema<T, DisplayFromStr> {
    forward_schema!(String);
}

impl<'a, T: 'a> JsonSchema for WrapSchema<Cow<'a, T>, BorrowCow>
where
    T: ?Sized + ToOwned,
    Cow<'a, T>: JsonSchema,
{
    forward_schema!(Cow<'a, T>);
}

impl<T> JsonSchema for WrapSchema<T, Bytes> {
    forward_schema!(Vec<u8>);
}

impl<T, TA> JsonSchema for WrapSchema<T, DefaultOnError<TA>>
where
    WrapSchema<T, TA>: JsonSchema,
{
    forward_schema!(WrapSchema<T, TA>);
}

impl<T, TA> JsonSchema for WrapSchema<T, DefaultOnNull<TA>>
where
    WrapSchema<T, TA>: JsonSchema,
{
    forward_schema!(Option<WrapSchema<T, TA>>);
}

impl<T: JsonSchema> JsonSchema for WrapSchema<T, FromInto<T>> {
    forward_schema!(T);
}

impl<T: JsonSchema> JsonSchema for WrapSchema<T, FromIntoRef<T>> {
    forward_schema!(T);
}

macro_rules! schema_for_map {
    ($type:ty) => {
        impl<K, V, KA, VA> JsonSchema for WrapSchema<$type, Map<KA, VA>>
        where
            WrapSchema<V, VA>: JsonSchema,
        {
            forward_schema!(WrapSchema<BTreeMap<K, V>, BTreeMap<KA, VA>>);
        }
    };
}

schema_for_map!([(K, V)]);
schema_for_map!(BTreeSet<(K, V)>);
schema_for_map!(BinaryHeap<(K, V)>);
schema_for_map!(Box<[(K, V)]>);
schema_for_map!(LinkedList<(K, V)>);
schema_for_map!(Vec<(K, V)>);
schema_for_map!(VecDeque<(K, V)>);

impl<K, V, S, KA, VA> JsonSchema for WrapSchema<HashSet<(K, V), S>, Map<KA, VA>>
where
    WrapSchema<V, VA>: JsonSchema,
{
    forward_schema!(WrapSchema<BTreeMap<K, V>, BTreeMap<KA, VA>>);
}

macro_rules! map_first_last_wins_schema {
    ($(=> $extra:ident)? $type:ty) => {
        impl<K, V, $($extra,)? KA, VA> JsonSchema for WrapSchema<$type, MapFirstKeyWins<KA, VA>>
        where
            WrapSchema<V, VA>: JsonSchema
        {
            forward_schema!(BTreeMap<WrapSchema<K, KA>, WrapSchema<V, VA>>);
        }

        impl<K, V, $($extra,)? KA, VA> JsonSchema for WrapSchema<$type, MapPreventDuplicates<KA, VA>>
        where
            WrapSchema<V, VA>: JsonSchema
        {
            forward_schema!(BTreeMap<WrapSchema<K, KA>, WrapSchema<V, VA>>);
        }
    }
}

map_first_last_wins_schema!(BTreeMap<K, V>);
map_first_last_wins_schema!(=> S HashMap<K, V, S>);
#[cfg(feature = "hashbrown_0_14")]
map_first_last_wins_schema!(=> S hashbrown_0_14::HashMap<K, V, S>);
#[cfg(feature = "indexmap_1")]
map_first_last_wins_schema!(=> S indexmap_1::IndexMap<K, V, S>);
#[cfg(feature = "indexmap_2")]
map_first_last_wins_schema!(=> S indexmap_2::IndexMap<K, V, S>);

impl<T, TA> JsonSchema for WrapSchema<T, SetLastValueWins<TA>>
where
    WrapSchema<T, TA>: JsonSchema,
{
    forward_schema!(WrapSchema<T, TA>);
}

impl<T, TA> JsonSchema for WrapSchema<T, SetPreventDuplicates<TA>>
where
    WrapSchema<T, TA>: JsonSchema,
{
    forward_schema!(WrapSchema<T, TA>);
}

impl<SEP, T, TA> JsonSchema for WrapSchema<T, StringWithSeparator<SEP, TA>>
where
    SEP: Separator,
{
    forward_schema!(String);
}
