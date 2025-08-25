use cgp::prelude::*;
use cgp_serde::components::{
    CanDeserializeValue, CanSerializeValue, ValueDeserializer, ValueDeserializerComponent,
    ValueSerializer, ValueSerializerComponent,
};
use chrono::{DateTime, Utc};
use serde::de::Error;

pub struct SerializeTimestamp;

#[cgp_provider]
impl<Context> ValueSerializer<Context, DateTime<Utc>> for SerializeTimestamp
where
    Context: CanSerializeValue<i64>,
{
    fn serialize<S>(
        context: &Context,
        value: &DateTime<Utc>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        context.serialize(&value.timestamp(), serializer)
    }
}

#[cgp_provider]
impl<'de, Context> ValueDeserializer<'de, Context, DateTime<Utc>> for SerializeTimestamp
where
    Context: CanDeserializeValue<'de, i64>,
{
    fn deserialize<D>(context: &Context, deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let timestamp = context.deserialize(deserializer)?;
        let date = DateTime::from_timestamp(timestamp, 0)
            .ok_or_else(|| Error::custom("invalid timestamp"))?;
        Ok(date)
    }
}
