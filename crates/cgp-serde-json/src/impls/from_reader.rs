use cgp::prelude::*;
use cgp_serde::components::{
    CanDeserializeValue, ValueFromDeserializer, ValueFromDeserializerComponent,
};
use serde_json::de::Read;
use serde_json::{Deserializer, Error};

#[cgp_new_provider]
impl<Context, Code, Value, R, 'de> ValueFromDeserializer<Context, Code, Value, R>
    for DeserializeFromJsonReader
where
    R: Read<'de>,
    Context: CanDeserializeValue<'de, Value> + CanRaiseError<Error>,
{
    fn deserialize_from(
        context: &Context,
        _code: PhantomData<Code>,
        source: R,
    ) -> Result<Value, Context::Error> {
        let mut deserializer = Deserializer::new(source);
        let value = context
            .deserialize(&mut deserializer)
            .map_err(Context::raise_error)?;
        deserializer.end().map_err(Context::raise_error)?;

        Ok(value)
    }
}
