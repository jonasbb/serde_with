//! Internal Helper types

use crate::{
    formats::{Flexible, Format, Strict, Strictness},
    utils, DeserializeAs, DurationMicroSeconds, DurationMicroSecondsWithFrac, DurationMilliSeconds,
    DurationMilliSecondsWithFrac, DurationNanoSeconds, DurationNanoSecondsWithFrac,
    DurationSeconds, DurationSecondsWithFrac, SerializeAs,
};
use serde::{
    de::{self, Unexpected, Visitor},
    ser, Deserialize, Deserializer, Serialize, Serializer,
};
use std::{
    fmt,
    ops::Neg,
    time::{Duration, SystemTime},
};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum Sign {
    Positive,
    Negative,
}

impl Sign {
    #[allow(dead_code)]
    pub(crate) fn is_positive(&self) -> bool {
        *self == Sign::Positive
    }

    #[allow(dead_code)]
    pub(crate) fn is_negative(&self) -> bool {
        *self == Sign::Negative
    }

    pub(crate) fn apply<T>(&self, value: T) -> T
    where
        T: Neg<Output = T>,
    {
        match *self {
            Sign::Positive => value,
            Sign::Negative => value.neg(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct DurationSigned {
    pub(crate) sign: Sign,
    pub(crate) duration: Duration,
}

impl DurationSigned {
    pub(crate) fn new(sign: Sign, secs: u64, nanosecs: u32) -> Self {
        Self {
            sign,
            duration: Duration::new(secs, nanosecs),
        }
    }

    #[cfg(feature = "chrono")]
    pub(crate) fn with_duration(sign: Sign, duration: Duration) -> Self {
        Self { sign, duration }
    }

    pub(crate) fn to_system_time<'de, D>(self) -> Result<SystemTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        match self.sign {
            Sign::Positive => SystemTime::UNIX_EPOCH.checked_add(self.duration),
            Sign::Negative => SystemTime::UNIX_EPOCH.checked_sub(self.duration),
        }
        .ok_or_else(|| {
            de::Error::custom("timestamp is outside the range for std::time::SystemTime")
        })
    }

    pub(crate) fn to_std_duration<'de, D>(self) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        match self.sign {
            Sign::Positive => Ok(self.duration),
            Sign::Negative => Err(de::Error::custom("std::time::Duration cannot be negative")),
        }
    }
}

impl From<&Duration> for DurationSigned {
    fn from(&duration: &Duration) -> Self {
        Self {
            sign: Sign::Positive,
            duration,
        }
    }
}

impl From<&SystemTime> for DurationSigned {
    fn from(time: &SystemTime) -> Self {
        match time.duration_since(SystemTime::UNIX_EPOCH) {
            Ok(dur) => DurationSigned {
                sign: Sign::Positive,
                duration: dur,
            },
            Err(err) => DurationSigned {
                sign: Sign::Negative,
                duration: err.duration(),
            },
        }
    }
}

impl std::ops::Mul<u32> for DurationSigned {
    type Output = DurationSigned;

    fn mul(mut self, rhs: u32) -> Self::Output {
        self.duration *= rhs;
        self
    }
}

impl std::ops::Div<u32> for DurationSigned {
    type Output = DurationSigned;

    fn div(mut self, rhs: u32) -> Self::Output {
        self.duration /= rhs;
        self
    }
}

impl<STRICTNESS> SerializeAs<DurationSigned> for DurationSeconds<u64, STRICTNESS>
where
    STRICTNESS: Strictness,
{
    fn serialize_as<S>(source: &DurationSigned, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if source.sign.is_negative() {
            return Err(ser::Error::custom(
                "cannot serialize a negative Duration as u64",
            ));
        }

        let mut secs = source.duration.as_secs();

        // Properly round the value
        if source.duration.subsec_millis() >= 500 {
            if source.sign.is_positive() {
                secs += 1;
            } else {
                secs -= 1;
            }
        }
        secs.serialize(serializer)
    }
}

impl<STRICTNESS> SerializeAs<DurationSigned> for DurationSeconds<i64, STRICTNESS>
where
    STRICTNESS: Strictness,
{
    fn serialize_as<S>(source: &DurationSigned, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut secs = source.sign.apply(source.duration.as_secs() as i64);

        // Properly round the value
        if source.duration.subsec_millis() >= 500 {
            if source.sign.is_positive() {
                secs += 1;
            } else {
                secs -= 1;
            }
        }
        secs.serialize(serializer)
    }
}

impl<STRICTNESS> SerializeAs<DurationSigned> for DurationSeconds<f64, STRICTNESS>
where
    STRICTNESS: Strictness,
{
    fn serialize_as<S>(source: &DurationSigned, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut secs = source.sign.apply(source.duration.as_secs() as f64);

        // Properly round the value
        if source.duration.subsec_millis() >= 500 {
            if source.sign.is_positive() {
                secs += 1.;
            } else {
                secs -= 1.;
            }
        }
        secs.serialize(serializer)
    }
}

impl<STRICTNESS> SerializeAs<DurationSigned> for DurationSeconds<String, STRICTNESS>
where
    STRICTNESS: Strictness,
{
    fn serialize_as<S>(source: &DurationSigned, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut secs = source.sign.apply(source.duration.as_secs() as i64);

        // Properly round the value
        if source.duration.subsec_millis() >= 500 {
            if source.sign.is_positive() {
                secs += 1;
            } else {
                secs -= 1;
            }
        }
        secs.to_string().serialize(serializer)
    }
}

impl<STRICTNESS> SerializeAs<DurationSigned> for DurationSecondsWithFrac<f64, STRICTNESS>
where
    STRICTNESS: Strictness,
{
    fn serialize_as<S>(source: &DurationSigned, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        source
            .sign
            .apply(utils::duration_as_secs_f64(&source.duration))
            .serialize(serializer)
    }
}

impl<STRICTNESS> SerializeAs<DurationSigned> for DurationSecondsWithFrac<String, STRICTNESS>
where
    STRICTNESS: Strictness,
{
    fn serialize_as<S>(source: &DurationSigned, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        source
            .sign
            .apply(utils::duration_as_secs_f64(&source.duration))
            .to_string()
            .serialize(serializer)
    }
}

macro_rules! duration_impls {
    ($($inner:ident { $($factor:literal => $outer:ident,)+ })+) => {
        $($(

        impl<FORMAT, STRICTNESS> SerializeAs<DurationSigned> for $outer<FORMAT, STRICTNESS>
        where
            FORMAT: Format,
            STRICTNESS: Strictness,
            $inner<FORMAT, STRICTNESS>: SerializeAs<DurationSigned>
        {
            fn serialize_as<S>(source: &DurationSigned, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                $inner::<FORMAT, STRICTNESS>::serialize_as(&(*source * $factor), serializer)
            }
        }

        impl<'de, FORMAT, STRICTNESS> DeserializeAs<'de, DurationSigned> for $outer<FORMAT, STRICTNESS>
        where
            FORMAT: Format,
            STRICTNESS: Strictness,
            $inner<FORMAT, STRICTNESS>: DeserializeAs<'de, DurationSigned>,
        {
            fn deserialize_as<D>(deserializer: D) -> Result<DurationSigned, D::Error>
            where
                D: Deserializer<'de>,
            {
                let dur = $inner::<FORMAT, STRICTNESS>::deserialize_as(deserializer)?;
                Ok(dur / $factor)
            }
        }

        )+)+    };
}
duration_impls!(
    DurationSeconds {
        1000u32 => DurationMilliSeconds,
        1_000_000u32 => DurationMicroSeconds,
        1_000_000_000u32 => DurationNanoSeconds,
    }
    DurationSecondsWithFrac {
        1000u32 => DurationMilliSecondsWithFrac,
        1_000_000u32 => DurationMicroSecondsWithFrac,
        1_000_000_000u32 => DurationNanoSecondsWithFrac,
    }
);

struct DurationVisitorFlexible;
impl<'de> Visitor<'de> for DurationVisitorFlexible {
    type Value = DurationSigned;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> ::std::fmt::Result {
        formatter.write_str("an integer, a float, or a string containing a number")
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if value >= 0 {
            Ok(DurationSigned::new(Sign::Positive, value as u64, 0))
        } else {
            Ok(DurationSigned::new(Sign::Negative, (-value) as u64, 0))
        }
    }

    fn visit_u64<E>(self, secs: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(DurationSigned::new(Sign::Positive, secs, 0))
    }

    fn visit_f64<E>(self, secs: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        utils::duration_signed_from_secs_f64(secs).map_err(de::Error::custom)
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match parse_float_into_time_parts(value) {
            Ok((sign, seconds, subseconds)) => Ok(DurationSigned::new(sign, seconds, subseconds)),
            Err(ParseFloatError::InvalidValue) => {
                Err(de::Error::invalid_value(Unexpected::Str(value), &self))
            }
            Err(ParseFloatError::Custom(msg)) => Err(de::Error::custom(msg)),
        }
    }
}

impl<'de> DeserializeAs<'de, DurationSigned> for DurationSeconds<u64, Strict> {
    fn deserialize_as<D>(deserializer: D) -> Result<DurationSigned, D::Error>
    where
        D: Deserializer<'de>,
    {
        u64::deserialize(deserializer).map(|secs: u64| DurationSigned::new(Sign::Positive, secs, 0))
    }
}

impl<'de> DeserializeAs<'de, DurationSigned> for DurationSeconds<i64, Strict> {
    fn deserialize_as<D>(deserializer: D) -> Result<DurationSigned, D::Error>
    where
        D: Deserializer<'de>,
    {
        i64::deserialize(deserializer).map(|mut secs: i64| {
            let mut sign = Sign::Positive;
            if secs.is_negative() {
                secs = -secs;
                sign = Sign::Negative;
            }
            DurationSigned::new(sign, secs as u64, 0)
        })
    }
}

impl<'de> DeserializeAs<'de, DurationSigned> for DurationSeconds<f64, Strict> {
    fn deserialize_as<D>(deserializer: D) -> Result<DurationSigned, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val = f64::deserialize(deserializer)?.round();
        utils::duration_signed_from_secs_f64(val).map_err(de::Error::custom)
    }
}

impl<'de> DeserializeAs<'de, DurationSigned> for DurationSeconds<String, Strict> {
    fn deserialize_as<D>(deserializer: D) -> Result<DurationSigned, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DurationDeserializationVisitor;

        impl<'de> Visitor<'de> for DurationDeserializationVisitor {
            type Value = DurationSigned;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(formatter, "a string containing a number")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let mut secs: i64 = value.parse().map_err(de::Error::custom)?;
                let mut sign = Sign::Positive;
                if secs.is_negative() {
                    secs = -secs;
                    sign = Sign::Negative;
                }
                Ok(DurationSigned::new(sign, secs as u64, 0))
            }
        }

        deserializer.deserialize_str(DurationDeserializationVisitor)
    }
}

impl<'de, FORMAT> DeserializeAs<'de, DurationSigned> for DurationSeconds<FORMAT, Flexible>
where
    FORMAT: Format,
{
    fn deserialize_as<D>(deserializer: D) -> Result<DurationSigned, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(DurationVisitorFlexible)
    }
}

impl<'de> DeserializeAs<'de, DurationSigned> for DurationSecondsWithFrac<f64, Strict> {
    fn deserialize_as<D>(deserializer: D) -> Result<DurationSigned, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val = f64::deserialize(deserializer)?;
        utils::duration_signed_from_secs_f64(val).map_err(de::Error::custom)
    }
}

impl<'de> DeserializeAs<'de, DurationSigned> for DurationSecondsWithFrac<String, Strict> {
    fn deserialize_as<D>(deserializer: D) -> Result<DurationSigned, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match parse_float_into_time_parts(&value) {
            Ok((sign, seconds, subseconds)) => Ok(DurationSigned {
                sign,
                duration: Duration::new(seconds, subseconds),
            }),
            Err(ParseFloatError::InvalidValue) => Err(de::Error::invalid_value(
                Unexpected::Str(&value),
                &"a string containing an integer or float",
            )),
            Err(ParseFloatError::Custom(msg)) => Err(de::Error::custom(msg)),
        }
    }
}

impl<'de, FORMAT> DeserializeAs<'de, DurationSigned> for DurationSecondsWithFrac<FORMAT, Flexible>
where
    FORMAT: Format,
{
    fn deserialize_as<D>(deserializer: D) -> Result<DurationSigned, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(DurationVisitorFlexible)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum ParseFloatError {
    InvalidValue,
    Custom(String),
}

fn parse_float_into_time_parts(mut value: &str) -> Result<(Sign, u64, u32), ParseFloatError> {
    let sign = match value.chars().next() {
        // Advance by the size of the parsed char
        Some('+') => {
            value = &value[1..];
            Sign::Positive
        }
        Some('-') => {
            value = &value[1..];
            Sign::Negative
        }
        _ => Sign::Positive,
    };

    let parts: Vec<_> = value.split('.').collect();
    match *parts.as_slice() {
        [seconds] => {
            if let Ok(seconds) = seconds.parse() {
                Ok((sign, seconds, 0))
            } else {
                Err(ParseFloatError::InvalidValue)
            }
        }
        [seconds, subseconds] => {
            if let Ok(seconds) = seconds.parse() {
                let subseclen = subseconds.chars().count() as u32;
                if subseclen > 9 {
                    return Err(ParseFloatError::Custom(format!(
                        "Duration and Timestamps with no more than 9 digits precision, but '{}' has more",
                        value
                    )));
                }

                if let Ok(mut subseconds) = subseconds.parse() {
                    // convert subseconds to nanoseconds (10^-9), require 9 places for nanoseconds
                    subseconds *= 10u32.pow(9 - subseclen);
                    Ok((sign, seconds, subseconds))
                } else {
                    Err(ParseFloatError::InvalidValue)
                }
            } else {
                Err(ParseFloatError::InvalidValue)
            }
        }

        _ => Err(ParseFloatError::InvalidValue),
    }
}

#[test]
fn test_parse_float_into_time_parts() {
    // Test normal behavior
    assert_eq!(
        Ok((Sign::Positive, 123, 456_000_000)),
        parse_float_into_time_parts("+123.456")
    );
    assert_eq!(
        Ok((Sign::Negative, 123, 987_000)),
        parse_float_into_time_parts("-123.000987")
    );
    assert_eq!(
        Ok((Sign::Positive, 18446744073709551615, 123_456_789)),
        parse_float_into_time_parts("18446744073709551615.123456789")
    );

    // Test behavior around 0
    assert_eq!(
        Ok((Sign::Positive, 0, 456_000_000)),
        parse_float_into_time_parts("+0.456")
    );
    assert_eq!(
        Ok((Sign::Negative, 0, 987_000)),
        parse_float_into_time_parts("-0.000987")
    );
    assert_eq!(
        Ok((Sign::Positive, 0, 123_456_789)),
        parse_float_into_time_parts("0.123456789")
    );
}
