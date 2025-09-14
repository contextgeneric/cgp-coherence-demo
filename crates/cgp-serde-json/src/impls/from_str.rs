use cgp::prelude::*;
use serde_json::de::StrRead;

use crate::DeserializeFromJsonReader;

#[cgp_new_provider]
impl<'a, Context, Code, Value, S> TryComputer<Context, Code, &'a S> for DeserializeFromJsonString
where
    Context: HasErrorType,
    DeserializeFromJsonReader: TryComputer<Context, Code, StrRead<'a>, Output = Value>,
    S: AsRef<str>,
{
    type Output = Value;

    fn try_compute(
        context: &Context,
        code: PhantomData<Code>,
        source: &'a S,
    ) -> Result<Value, Context::Error> {
        DeserializeFromJsonReader::try_compute(context, code, StrRead::new(source.as_ref()))
    }
}
