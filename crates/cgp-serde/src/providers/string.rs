use alloc::string::ToString;
use core::fmt::Display;
use core::str::FromStr;

use cgp::prelude::*;
use serde::Serialize as _;
use serde::de::Error;

use crate::components::{
    DeserializeImpl, DeserializeImplComponent, SerializeImpl, SerializeImplComponent,
};
use crate::providers::UseSerde;

pub struct SerializeString;

#[cgp_provider]
impl<Value> SerializeImpl<Value> for SerializeString
where
    Value: Display,
{
    fn serialize<S>(value: &Value, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let str_value = value.to_string();
        str_value.serialize(serializer)
    }
}

#[cgp_provider]
impl<'a, Value> DeserializeImpl<'a, Value> for SerializeString
where
    UseSerde: DeserializeImpl<'a, &'a str>,
    Value: FromStr<Err: Display>,
{
    fn deserialize<D>(deserializer: D) -> Result<Value, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        let str_value = UseSerde::deserialize(deserializer)?;
        Value::from_str(str_value).map_err(D::Error::custom)
    }
}
