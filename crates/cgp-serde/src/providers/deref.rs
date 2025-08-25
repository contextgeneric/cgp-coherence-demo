use core::ops::Deref;

use cgp::prelude::*;

use crate::components::{CanSerializeValue, ValueSerializer, ValueSerializerComponent};

#[cgp_new_provider]
impl<Context, Value> ValueSerializer<Context, Value> for SerializeDeref
where
    Value: Deref,
    Context: CanSerializeValue<Value::Target>,
{
    fn serialize<S>(context: &Context, value: &Value, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        context.serialize(value.deref(), serializer)
    }
}
