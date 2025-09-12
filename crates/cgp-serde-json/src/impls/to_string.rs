use alloc::string::String;

use cgp::prelude::*;
use cgp_serde::components::{CanSerializeValue, ValueToSerializer, ValueToSerializerComponent};
use cgp_serde::types::SerializeWithContext;
use serde_json::Error;

#[cgp_new_provider]
impl<Context, Value> ValueToSerializer<Context, Value, String> for SerializeToJsonString
where
    Context: CanSerializeValue<Value>,
{
    type Error = Error;

    fn serialize_to(context: &Context, value: &Value) -> Result<String, Error> {
        serde_json::to_string(&SerializeWithContext::new(context, value))
    }
}
