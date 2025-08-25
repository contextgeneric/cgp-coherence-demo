use cgp::prelude::*;
use cgp_serde::components::ValueSerializerComponent;
use cgp_serde::providers::{SerializeFields, UseSerde};
use cgp_serde::types::SerializeWithContext;

#[derive(HasField, HasFields)]
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
                [
                    u64,
                    String,
                    Vec<u8>,
                ]:
                    UseSerde,
                Payload:
                    SerializeFields,
            }>
    }
}

check_components! {
    CanUseApp for App {
        ValueSerializerComponent: [
            u64,
            String,
            Vec<u8>,
            Payload,
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

    let serialized = serde_json::to_string(&SerializeWithContext::new(&context, &value)).unwrap();
    println!("serialized: {serialized}");
}
