use alloc::string::ToString;
use core::fmt::Display;
use core::str::FromStr;

use cgp::prelude::*;
use serde::Serialize as _;
use serde::de::Error;

use crate::components::{
    CanDeserializeValue, ValueDeserializer, ValueDeserializerComponent, ValueSerializer,
    ValueSerializerComponent,
};

pub struct SerializeString;

#[cgp_provider]
impl<Context, Value> ValueSerializer<Context, Value> for SerializeString
where
    Value: Display,
{
    fn serialize<S>(_context: &Context, value: &Value, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let str_value = value.to_string();
        str_value.serialize(serializer)
    }
}

#[cgp_provider]
impl<'a, Context, Value> ValueDeserializer<'a, Context, Value> for SerializeString
where
    Context: CanDeserializeValue<'a, &'a str>,
    Value: FromStr<Err: Display>,
{
    fn deserialize<D>(context: &Context, deserializer: D) -> Result<Value, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        let str_value = context.deserialize(deserializer)?;
        Value::from_str(str_value).map_err(D::Error::custom)
    }
}
