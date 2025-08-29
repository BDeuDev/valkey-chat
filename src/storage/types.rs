use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub user: String,
    pub room: String,
    pub text: String,
    pub timestamp: i64,
}