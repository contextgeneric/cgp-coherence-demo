use cgp::prelude::*;
use cgp_serde::components::{ValueFromDeserializer, ValueFromDeserializerComponent};
use serde_json::de::StrRead;

use crate::DeserializeFromJsonReader;

#[cgp_new_provider]
impl<'a, Context, Value, S> ValueFromDeserializer<Context, Value, &'a S>
    for DeserializeFromJsonString
where
    Context: HasErrorType,
    DeserializeFromJsonReader: ValueFromDeserializer<Context, Value, StrRead<'a>>,
    S: AsRef<str>,
{
    fn deserialize_from(context: &Context, source: &'a S) -> Result<Value, Context::Error> {
        DeserializeFromJsonReader::deserialize_from(context, StrRead::new(source.as_ref()))
    }
}
