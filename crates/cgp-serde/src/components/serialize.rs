use cgp::prelude::*;

#[cgp_component {
    provider: ValueSerializer,
    derive_delegate: UseDelegate<Value>,
}]
pub trait CanSerializeValue<Value: ?Sized> {
    fn serialize<S>(&self, value: &Value, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer;
}

#[cgp_component {
    provider: ValueToSerializer,
    derive_delegate: UseDelegate<Target>,
}]
pub trait CanSerializeValueTo<Value, Target> {
    type Error;

    fn serialize_to(&self, value: &Value) -> Result<Target, Self::Error>;
}
