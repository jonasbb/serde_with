use crate::prelude::*;

enum GoodOrError<T, TAs> {
    Good(T),
    // Only here to consume the TAs generic
    Error(PhantomData<TAs>),
}

impl<'de, T, TAs> Deserialize<'de> for GoodOrError<T, TAs>
where
    TAs: DeserializeAs<'de, T>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let is_hr = deserializer.is_human_readable();
        let content: content::de::Content<'de> = Deserialize::deserialize(deserializer)?;

        Ok(
            match <DeserializeAsWrap<T, TAs>>::deserialize(content::de::ContentDeserializer::<
                D::Error,
            >::new(content, is_hr))
            {
                Ok(elem) => GoodOrError::Good(elem.into_inner()),
                Err(_) => GoodOrError::Error(PhantomData),
            },
        )
    }
}

impl<'de, T, U> DeserializeAs<'de, Vec<T>> for VecSkipError<U>
where
    U: DeserializeAs<'de, T>,
{
    fn deserialize_as<D>(deserializer: D) -> Result<Vec<T>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SeqVisitor<T, U> {
            marker: PhantomData<T>,
            marker2: PhantomData<U>,
        }

        impl<'de, T, TAs> Visitor<'de> for SeqVisitor<T, TAs>
        where
            TAs: DeserializeAs<'de, T>,
        {
            type Value = Vec<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a sequence")
            }

            fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                utils::SeqIter::new(seq)
                    .filter_map(|res: Result<GoodOrError<T, TAs>, A::Error>| match res {
                        Ok(GoodOrError::Good(value)) => Some(Ok(value)),
                        Ok(GoodOrError::Error(_)) => None,
                        Err(err) => Some(Err(err)),
                    })
                    .collect()
            }
        }

        let visitor = SeqVisitor::<T, U> {
            marker: PhantomData,
            marker2: PhantomData,
        };
        deserializer.deserialize_seq(visitor)
    }
}
