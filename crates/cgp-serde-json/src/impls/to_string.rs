use alloc::string::String;

use cgp::prelude::*;
use cgp_serde::components::{CanSerializeValue, ValueToSerializer, ValueToSerializerComponent};
use cgp_serde::types::SerializeWithContext;
use serde_json::Error;

#[cgp_new_provider]
impl<Context, Code, Value> ValueToSerializer<Context, Code, Value> for SerializeToJsonString
where
    Context: CanSerializeValue<Value> + CanRaiseError<Error>,
{
    type Target = String;

    fn serialize_to(
        context: &Context,
        _code: PhantomData<Code>,
        value: &Value,
    ) -> Result<String, Context::Error> {
        serde_json::to_string(&SerializeWithContext::new(context, value))
            .map_err(Context::raise_error)
    }
}
