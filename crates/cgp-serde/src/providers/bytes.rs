use core::fmt::Display;

use cgp::prelude::*;
use serde::de::{Error, Visitor};

use crate::components::{
    ValueDeserializer, ValueDeserializerComponent, ValueSerializer, ValueSerializerComponent,
};

pub struct SerializeBytes;

#[cgp_provider]
impl<Context, Value> ValueSerializer<Context, Value> for SerializeBytes
where
    Value: AsRef<[u8]>,
{
    fn serialize<S>(_context: &Context, value: &Value, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(value.as_ref())
    }
}

#[cgp_provider]
impl<'a, Context, Value> ValueDeserializer<'a, Context, Value> for SerializeBytes
where
    Value: From<&'a [u8]>,
{
    fn deserialize<D>(_context: &Context, deserializer: D) -> Result<Value, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        let bytes = deserializer.deserialize_bytes(Self)?;
        Ok(bytes.into())
    }
}

#[cgp_new_provider]
impl<'a, Context, Value> ValueDeserializer<'a, Context, Value> for TryDeserializeBytes
where
    Value: TryFrom<&'a [u8], Error: Display>,
{
    fn deserialize<D>(_context: &Context, deserializer: D) -> Result<Value, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        let bytes = deserializer.deserialize_bytes(SerializeBytes)?;
        let value = bytes.try_into().map_err(D::Error::custom)?;

        Ok(value)
    }
}

impl<'a> Visitor<'a> for SerializeBytes {
    type Value = &'a [u8];

    fn expecting(&self, formatter: &mut alloc::fmt::Formatter) -> alloc::fmt::Result {
        formatter.write_str("bytes")
    }

    fn visit_borrowed_bytes<E>(self, bytes: &'a [u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(bytes)
    }
}
