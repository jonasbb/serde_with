use crate::prelude::*;

impl<T, U> SerializeAs<Vec<T>> for VecSkipError<U>
where
    U: SerializeAs<T>,
{
    fn serialize_as<S>(source: &Vec<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Vec::<U>::serialize_as(source, serializer)
    }
}
