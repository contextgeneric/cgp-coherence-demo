use cgp::prelude::*;
use cgp_serde::components::{
    CanDeserializeValue, ValueFromDeserializer, ValueFromDeserializerComponent,
};
use serde_json::de::Read;
use serde_json::{Deserializer, Error};

#[cgp_new_provider]
impl<Context, Value, R, 'de> ValueFromDeserializer<Context, Value, R> for DeserializeFromJsonReader
where
    R: Read<'de>,
    Context: CanDeserializeValue<'de, Value>,
{
    type Error = Error;

    fn deserialize_from(context: &Context, source: R) -> Result<Value, Error> {
        let mut deserializer = Deserializer::new(source);
        let value = context.deserialize(&mut deserializer)?;
        deserializer.end()?;

        Ok(value)
    }
}
