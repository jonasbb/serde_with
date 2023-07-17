use crate::prelude::*;
use ::schemars_0_8::{gen::SchemaGenerator, schema, JsonSchema};

///////////////////////////////////////////////////////////////////////////////
// region: Simple Wrapper types (e.g., Box, Option)

impl<T, TAs> JsonSchema for Schema<Box<T>, Box<TAs>>
where
    Schema<T, TAs>: JsonSchema,
{
    fn schema_name() -> String {
        <Box<Schema<T, TAs>>>::schema_name()
    }

    fn json_schema(gen: &mut SchemaGenerator) -> schema::Schema {
        <Box<Schema<T, TAs>>>::json_schema(gen)
    }
}

// endregion
///////////////////////////////////////////////////////////////////////////////
// region: Collection Types (e.g., Maps, Sets, Vec)

impl<T, TAs> JsonSchema for Schema<Vec<T>, Vec<TAs>>
where
    Schema<T, TAs>: JsonSchema,
{
    fn schema_name() -> String {
        <Vec<Schema<T, TAs>>>::schema_name()
    }

    fn json_schema(gen: &mut SchemaGenerator) -> schema::Schema {
        <Vec<Schema<T, TAs>>>::json_schema(gen)
    }
}

// endregion
///////////////////////////////////////////////////////////////////////////////
// region: Conversion types which cause different serialization behavior

impl<T> JsonSchema for Schema<T, Same>
where
    T: JsonSchema,
{
    fn schema_name() -> String {
        T::schema_name()
    }

    fn json_schema(gen: &mut SchemaGenerator) -> schema::Schema {
        T::json_schema(gen)
    }
}

impl<T> JsonSchema for Schema<T, DisplayFromStr> {
    fn schema_name() -> String {
        String::schema_name()
    }

    fn json_schema(gen: &mut SchemaGenerator) -> schema::Schema {
        String::json_schema(gen)
    }
}

impl<T> JsonSchema for Schema<Option<T>, NoneAsEmptyString> {
    fn schema_name() -> String {
        <Option<String>>::schema_name()
    }

    fn json_schema(gen: &mut SchemaGenerator) -> schema::Schema {
        <Option<String>>::json_schema(gen)
    }
}

// endregion
