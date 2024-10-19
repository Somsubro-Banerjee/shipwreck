// Message format for shared use between producer, consumer, and broker
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub key: String,
    pub value: String,
}

impl Message {
    pub fn new(key: &str, value: &str) -> Self {
        Self {
            key: key.to_string(),
            value: value.to_string(),
        }
    }
}

// Serialization and deserialization utilities
pub fn serialize_message(msg: &Message) -> Vec<u8> {
    serde_json::to_vec(msg).expect("Failed to serialize message")
}

pub fn deserialize_message(bytes: &[u8]) -> Message {
    serde_json::from_slice(bytes).expect("Failed to deserialize message")
}
