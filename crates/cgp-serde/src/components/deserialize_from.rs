use cgp::prelude::*;

#[cgp_component {
    provider: ValueFromDeserializer,
    derive_delegate: UseDelegate<Code>,
}]
pub trait CanDeserializeValueFrom<Code, Value, Source>: HasErrorType {
    fn deserialize_from(
        &self,
        _code: PhantomData<Code>,
        source: Source,
    ) -> Result<Value, Self::Error>;
}
