use cgp::prelude::*;
use cgp_serde::components::ValueSerializerComponent;
use cgp_serde::providers::{SerializeFields, UseSerde};
use cgp_serde::types::SerializeWithContext;
use cgp_serde_extra::providers::SerializeHex;

#[derive(HasField, HasFields)]
pub struct Message {
    pub message_id: u64,
    pub author_id: u64,
    pub data: Vec<u8>,
}

// pub struct MessagesBy

#[cgp_context]
pub struct App;

delegate_components! {
    AppComponents {
        ValueSerializerComponent:
            UseDelegate<new SerializerComponents {
                [
                    u64,
                    String,
                ]:
                    UseSerde,
                Vec<u8>:
                    SerializeHex,
                Message:
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
            Message,
        ]
    }
}
