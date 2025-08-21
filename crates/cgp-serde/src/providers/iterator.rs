use alloc::vec::Vec;
use cgp::prelude::*;

use crate::components::{
    DeserializeImpl, DeserializeImplComponent, SerializeImpl, SerializeImplComponent,
};
use crate::providers::UseSerde;

pub struct SerializeIterator;

#[cgp_provider]
impl<Value> SerializeImpl<Value> for SerializeIterator
where
    for<'a> &'a Value: IntoIterator<Item: serde::Serialize>,
{
    fn serialize<S>(value: &Value, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_seq(value)
    }
}

#[cgp_provider]
impl<'a, Value, Item> DeserializeImpl<'a, Value> for SerializeIterator
where
    Value: IntoIterator<Item = Item> + FromIterator<Item>,
    UseSerde: DeserializeImpl<'a, Vec<Item>>,
{
    fn deserialize<D>(deserializer: D) -> Result<Value, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        let items = UseSerde::deserialize(deserializer)?;
        Ok(Value::from_iter(items))
    }
}
