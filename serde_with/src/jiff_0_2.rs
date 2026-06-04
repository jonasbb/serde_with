//! De/Serialization of [jiff] types
//!
//! This modules is only available if using the `jiff_0_2` feature of the crate.
//!
//! [jiff]: https://docs.rs/jiff/

// Serialization of large numbers can result in overflows
// The time calculations are prone to this, so lint here extra
// https://github.com/jonasbb/serde_with/issues/771
#![warn(clippy::as_conversions)]

use crate::{
    formats::{Flexible, Format, Strict, Strictness},
    prelude::*,
};
use ::jiff_0_2::{SignedDuration, Timestamp};

/// Convert a [`jiff_0_2::Timestamp`] into a [`DurationSigned`]
fn timestamp_into_duration_signed(ts: &Timestamp) -> DurationSigned {
    let dur = ts.as_duration();
    let sign = if dur.is_negative() {
        Sign::Negative
    } else {
        Sign::Positive
    };
    let dur = dur.unsigned_abs();
    DurationSigned::with_duration(sign, dur)
}

/// Convert a [`DurationSigned`] into a [`jiff_0_2::Timestamp`]
fn timestamp_from_duration_signed<'de, D>(dur: DurationSigned) -> Result<Timestamp, D::Error>
where
    D: Deserializer<'de>,
{
    let mut jiff_dur = SignedDuration::try_from(dur.duration).map_err(|msg| {
        DeError::custom(format_args!(
            "Timestamp is outside of the representable range: {msg}"
        ))
    })?;
    if dur.sign.is_negative() {
        jiff_dur = -jiff_dur;
    }

    Timestamp::from_duration(jiff_dur).map_err(|msg| {
        DeError::custom(format_args!(
            "Timestamp is outside of the representable range: {msg}"
        ))
    })
}

/// Convert a [`jiff_0_2::SignedDuration`] into a [`DurationSigned`]
fn signed_duration_into_duration_signed(dur: &SignedDuration) -> DurationSigned {
    let sign = if dur.is_negative() {
        Sign::Negative
    } else {
        Sign::Positive
    };
    let dur = dur.unsigned_abs();
    DurationSigned::with_duration(sign, dur)
}

/// Convert a [`DurationSigned`] into a [`jiff_0_2::SignedDuration`]
fn signed_duration_from_duration_signed<'de, D>(
    dur: DurationSigned,
) -> Result<SignedDuration, D::Error>
where
    D: Deserializer<'de>,
{
    let mut jiff_dur = SignedDuration::try_from(dur.duration).map_err(|msg| {
        DeError::custom(format_args!(
            "Duration is outside of the representable range: {msg}"
        ))
    })?;
    if dur.sign.is_negative() {
        jiff_dur = -jiff_dur;
    }
    Ok(jiff_dur)
}

macro_rules! use_duration_signed_ser {
    (
        $main_trait:ident $internal_trait:ident =>
        {
            $ty:ty; $converter:ident =>
            $({
                $format:ty, $strictness:ty =>
                $($tbound:ident: $bound:ident $(,)?)*
            })*
        }
    ) => {
        $(
            impl<$($tbound ,)*> SerializeAs<$ty> for $main_trait<$format, $strictness>
            where
                $($tbound: $bound,)*
            {
                fn serialize_as<S>(source: &$ty, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    let dur: DurationSigned = $converter(source);
                    $internal_trait::<$format, $strictness>::serialize_as(
                        &dur,
                        serializer,
                    )
                }
            }
        )*
    };
    (
        $( $main_trait:ident $internal_trait:ident, )+ => $rest:tt
    ) => {
        $( use_duration_signed_ser!($main_trait $internal_trait => $rest); )+
    };
}
use_duration_signed_ser!(
    DurationSeconds DurationSeconds,
    DurationMilliSeconds DurationMilliSeconds,
    DurationMicroSeconds DurationMicroSeconds,
    DurationNanoSeconds DurationNanoSeconds,
    => {
        SignedDuration; signed_duration_into_duration_signed =>
        {i64, STRICTNESS => STRICTNESS: Strictness}
        {f64, STRICTNESS => STRICTNESS: Strictness}
    }
);
#[cfg(feature = "alloc")]
use_duration_signed_ser!(
    DurationSeconds DurationSeconds,
    DurationMilliSeconds DurationMilliSeconds,
    DurationMicroSeconds DurationMicroSeconds,
    DurationNanoSeconds DurationNanoSeconds,
    => {
        SignedDuration; signed_duration_into_duration_signed =>
        {String, STRICTNESS => STRICTNESS: Strictness}
    }
);
use_duration_signed_ser!(
    TimestampSeconds DurationSeconds,
    TimestampMilliSeconds DurationMilliSeconds,
    TimestampMicroSeconds DurationMicroSeconds,
    TimestampNanoSeconds DurationNanoSeconds,
    => {
        Timestamp; timestamp_into_duration_signed =>
        {i64, STRICTNESS => STRICTNESS: Strictness}
        {f64, STRICTNESS => STRICTNESS: Strictness}
    }
);
#[cfg(feature = "alloc")]
use_duration_signed_ser!(
    TimestampSeconds DurationSeconds,
    TimestampMilliSeconds DurationMilliSeconds,
    TimestampMicroSeconds DurationMicroSeconds,
    TimestampNanoSeconds DurationNanoSeconds,
    => {
        Timestamp; timestamp_into_duration_signed =>
        {String, STRICTNESS => STRICTNESS: Strictness}
    }
);

// Duration/Timestamp WITH FRACTIONS
use_duration_signed_ser!(
    DurationSecondsWithFrac DurationSecondsWithFrac,
    DurationMilliSecondsWithFrac DurationMilliSecondsWithFrac,
    DurationMicroSecondsWithFrac DurationMicroSecondsWithFrac,
    DurationNanoSecondsWithFrac DurationNanoSecondsWithFrac,
    => {
        SignedDuration; signed_duration_into_duration_signed =>
        {f64, STRICTNESS => STRICTNESS: Strictness}
    }
);
#[cfg(feature = "alloc")]
use_duration_signed_ser!(
    DurationSecondsWithFrac DurationSecondsWithFrac,
    DurationMilliSecondsWithFrac DurationMilliSecondsWithFrac,
    DurationMicroSecondsWithFrac DurationMicroSecondsWithFrac,
    DurationNanoSecondsWithFrac DurationNanoSecondsWithFrac,
    => {
        SignedDuration; signed_duration_into_duration_signed =>
        {String, STRICTNESS => STRICTNESS: Strictness}
    }
);
use_duration_signed_ser!(
    TimestampSecondsWithFrac DurationSecondsWithFrac,
    TimestampMilliSecondsWithFrac DurationMilliSecondsWithFrac,
    TimestampMicroSecondsWithFrac DurationMicroSecondsWithFrac,
    TimestampNanoSecondsWithFrac DurationNanoSecondsWithFrac,
    => {
        Timestamp; timestamp_into_duration_signed =>
        {f64, STRICTNESS => STRICTNESS: Strictness}
    }
);
#[cfg(feature = "alloc")]
use_duration_signed_ser!(
    TimestampSecondsWithFrac DurationSecondsWithFrac,
    TimestampMilliSecondsWithFrac DurationMilliSecondsWithFrac,
    TimestampMicroSecondsWithFrac DurationMicroSecondsWithFrac,
    TimestampNanoSecondsWithFrac DurationNanoSecondsWithFrac,
    => {
        Timestamp; timestamp_into_duration_signed =>
        {String, STRICTNESS => STRICTNESS: Strictness}
    }
);

macro_rules! use_duration_signed_de {
    (
        $main_trait:ident $internal_trait:ident =>
        {
            $ty:ty; $converter:ident =>
            $({
                $format:ty, $strictness:ty =>
                $($tbound:ident: $bound:ident)*
            })*
        }
    ) =>{
        $(
            impl<'de, $($tbound,)*> DeserializeAs<'de, $ty> for $main_trait<$format, $strictness>
            where
                $($tbound: $bound,)*
            {
                fn deserialize_as<D>(deserializer: D) -> Result<$ty, D::Error>
                where
                    D: Deserializer<'de>,
                {
                    let dur: DurationSigned = $internal_trait::<$format, $strictness>::deserialize_as(deserializer)?;
                    $converter::<D>(dur)
                }
            }
        )*
    };
    (
        $( $main_trait:ident $internal_trait:ident, )+ => $rest:tt
    ) => {
        $( use_duration_signed_de!($main_trait $internal_trait => $rest); )+
    };
}

// No subsecond precision
use_duration_signed_de!(
    DurationSeconds DurationSeconds,
    DurationMilliSeconds DurationMilliSeconds,
    DurationMicroSeconds DurationMicroSeconds,
    DurationNanoSeconds DurationNanoSeconds,
    => {
        SignedDuration; signed_duration_from_duration_signed =>
        {i64, Strict =>}
        {FORMAT, Flexible => FORMAT: Format}
    }
);
#[cfg(feature = "alloc")]
use_duration_signed_de!(
    DurationSeconds DurationSeconds,
    DurationMilliSeconds DurationMilliSeconds,
    DurationMicroSeconds DurationMicroSeconds,
    DurationNanoSeconds DurationNanoSeconds,
    => {
        SignedDuration; signed_duration_from_duration_signed =>
        {String, Strict =>}
    }
);
#[cfg(feature = "std")]
use_duration_signed_de!(
    DurationSeconds DurationSeconds,
    DurationMilliSeconds DurationMilliSeconds,
    DurationMicroSeconds DurationMicroSeconds,
    DurationNanoSeconds DurationNanoSeconds,
    => {
        SignedDuration; signed_duration_from_duration_signed =>
        {f64, Strict =>}
    }
);
use_duration_signed_de!(
    TimestampSeconds DurationSeconds,
    TimestampMilliSeconds DurationMilliSeconds,
    TimestampMicroSeconds DurationMicroSeconds,
    TimestampNanoSeconds DurationNanoSeconds,
    => {
        Timestamp; timestamp_from_duration_signed =>
        {i64, Strict =>}
        {FORMAT, Flexible => FORMAT: Format}
    }
);
#[cfg(feature = "alloc")]
use_duration_signed_de!(
    TimestampSeconds DurationSeconds,
    TimestampMilliSeconds DurationMilliSeconds,
    TimestampMicroSeconds DurationMicroSeconds,
    TimestampNanoSeconds DurationNanoSeconds,
    => {
        Timestamp; timestamp_from_duration_signed =>
        {String, Strict =>}
    }
);
#[cfg(feature = "std")]
use_duration_signed_de!(
    TimestampSeconds DurationSeconds,
    TimestampMilliSeconds DurationMilliSeconds,
    TimestampMicroSeconds DurationMicroSeconds,
    TimestampNanoSeconds DurationNanoSeconds,
    => {
        Timestamp; timestamp_from_duration_signed =>
        {f64, Strict =>}
    }
);

// Duration/Timestamp WITH FRACTIONS
use_duration_signed_de!(
    DurationSecondsWithFrac DurationSecondsWithFrac,
    DurationMilliSecondsWithFrac DurationMilliSecondsWithFrac,
    DurationMicroSecondsWithFrac DurationMicroSecondsWithFrac,
    DurationNanoSecondsWithFrac DurationNanoSecondsWithFrac,
    => {
        SignedDuration; signed_duration_from_duration_signed =>
        {f64, Strict =>}
        {FORMAT, Flexible => FORMAT: Format}
    }
);
#[cfg(feature = "alloc")]
use_duration_signed_de!(
    DurationSecondsWithFrac DurationSecondsWithFrac,
    DurationMilliSecondsWithFrac DurationMilliSecondsWithFrac,
    DurationMicroSecondsWithFrac DurationMicroSecondsWithFrac,
    DurationNanoSecondsWithFrac DurationNanoSecondsWithFrac,
    => {
        SignedDuration; signed_duration_from_duration_signed =>
        {String, Strict =>}
    }
);
use_duration_signed_de!(
    TimestampSecondsWithFrac DurationSecondsWithFrac,
    TimestampMilliSecondsWithFrac DurationMilliSecondsWithFrac,
    TimestampMicroSecondsWithFrac DurationMicroSecondsWithFrac,
    TimestampNanoSecondsWithFrac DurationNanoSecondsWithFrac,
    => {
        Timestamp; timestamp_from_duration_signed =>
        {f64, Strict =>}
        {FORMAT, Flexible => FORMAT: Format}
    }
);
#[cfg(feature = "alloc")]
use_duration_signed_de!(
    TimestampSecondsWithFrac DurationSecondsWithFrac,
    TimestampMilliSecondsWithFrac DurationMilliSecondsWithFrac,
    TimestampMicroSecondsWithFrac DurationMicroSecondsWithFrac,
    TimestampNanoSecondsWithFrac DurationNanoSecondsWithFrac,
    => {
        Timestamp; timestamp_from_duration_signed =>
        {String, Strict =>}
    }
);
