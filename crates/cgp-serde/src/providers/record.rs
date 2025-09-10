use cgp::prelude::*;

use crate::components::{ValueDeserializer, ValueDeserializerComponent};

pub struct DeserializeRecordFields<Record>(pub PhantomData<Record>);

#[cgp_provider]
impl<'de, Context, Record> ValueDeserializer<'de, Context, Record>
    for DeserializeRecordFields<Record>
where
    Record: HasBuilder + HasFields,
{
    fn deserialize<D>(_context: &Context, _deserializer: D) -> Result<Record, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let _builder = Record::builder();

        todo!()
    }
}
