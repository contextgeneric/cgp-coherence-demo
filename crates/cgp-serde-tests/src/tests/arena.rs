use cgp::prelude::*;
use cgp_serde::components::{
    CanDeserializeValueFrom, ValueDeserializerComponent, ValueFromDeserializerComponent,
};
use cgp_serde::providers::{
    DeserializeRecordFields, SerializeIterator, TrySerializeFrom, UseSerde,
};
use cgp_serde_alloc::providers::DeserailizeAndAllocate;
use cgp_serde_alloc::traits::AllocatorComponent;
use cgp_serde_json::DeserializeFromJsonString;
use cgp_serde_typed_arena::providers::AllocateWithArena;
use cgp_serde_typed_arena::traits::ArenaGetterComponent;
use typed_arena::Arena;

pub type Data = [u8; 4];

#[derive(HasFields, BuildField)]
pub struct Payload<'a> {
    pub id: u64,
    pub data: &'a Data,
}

#[cgp_context]
#[derive(HasField)]
pub struct App<'a> {
    pub arena: &'a Arena<Data>,
}

delegate_components! {
    AppComponents {
        ArenaGetterComponent:
            UseField<Symbol!("arena")>,
        AllocatorComponent:
            AllocateWithArena,
        ValueDeserializerComponent:
            UseDelegate<new DeserializeComponents {
                u64: UseSerde,
                Vec<u8>: SerializeIterator,
                Data: TrySerializeFrom<Vec<u8>>,
                <'a> &'a Data: DeserailizeAndAllocate,
                <'a> Payload<'a>: DeserializeRecordFields,
            }>,
        ValueFromDeserializerComponent:
            DeserializeFromJsonString,
    }
}

check_components! {
    <'a> CanUseApp for App<'a> {
        ArenaGetterComponent:
            (&'a (), Data),

    }
}

check_components! {
    <'de, 'a> CanDeserializeApp for App<'a> {
        ValueDeserializerComponent: [
            (&'de (), u64),
            (&'de (), Data),
            (&'de (), &'a Data),
            (&'de (), Payload<'a>),
        ]
    }
}

#[test]
fn test_deserialize_with_arena() {
    let serialized = r#"
{ "id": 8, "data": [1, 2, 3, 4] }
"#;

    let arena = Arena::new();
    let app = App { arena: &arena };

    let deserialized: Payload<'_> = app.deserialize_from(&serialized).unwrap();
    assert_eq!(deserialized.id, 8);
    assert_eq!(deserialized.data, &[1, 2, 3, 4]);
}
