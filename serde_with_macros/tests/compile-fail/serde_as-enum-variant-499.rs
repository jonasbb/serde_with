use serde::{Deserialize, Serialize};
use serde_with_macros::serde_as;

#[serde_as]
#[derive(Serialize, Deserialize)]
#[serde(tag = "messageType", content = "content")]
/// The contents of a message combined with the `MessageType`
pub enum MessageContentJsonStringEnum {
    /// A normal message
    Text(String),
    /// Fancy object message
    #[serde_as(as = "serde_with::json::JsonString")]
    Object(String),
}

fn main() {}
