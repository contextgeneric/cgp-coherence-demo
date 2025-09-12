use cgp::prelude::*;

#[cgp_component {
    provider: ValueDeserializer,
    derive_delegate: UseDelegate<Value>,
}]
pub trait CanDeserializeValue<'de, Value> {
    fn deserialize<D>(&self, deserializer: D) -> Result<Value, D::Error>
    where
        D: serde::Deserializer<'de>;
}

#[cgp_component {
    provider: ValueFromDeserializer,
    derive_delegate: UseDelegate<Source>,
}]
pub trait CanDeserializeValueFrom<Value, Source> {
    type Error;

    fn deserialize_from(&self, source: Source) -> Result<Value, Self::Error>;
}
