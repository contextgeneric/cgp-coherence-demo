use cgp::prelude::*;

#[cgp_component {
    provider: ValueFromDeserializer,
    derive_delegate: UseDelegate<Source>,
}]
pub trait CanDeserializeValueFrom<Value, Source>: HasErrorType {
    fn deserialize_from(&self, source: Source) -> Result<Value, Self::Error>;
}
