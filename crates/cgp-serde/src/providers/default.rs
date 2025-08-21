use cgp::prelude::*;
use serde::de::Visitor;

use crate::components::{DeserializeImpl, DeserializeImplComponent, SerializeImplComponent};
use crate::providers::UseSerde;

pub struct DeserializeDefault<Provider = UseSerde>(pub PhantomData<Provider>);

delegate_components! {
    <Provider>
    DeserializeDefault<Provider> {
        SerializeImplComponent: Provider,
    }
}

#[cgp_provider]
impl<'a, Value, Provider> DeserializeImpl<'a, Value> for DeserializeDefault<Provider>
where
    Value: Default,
    Provider: DeserializeImpl<'a, Value>,
{
    fn deserialize<D>(deserializer: D) -> Result<Value, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        deserializer.deserialize_option(DefaultVisitor::<Value, Provider>(PhantomData))
    }
}

struct DefaultVisitor<Value, Provider>(pub PhantomData<(Value, Provider)>);

impl<'a, Value, Provider> Visitor<'a> for DefaultVisitor<Value, Provider>
where
    Value: Default,
    Provider: DeserializeImpl<'a, Value>,
{
    type Value = Value;

    fn expecting(&self, formatter: &mut alloc::fmt::Formatter) -> alloc::fmt::Result {
        formatter.write_str("optional")
    }

    fn visit_none<E>(self) -> Result<Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Value::default())
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Value, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        Provider::deserialize(deserializer)
    }
}
