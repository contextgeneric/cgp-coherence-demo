use cgp::prelude::*;

use crate::components::{
    DeserializeImpl, DeserializeImplComponent, SerializeImpl, SerializeImplComponent,
};

pub struct SerializeFrom<Target, Provider = UseContext>(pub PhantomData<(Target, Provider)>);

#[cgp_provider]
impl<Value, Target, Provider> SerializeImpl<Value> for SerializeFrom<Target, Provider>
where
    Value: Clone + Into<Target>,
    Provider: SerializeImpl<Target>,
{
    fn serialize<S>(value: &Value, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let target = value.clone().into();
        Provider::serialize(&target, serializer)
    }
}

#[cgp_provider]
impl<'a, Value, Target, Provider> DeserializeImpl<'a, Value> for SerializeFrom<Target, Provider>
where
    Provider: DeserializeImpl<'a, Target>,
    Target: Into<Value>,
{
    fn deserialize<D>(deserializer: D) -> Result<Value, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        let target = Provider::deserialize(deserializer)?;
        Ok(target.into())
    }
}
