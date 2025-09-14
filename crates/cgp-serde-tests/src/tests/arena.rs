use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::extra::handler::CanTryCompute;
use cgp::prelude::*;
use cgp_error_anyhow::{RaiseAnyhowError, UseAnyhowError};
use cgp_serde::components::{ValueDeserializerComponent, ValueFromDeserializerComponent};
use cgp_serde::providers::{DeserializeExtend, DeserializeRecordFields, UseSerde};
use cgp_serde_alloc::providers::DeserailizeAndAllocate;
use cgp_serde_alloc::traits::AllocatorComponent;
use cgp_serde_json::code::{DeserializeJson, SerializeJson};
use cgp_serde_json::{DeserializeFromJsonString, SerializeToJsonString};
use cgp_serde_typed_arena::providers::AllocateWithArena;
use cgp_serde_typed_arena::traits::ArenaGetterComponent;
use typed_arena::Arena;

#[derive(Debug, PartialEq, Eq, HasFields, BuildField)]
pub struct Coord {
    pub x: u64,
    pub y: u64,
    pub z: u64,
}

#[derive(Debug, PartialEq, Eq, HasFields, BuildField)]
pub struct Payload<'a> {
    pub id: u64,
    pub coords: Vec<&'a Coord>,
}

#[cgp_context]
#[derive(HasField)]
pub struct App<'a> {
    pub arena: &'a Arena<Coord>,
}

delegate_components! {
    AppComponents {
        ErrorTypeProviderComponent:
            UseAnyhowError,
        ErrorRaiserComponent:
            RaiseAnyhowError,
        ArenaGetterComponent:
            UseField<Symbol!("arena")>,
        AllocatorComponent:
            AllocateWithArena,
        ValueDeserializerComponent:
            UseDelegate<new DeserializeComponents {
                u64: UseSerde,
                Coord:
                    DeserializeRecordFields,
                <'a> &'a Coord:
                    DeserailizeAndAllocate,
                <'a> Vec<&'a Coord>:
                    DeserializeExtend,
                <'a> Payload<'a>:
                    DeserializeRecordFields,

            }>,
        TryComputerComponent:
            UseDelegate<new JsonEncodingComponents {
                SerializeJson:
                    SerializeToJsonString,
                <T> DeserializeJson<T>:
                    DeserializeFromJsonString
            }>,
        ValueFromDeserializerComponent:
            DeserializeFromJsonString,
    }
}

check_components! {
    <'a> CanUseApp for App<'a> {
        ArenaGetterComponent:
            (&'a (), Coord),

    }
}

check_components! {
    <'de, 'a> CanDeserializeApp for App<'a> {
        ValueDeserializerComponent: [
            (&'de (), u64),
            (&'de (), Coord),
            (&'de (), &'a Coord),
            (&'de (), Payload<'a>),
        ]
    }
}

#[test]
fn test_deserialize_with_arena() {
    let serialized = r#"
{
    "id": 8,
    "coords": [
        { "x": 1, "y": 2, "z": 3 },
        { "x": 4, "y": 5, "z": 6 }
    ]
}
"#;

    let arena = Arena::new();
    let app = App { arena: &arena };

    let deserialized: Payload<'_> = app
        .try_compute(PhantomData::<DeserializeJson<Payload<'_>>>, &serialized)
        .unwrap();
    assert_eq!(
        deserialized,
        Payload {
            id: 8,
            coords: vec![&Coord { x: 1, y: 2, z: 3 }, &Coord { x: 4, y: 5, z: 6 },]
        }
    );
}
