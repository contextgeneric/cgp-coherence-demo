use alloc::string::String;

use cgp::prelude::*;
use cgp_serde::components::{
    CanDeserializeValue, CanSerializeValue, ValueDeserializer, ValueDeserializerComponent,
    ValueSerializer, ValueSerializerComponent,
};
use chrono::{DateTime, Utc};
use serde::de::Error;

pub struct SerializeDate;

#[cgp_provider]
impl<Context> ValueSerializer<Context, DateTime<Utc>> for SerializeDate
where
    Context: CanSerializeValue<String>,
{
    fn serialize<S>(
        context: &Context,
        value: &DateTime<Utc>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        context.serialize(&value.to_rfc3339(), serializer)
    }
}

#[cgp_provider]
impl<'de, Context> ValueDeserializer<'de, Context, DateTime<Utc>> for SerializeDate
where
    Context: CanDeserializeValue<'de, String>,
{
    fn deserialize<D>(context: &Context, deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let date_string = context.deserialize(deserializer)?;
        let date = DateTime::parse_from_rfc3339(&date_string).map_err(Error::custom)?;
        Ok(date.into())
    }
}
