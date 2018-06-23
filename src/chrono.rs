//! De/Serialization of [chrono][] types
//!
//! This modules is only available if using the `chrono` feature of the crate.
//!
//! [chrono]: https://docs.rs/chrono/

/// Deserialize a Unix timestamp with optional subsecond precision into a `DateTime<Utc>`.
///
/// The `DateTime<Utc>` can be serialized from an integer, a float, or a string representing a number.
///
/// # Examples
///
/// ```
/// # extern crate chrono;
/// # extern crate serde;
/// # #[macro_use]
/// # extern crate serde_derive;
/// # extern crate serde_json;
/// # extern crate serde_with;
/// # use chrono::{DateTime, Utc};
/// #[derive(Debug, Deserialize)]
/// struct S {
///     #[serde(with = "serde_with::chrono::datetime_utc_ts_seconds_from_any")]
///     date: DateTime<Utc>,
/// }
///
/// # fn main() {
/// // Deserializes integers
/// assert!(serde_json::from_str::<S>(r#"{ "date": 1478563200 }"#).is_ok());
/// // floats
/// assert!(serde_json::from_str::<S>(r#"{ "date": 1478563200.123 }"#).is_ok());
/// // and strings with numbers, for high-precision values
/// assert!(serde_json::from_str::<S>(r#"{ "date": "1478563200.123" }"#).is_ok());
/// # }
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

            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
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

use serde::{de, ser, Deserialize};
use std::marker::PhantomData;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Timestamp {
    pub seconds: i64,
    pub nanoseconds: u32,
}

impl Timestamp {
    fn round_subsecond<T: TimeUnit>(&mut self) {
        self.nanoseconds = T::round_nanosecond(self.nanoseconds);
    }
}

impl<'a> From<&'a Timestamp> for Timestamp {
    fn from(other: &'a Timestamp) -> Self {
        *other
    }
}

pub type TsSecondsWithMsAsFloat = Ts<Second, MilliSecond, Float>;

pub struct Ts<Unit, Precision, Type>(PhantomData<Unit>, PhantomData<Precision>, PhantomData<Type>)
where
    Unit: TimeUnit,
    Precision: TimeUnit,
    Precision: HigherPrecisionThan<Unit>,
    Type: SerializeType;

impl<Unit, Precision, Type> Ts<Unit, Precision, Type>
where
    Unit: TimeUnit,
    Precision: TimeUnit,
    Precision: HigherPrecisionThan<Unit>,
    Type: SerializeType,
{
    pub fn deserialize<'de, Time, D>(deserializer: D) -> Result<Time, D::Error>
    where
        Time: From<Timestamp>,
        D: de::Deserializer<'de>,
    {
        let (base, subbase) = Type::deserialize(deserializer)?;
        let mut time = Unit::from_base(base, subbase);
        time.round_subsecond::<Precision>();
        Ok(Time::from(time))
    }

    pub fn serialize<S, Time>(time: Time, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
        Time: Into<Timestamp>,
    {
        let mut time: Timestamp = time.into();
        time.round_subsecond::<Precision>();
        let (base, subbase) = Unit::to_base(time);
        Type::serialize(serializer, base, subbase)
    }
}

pub trait TimeUnit {
    fn round_nanosecond(ns: u32) -> u32;
    fn to_base(time: Timestamp) -> (i128, u32);
    fn from_base(base: i128, subbase: u32) -> Timestamp;
}
pub trait HigherPrecisionThan<TimeUnit> {}

pub struct Second;
impl TimeUnit for Second {
    fn round_nanosecond(_ns: u32) -> u32 {
        // a second does not have any subseconds
        0
    }
    fn to_base(time: Timestamp) -> (i128, u32) {
        (time.seconds as i128, time.nanoseconds)
    }
    fn from_base(base: i128, subbase: u32) -> Timestamp {
        Timestamp {
            seconds: base as i64,
            nanoseconds: subbase,
        }
    }
}
pub struct MilliSecond;
impl TimeUnit for MilliSecond {
    fn round_nanosecond(ns: u32) -> u32 {
        // a second does not have any subseconds
        ns / 1_000_000 * 1_000_000
    }
    fn to_base(time: Timestamp) -> (i128, u32) {
        let mut base = time.seconds as i128 * 1000;
        base += (time.nanoseconds as i128 * 1000) / 1_000_000_000;
        (
            base,
            ((time.nanoseconds as i128 * 1000) % 1_000_000_000) as u32,
        )
    }
    fn from_base(base: i128, subbase: u32) -> Timestamp {
        let mut nanoseconds = subbase / 1000;
        nanoseconds += (base % 1000).abs() as u32;
        let seconds = (base / 1000) as i64;
        Timestamp {
            seconds,
            nanoseconds,
        }
    }
}
impl HigherPrecisionThan<Second> for MilliSecond {}

pub trait SerializeType {
    fn deserialize<'de, D>(deserializer: D) -> Result<(i128, u32), D::Error>
    where
        D: de::Deserializer<'de>;
    fn serialize<S: ser::Serializer>(
        serializer: S,
        base: i128,
        subbase: u32,
    ) -> Result<S::Ok, S::Error>;
}

pub struct Float;
impl SerializeType for Float {
    fn deserialize<'de, D>(deserializer: D) -> Result<(i128, u32), D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let value = f64::deserialize(deserializer)?;
        Ok((
            value as i128,
            (value.fract().abs() * 1_000_000_000_f64) as u32,
        ))
    }

    fn serialize<S>(serializer: S, base: i128, subbase: u32) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut value = base as f64;
        if base >= 0 {
            value += subbase as f64 / 1_000_000_000_f64;
        } else {
            value -= subbase as f64 / 1_000_000_000_f64;
        }
        serializer.serialize_f64(value)
    }
}
