use cgp::prelude::*;
use serde_json::de::StrRead;

use crate::DeserializeFromJsonReader;

pub struct DeserializeFromJsonString<InDeserializer = DeserializeFromJsonReader>(
    pub PhantomData<InDeserializer>,
);

#[cgp_provider]
impl<'a, Context, Code, Value, S, InDeserializer> TryComputer<Context, Code, &'a S>
    for DeserializeFromJsonString<InDeserializer>
where
    Context: HasErrorType,
    InDeserializer: TryComputer<Context, Code, StrRead<'a>, Output = Value>,
    S: AsRef<str>,
{
    type Output = Value;

    fn try_compute(
        context: &Context,
        code: PhantomData<Code>,
        source: &'a S,
    ) -> Result<Value, Context::Error> {
        InDeserializer::try_compute(context, code, StrRead::new(source.as_ref()))
    }
}
