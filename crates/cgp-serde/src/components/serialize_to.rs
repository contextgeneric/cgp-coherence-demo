use cgp::prelude::*;

#[cgp_component {
    provider: ValueToSerializer,
    derive_delegate: UseDelegate<Code>,
}]
pub trait CanSerializeValueTo<Code, Value>: HasErrorType {
    type Target;

    fn serialize_to(
        &self,
        _code: PhantomData<Code>,
        value: &Value,
    ) -> Result<Self::Target, Self::Error>;
}
