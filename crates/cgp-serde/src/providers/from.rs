use cgp::prelude::*;

use crate::components::{
    DeserializeImpl, DeserializeImplComponent, SerializeImpl, SerializeImplComponent,
};
use crate::providers::UseSerde;

pub struct SerializeFrom<Target>(pub PhantomData<Target>);

#[cgp_provider]
impl<Value, Target> SerializeImpl<Value> for SerializeFrom<Target>
where
    Value: Clone + Into<Target>,
    UseSerde: SerializeImpl<Target>,
{
    fn serialize<S>(value: &Value, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let target = value.clone().into();
        UseSerde::serialize(&target, serializer)
    }
}

#[cgp_provider]
impl<'a, Value, Target> DeserializeImpl<'a, Value> for SerializeFrom<Target>
where
    UseSerde: DeserializeImpl<'a, Target>,
    Target: Into<Value>,
{
    fn deserialize<D>(deserializer: D) -> Result<Value, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        let target = UseSerde::deserialize(deserializer)?;
        Ok(target.into())
    }
}
