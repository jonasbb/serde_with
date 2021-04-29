/// TODO
#[macro_export]
macro_rules! serde_conv {
    ($m:ident, $t:ty, $ser:expr, $de:expr) => {
        #[allow(non_camel_case_types)]
        pub struct $m;
        impl $m {
            pub fn serialize<S>(x: &$t, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
            where
                S: $crate::serde::Serializer,
            {
                let y = $ser(x);
                $crate::serde::Serialize::serialize(&y, serializer)
            }

            pub fn deserialize<'de, D>(deserializer: D) -> ::std::result::Result<$t, D::Error>
            where
                D: $crate::serde::Deserializer<'de>,
            {
                let y = $crate::serde::Deserialize::deserialize(deserializer)?;
                ::std::result::Result::Ok($de(y).map_err($crate::serde::de::Error::custom)?)
            }
        }

        impl $crate::SerializeAs<$t> for $m {
            fn serialize_as<S>(x: &$t, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
            where
                S: $crate::serde::Serializer,
            {
                let y = $ser(x);
                $crate::serde::Serialize::serialize(&y, serializer)
            }
        }

        impl<'de> $crate::DeserializeAs<'de, $t> for $m {
            fn deserialize_as<D>(deserializer: D) -> ::std::result::Result<$t, D::Error>
            where
                D: $crate::serde::Deserializer<'de>,
            {
                let y = $crate::serde::Deserialize::deserialize(deserializer)?;
                ::std::result::Result::Ok($de(y).map_err($crate::serde::de::Error::custom)?)
            }
        }
    };
}
