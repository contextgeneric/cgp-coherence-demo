use cgp::prelude::*;
use serde::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};

use crate::components::{
    ValueDeserializer, ValueDeserializerComponent, ValueSerializer, ValueSerializerComponent,
};

pub struct UseSerde;

#[cgp_provider]
impl<Context, Value> ValueSerializer<Context, Value> for UseSerde
where
    Value: SerdeSerialize,
{
    fn serialize<S>(_context: &Context, value: &Value, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        value.serialize(serializer)
    }
}

#[cgp_provider]
impl<'a, Context, Value> ValueDeserializer<'a, Context, Value> for UseSerde
where
    Value: SerdeDeserialize<'a>,
{
    fn deserialize<D>(_context: &Context, deserializer: D) -> Result<Value, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        Value::deserialize(deserializer)
    }
}
