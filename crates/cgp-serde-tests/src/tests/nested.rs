use cgp::prelude::*;
use cgp_serde::components::ValueSerializerComponent;
use cgp_serde::providers::{SerializeDeref, SerializeFields, SerializeIterator, UseSerde};
use cgp_serde::types::SerializeWithContext;
use cgp_serde_extra::providers::{SerializeHex, SerializeRfc3339Date};
use chrono::{DateTime, TimeZone, Utc};

#[derive(HasField, HasFields)]
pub struct EncryptedMessage {
    pub message_id: u64,
    pub author_id: u64,
    pub date: DateTime<Utc>,
    pub encrypted_data: Vec<u8>,
}

#[derive(HasField, HasFields)]
pub struct MessagesByTopic {
    pub encrypted_topic: Vec<u8>,
    pub messages: Vec<EncryptedMessage>,
}

#[derive(HasField, HasFields)]
pub struct MessagesArchive {
    pub decryption_key: Vec<u8>,
    pub messages_by_topics: Vec<MessagesByTopic>,
}

#[cgp_context]
pub struct App;

delegate_components! {
    AppComponents {
        ValueSerializerComponent:
            UseDelegate<new SerializerComponents {
                <'a, T> &'a T:
                    SerializeDeref,
                [
                    u64,
                    String,
                ]:
                    UseSerde,
                Vec<u8>:
                    SerializeHex,
                DateTime<Utc>:
                    SerializeRfc3339Date,
                [
                    Vec<EncryptedMessage>,
                    Vec<MessagesByTopic>,
                ]:
                    SerializeIterator,
                [
                    MessagesArchive,
                    MessagesByTopic,
                    EncryptedMessage,
                ]:
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
            DateTime<Utc>,
            EncryptedMessage,
            MessagesByTopic,
            MessagesArchive,
        ]
    }
}

#[test]
fn test_nested_serialization() {
    let archive = MessagesArchive {
        decryption_key: b"top-secret".into(),
        messages_by_topics: vec![MessagesByTopic {
            encrypted_topic: b"secret-deals".into(),
            messages: vec![
                EncryptedMessage {
                    message_id: 1,
                    author_id: 1,
                    date: Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap(),
                    encrypted_data: b"buy 1 free 1".into(),
                },
                EncryptedMessage {
                    message_id: 2,
                    author_id: 8,
                    date: Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap(),
                    encrypted_data: b"sales start tomorrow".into(),
                },
            ],
        }],
    };

    let serialized = serde_json::to_string(&SerializeWithContext::new(&App, &archive)).unwrap();
    println!("serialized: {serialized}")
}
