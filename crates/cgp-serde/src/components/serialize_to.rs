use cgp::prelude::*;

#[cgp_component {
    provider: ValueToSerializer,
    derive_delegate: UseDelegate<Target>,
}]
pub trait CanSerializeValueTo<Value, Target> {
    type Error;

    fn serialize_to(&self, value: &Value) -> Result<Target, Self::Error>;
}
