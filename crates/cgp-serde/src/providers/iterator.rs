use alloc::vec::Vec;

use cgp::prelude::*;

use crate::components::{
    CanDeserializeValue, ValueDeserializer, ValueDeserializerComponent, ValueSerializer,
    ValueSerializerComponent,
};

pub struct SerializeIterator;

#[cgp_provider]
impl<Context, Value> ValueSerializer<Context, Value> for SerializeIterator
where
    for<'a> &'a Value: IntoIterator<Item: serde::Serialize>,
{
    fn serialize<S>(_context: &Context, value: &Value, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_seq(value)
    }
}

#[cgp_provider]
impl<'a, Context, Value, Item> ValueDeserializer<'a, Context, Value> for SerializeIterator
where
    Value: IntoIterator<Item = Item> + FromIterator<Item>,
    Context: CanDeserializeValue<'a, Vec<Item>>,
{
    fn deserialize<D>(context: &Context, deserializer: D) -> Result<Value, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        let items = context.deserialize(deserializer)?;
        Ok(Value::from_iter(items))
    }
}
