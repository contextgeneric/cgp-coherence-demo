use cgp::prelude::*;
use cgp_serde::components::{
    CanDeserializeValueFrom, CanSerializeValueTo, ValueDeserializerComponent,
    ValueFromDeserializerComponent, ValueSerializerComponent, ValueToSerializerComponent,
};
use cgp_serde::providers::{DeserializeRecordFields, SerializeFields, SerializeString, UseSerde};
use cgp_serde_extra::providers::SerializeHex;
use cgp_serde_json::{DeserializeFromJsonString, SerializeToJsonString};
use serde::Deserialize;

#[derive(Debug, Eq, PartialEq, HasField, HasFields, BuildField, Deserialize)]
pub struct Payload {
    pub quantity: u64,
    pub message: String,
    pub data: Vec<u8>,
}

#[cgp_context]
pub struct App;

delegate_components! {
    AppComponents {
        ValueSerializerComponent:
            UseDelegate<new SerializerComponents {
                u64:
                    UseSerde,
                String:
                    SerializeString,
                Vec<u8>:
                    SerializeHex,
                Payload:
                    SerializeFields,
            }>,
        ValueDeserializerComponent:
            UseDelegate<new DeserializerComponents {
                [
                    u64,
                    String,
                ]:
                    UseSerde,
                Payload:
                    DeserializeRecordFields,
                Vec<u8>: SerializeHex,
            }>,
        ValueFromDeserializerComponent:
            DeserializeFromJsonString,
        ValueToSerializerComponent:
            SerializeToJsonString,
    }
}

check_components! {
    CanUseAppSerializer for App {
        ValueSerializerComponent: [
            u64,
            String,
            Vec<u8>,
            Payload,
        ],
    }
}

check_components! {
    <'de> CanUseAppDerializer for App {
        ValueDeserializerComponent: [
            (&'de (), u64),
            (&'de (), String),
            (&'de (), Vec<u8>),
            (&'de (), Payload),
        ]
    }
}

#[test]
fn test_basic_serialization() {
    let context = App;

    let value = Payload {
        quantity: 42,
        message: "hello".to_owned(),
        data: vec![1, 2, 3],
    };

    let serialized = context.serialize_to(&value).unwrap();

    assert_eq!(
        serialized,
        "{\"quantity\":42,\"message\":\"hello\",\"data\":\"010203\"}"
    );

    let deserialized: Payload = context.deserialize_from(&serialized).unwrap();

    assert_eq!(deserialized, value);
}
