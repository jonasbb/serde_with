//! De/Serialization of [chrono][] types
//!
//! This modules is only available if using the `chrono` feature of the crate.
//!
//! [chrono]: https://docs.rs/chrono/

use crate::{
    de::DeserializeAs,
    formats::{Flexible, Format, Strict, Strictness},
    ser::SerializeAs,
    utils, DurationSeconds, DurationSecondsWithFrac,
};
use chrono_crate::{DateTime, Duration, NaiveDateTime, Utc};
use serde::{
    de::{Error, Unexpected, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::fmt;
use utils::NANOS_PER_SEC;

/// Deserialize a Unix timestamp with optional subsecond precision into a `DateTime<Utc>`.
///
/// The `DateTime<Utc>` can be serialized from an integer, a float, or a string representing a number.
///
/// # Examples
///
/// ```
/// # use chrono_crate::{DateTime, Utc};
/// # use serde_derive::Deserialize;
/// #
/// #[derive(Debug, Deserialize)]
/// struct S {
///     #[serde(with = "serde_with::chrono::datetime_utc_ts_seconds_from_any")]
///     date: DateTime<Utc>,
/// }
///
/// // Deserializes integers
/// assert!(serde_json::from_str::<S>(r#"{ "date": 1478563200 }"#).is_ok());
/// // floats
/// assert!(serde_json::from_str::<S>(r#"{ "date": 1478563200.123 }"#).is_ok());
/// // and strings with numbers, for high-precision values
/// assert!(serde_json::from_str::<S>(r#"{ "date": "1478563200.123" }"#).is_ok());
/// ```
///
pub mod datetime_utc_ts_seconds_from_any {
    use chrono_crate::{DateTime, NaiveDateTime, Utc};
    use serde::de::{Deserializer, Error, Unexpected, Visitor};

    /// Deserialize a Unix timestamp with optional subsecond precision into a `DateTime<Utc>`.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Helper;
        impl<'de> Visitor<'de> for Helper {
            type Value = DateTime<Utc>;

            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("Invalid timestamp. Must be an integer, float, or string with optional subsecond precision.")
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                let ndt = NaiveDateTime::from_timestamp_opt(value, 0);
                if let Some(ndt) = ndt {
                    Ok(DateTime::<Utc>::from_utc(ndt, Utc))
                } else {
                    Err(Error::custom(format!(
                        "Invalid or out of range value '{}' for DateTime",
                        value
                    )))
                }
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                let ndt = NaiveDateTime::from_timestamp_opt(value as i64, 0);
                if let Some(ndt) = ndt {
                    Ok(DateTime::<Utc>::from_utc(ndt, Utc))
                } else {
                    Err(Error::custom(format!(
                        "Invalid or out of range value '{}' for DateTime",
                        value
                    )))
                }
            }

            fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                let seconds = value.trunc() as i64;
                let nsecs = (value.fract() * 1_000_000_000_f64).abs() as u32;
                let ndt = NaiveDateTime::from_timestamp_opt(seconds, nsecs);
                if let Some(ndt) = ndt {
                    Ok(DateTime::<Utc>::from_utc(ndt, Utc))
                } else {
                    Err(Error::custom(format!(
                        "Invalid or out of range value '{}' for DateTime",
                        value
                    )))
                }
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                let parts: Vec<_> = value.split('.').collect();

                match *parts.as_slice() {
                    [seconds] => {
                        if let Ok(seconds) = i64::from_str_radix(seconds, 10) {
                            let ndt = NaiveDateTime::from_timestamp_opt(seconds, 0);
                            if let Some(ndt) = ndt {
                                Ok(DateTime::<Utc>::from_utc(ndt, Utc))
                            } else {
                                Err(Error::custom(format!(
                                    "Invalid or out of range value '{}' for DateTime",
                                    value
                                )))
                            }
                        } else {
                            Err(Error::invalid_value(Unexpected::Str(value), &self))
                        }
                    }
                    [seconds, subseconds] => {
                        if let Ok(seconds) = i64::from_str_radix(seconds, 10) {
                            let subseclen = subseconds.chars().count() as u32;
                            if subseclen > 9 {
                                return Err(Error::custom(format!(
                                    "DateTimes only support nanosecond precision but '{}' has more than 9 digits.",
                                    value
                                )));
                            }

                            if let Ok(mut subseconds) = u32::from_str_radix(subseconds, 10) {
                                // convert subseconds to nanoseconds (10^-9), require 9 places for nanoseconds
                                subseconds *= 10u32.pow(9 - subseclen);
                                let ndt = NaiveDateTime::from_timestamp_opt(seconds, subseconds);
                                if let Some(ndt) = ndt {
                                    Ok(DateTime::<Utc>::from_utc(ndt, Utc))
                                } else {
                                    Err(Error::custom(format!(
                                        "Invalid or out of range value '{}' for DateTime",
                                        value
                                    )))
                                }
                            } else {
                                Err(Error::invalid_value(Unexpected::Str(value), &self))
                            }
                        } else {
                            Err(Error::invalid_value(Unexpected::Str(value), &self))
                        }
                    }

                    _ => Err(Error::invalid_value(Unexpected::Str(value), &self)),
                }
            }
        }

        deserializer.deserialize_any(Helper)
    }
}

impl SerializeAs<NaiveDateTime> for DateTime<Utc> {
    fn serialize_as<S>(source: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let datetime = DateTime::<Utc>::from_utc(*source, Utc);
        datetime.serialize(serializer)
    }
}

impl<'de> DeserializeAs<'de, NaiveDateTime> for DateTime<Utc> {
    fn deserialize_as<D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        DateTime::<Utc>::deserialize(deserializer).map(|datetime| datetime.naive_utc())
    }
}

fn duration_subsec_nanos(mut dur: Duration) -> u32 {
    if dur < Duration::zero() {
        dur = Duration::zero() - dur;
    }
    (dur - Duration::seconds(dur.num_seconds()))
        .num_nanoseconds()
        .unwrap() as u32
}

fn duration_as_secs_f64(dur: Duration) -> f64 {
    let mut secs = dur.num_seconds();
    let subsecs = duration_subsec_nanos(dur);

    // Properly round the value
    if dur < Duration::zero() && subsecs > 0 {
        secs -= 1;
    }

    (secs as f64) + (subsecs as f64) / (NANOS_PER_SEC as f64)
}

#[allow(clippy::float_cmp)]
#[test]
fn test_duration_as_secs_f64() {
    assert_eq!(duration_as_secs_f64(Duration::seconds(1)), 1.);
    assert_eq!(
        duration_as_secs_f64(Duration::nanoseconds(500_000_000)),
        0.5
    );
    assert_eq!(
        duration_as_secs_f64(Duration::nanoseconds(-500_000_000)),
        -0.5
    );
}

impl<STRICTNESS> SerializeAs<Duration> for DurationSeconds<i64, STRICTNESS>
where
    STRICTNESS: Strictness,
{
    fn serialize_as<S>(source: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut secs = source.num_seconds();
        let subsecs = duration_subsec_nanos(*source);

        // Properly round the value
        if subsecs >= 500_000_000 {
            if *source < Duration::zero() {
                secs -= 1;
            } else {
                secs += 1;
            }
        }
        secs.serialize(serializer)
    }
}

impl<STRICTNESS> SerializeAs<Duration> for DurationSeconds<f64, STRICTNESS>
where
    STRICTNESS: Strictness,
{
    fn serialize_as<S>(source: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        duration_as_secs_f64(*source).round().serialize(serializer)
    }
}

impl<STRICTNESS> SerializeAs<Duration> for DurationSeconds<String, STRICTNESS>
where
    STRICTNESS: Strictness,
{
    fn serialize_as<S>(source: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut secs = source.num_seconds();
        let subsecs = duration_subsec_nanos(*source);

        // Properly round the value
        if subsecs >= 500_000_000 {
            if *source < Duration::zero() {
                secs -= 1;
            } else {
                secs += 1;
            }
        }
        secs.to_string().serialize(serializer)
    }
}

impl<STRICTNESS> SerializeAs<Duration> for DurationSecondsWithFrac<f64, STRICTNESS>
where
    STRICTNESS: Strictness,
{
    fn serialize_as<S>(source: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        duration_as_secs_f64(*source).serialize(serializer)
    }
}

impl<STRICTNESS> SerializeAs<Duration> for DurationSecondsWithFrac<String, STRICTNESS>
where
    STRICTNESS: Strictness,
{
    fn serialize_as<S>(source: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        duration_as_secs_f64(*source)
            .to_string()
            .serialize(serializer)
    }
}
fn duration_from_secs_f64(secs: f64) -> Result<Duration, String> {
    const MAX_NANOS_F64: f64 = ((i64::max_value() as i128 + 1) * (NANOS_PER_SEC as i128)) as f64;
    // TODO why are the seconds converted to nanoseconds first?
    // Does it make sense to just truncate the value?
    let nanos = secs * (NANOS_PER_SEC as f64);
    if !nanos.is_finite() {
        return Err("got non-finite value when converting float to duration".into());
    }
    if nanos >= MAX_NANOS_F64 || nanos < -MAX_NANOS_F64 {
        return Err("overflow when converting float to duration".into());
    }
    let nanos = nanos as i128;
    let secs = Duration::seconds((nanos / (NANOS_PER_SEC as i128)) as i64);
    let subsec = Duration::nanoseconds((nanos % (NANOS_PER_SEC as i128)) as i64);
    Ok(secs + subsec)
}

struct DurationVisitiorFlexible;
impl<'de> Visitor<'de> for DurationVisitiorFlexible {
    type Value = Duration;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> ::std::fmt::Result {
        formatter.write_str("an integer, a float, or a string containing a number")
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Duration::seconds(value))
    }

    fn visit_u64<E>(self, secs: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        if secs <= i64::max_value() as u64 {
            Ok(Duration::seconds(secs as i64))
        } else {
            Err(Error::custom(format!(
                "Seconds larger than {} are not supported for chrono::Duration. Found {}",
                i64::max_value(),
                secs,
            )))
        }
    }

    fn visit_f64<E>(self, secs: f64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        duration_from_secs_f64(secs).map_err(Error::custom)
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let parts: Vec<_> = value.split('.').collect();

        match *parts.as_slice() {
            [seconds] => {
                if let Ok(seconds) = i64::from_str_radix(seconds, 10) {
                    Ok(Duration::seconds(seconds))
                } else {
                    Err(Error::invalid_value(Unexpected::Str(value), &self))
                }
            }
            [seconds, subseconds] => {
                if let Ok(mut seconds) = i64::from_str_radix(seconds, 10) {
                    let subseclen = subseconds.chars().count() as u32;
                    if subseclen > 9 {
                        return Err(Error::custom(format!(
                                    "Duration only support nanosecond precision but '{}' has more than 9 digits.",
                                    value
                                )));
                    }

                    if let Ok(mut subseconds) = u32::from_str_radix(subseconds, 10) {
                        // convert subseconds to nanoseconds (10^-9), require 9 places for nanoseconds
                        subseconds *= 10u32.pow(9 - subseclen);

                        // Check if first char of seconds part is negative sign
                        if parts[0].starts_with('-') {
                            seconds -= 1;
                        }

                        Ok(Duration::seconds(seconds)
                            + Duration::nanoseconds(i64::from(subseconds)))
                    } else {
                        Err(Error::invalid_value(Unexpected::Str(value), &self))
                    }
                } else {
                    Err(Error::invalid_value(Unexpected::Str(value), &self))
                }
            }

            _ => Err(Error::invalid_value(Unexpected::Str(value), &self)),
        }
    }
}

impl<'de> DeserializeAs<'de, Duration> for DurationSeconds<i64, Strict> {
    fn deserialize_as<D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        i64::deserialize(deserializer).map(Duration::seconds)
    }
}

impl<'de> DeserializeAs<'de, Duration> for DurationSeconds<f64, Strict> {
    fn deserialize_as<D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val = f64::deserialize(deserializer)?;
        duration_from_secs_f64(val).map_err(Error::custom)
    }
}

impl<'de> DeserializeAs<'de, Duration> for DurationSeconds<String, Strict> {
    fn deserialize_as<D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        crate::rust::display_fromstr::deserialize(deserializer).map(Duration::seconds)
    }
}

impl<'de, FORMAT> DeserializeAs<'de, Duration> for DurationSeconds<FORMAT, Flexible>
where
    FORMAT: Format,
{
    fn deserialize_as<D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(DurationVisitiorFlexible)
    }
}

impl<'de> DeserializeAs<'de, Duration> for DurationSecondsWithFrac<i64, Strict> {
    fn deserialize_as<D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        i64::deserialize(deserializer).map(Duration::seconds)
    }
}

impl<'de> DeserializeAs<'de, Duration> for DurationSecondsWithFrac<f64, Strict> {
    fn deserialize_as<D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val = f64::deserialize(deserializer)?;
        duration_from_secs_f64(val).map_err(Error::custom)
    }
}

impl<'de> DeserializeAs<'de, Duration> for DurationSecondsWithFrac<String, Strict> {
    fn deserialize_as<D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let dur = String::deserialize(deserializer)?;
        DurationVisitiorFlexible.visit_str(&*dur)
        // crate::rust::display_fromstr::deserialize(deserializer)
        //     .and_then(|val| duration_from_secs_f64(val).map_err(Error::custom))
    }
}

impl<'de, FORMAT> DeserializeAs<'de, Duration> for DurationSecondsWithFrac<FORMAT, Flexible>
where
    FORMAT: Format,
{
    fn deserialize_as<D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(DurationVisitiorFlexible)
    }
}
