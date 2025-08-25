use alloc::vec::Vec;

use cgp::prelude::*;
use serde::ser::SerializeSeq;

use crate::components::{
    CanDeserializeValue, CanSerializeValue, ValueDeserializer, ValueDeserializerComponent,
    ValueSerializer, ValueSerializerComponent,
};
use crate::types::SerializeWithContext;

pub struct SerializeIterator;

#[cgp_provider]
impl<Context, Value> ValueSerializer<Context, Value> for SerializeIterator
where
    for<'a> &'a Value: IntoIterator,
    Context: for<'a> CanSerializeValue<<&'a Value as IntoIterator>::Item>,
{
    fn serialize<S>(context: &Context, value: &Value, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let items = value.into_iter();
        let mut serializer = serializer.serialize_seq(None)?;
        for item in items {
            serializer.serialize_element(&SerializeWithContext {
                context,
                value: &item,
            })?
        }

        serializer.end()
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
