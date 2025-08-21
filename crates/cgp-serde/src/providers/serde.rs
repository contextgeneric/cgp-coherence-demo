use cgp::prelude::*;
use serde::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};

use crate::components::{
    DeserializeImpl, DeserializeImplComponent, SerializeImpl, SerializeImplComponent,
};

pub struct UseSerde;

#[cgp_provider]
impl<Value> SerializeImpl<Value> for UseSerde
where
    Value: SerdeSerialize,
{
    fn serialize<S>(value: &Value, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        value.serialize(serializer)
    }
}

#[cgp_provider]
impl<'a, Value> DeserializeImpl<'a, Value> for UseSerde
where
    Value: SerdeDeserialize<'a>,
{
    fn deserialize<D>(deserializer: D) -> Result<Value, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        Value::deserialize(deserializer)
    }
}
