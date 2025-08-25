use crate::components::CanSerializeValue;

pub struct SerializeWithContext<'a, Context, T> {
    pub context: &'a Context,
    pub value: &'a T,
}

impl<'a, Context, T> serde::Serialize for SerializeWithContext<'a, Context, T>
where
    Context: CanSerializeValue<T>,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.context.serialize(self.value, serializer)
    }
}
