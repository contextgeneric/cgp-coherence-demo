use alloc::borrow::ToOwned;
use alloc::string::String;

use cgp::prelude::*;
use serde::de::{Error, Visitor};

use crate::components::{
    ValueDeserializer, ValueDeserializerComponent, ValueSerializer, ValueSerializerComponent,
};

pub struct SerializeString;

#[cgp_provider]
impl<Context, Value> ValueSerializer<Context, Value> for SerializeString
where
    Value: AsRef<str>,
{
    fn serialize<S>(_context: &Context, value: &Value, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(value.as_ref())
    }
}

#[cgp_provider]
impl<'a, Context> ValueDeserializer<'a, Context, String> for SerializeString {
    fn deserialize<D>(_context: &Context, deserializer: D) -> Result<String, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        deserializer.deserialize_string(Self)
    }
}

impl<'a> Visitor<'a> for SerializeString {
    type Value = String;

    fn expecting(&self, formatter: &mut alloc::fmt::Formatter) -> alloc::fmt::Result {
        formatter.write_str("string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v.to_owned())
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v)
    }
}
