use chrono::Utc;
use redis::{aio::MultiplexedConnection, AsyncCommands};

use crate::storage::types::ChatMessage;

pub async fn save_message(
    conn: &mut MultiplexedConnection,
    room: &str,
    user: &str,
    text: &str,
) -> redis::RedisResult<()> {
    let msg = ChatMessage {
        user: user.to_string(),
        room: room.to_string(),
        text: text.to_string(),
        timestamp: Utc::now().timestamp(),
    };

    let key = format!("chat:{}:messages", room);
    let serialized = serde_json::to_string(&msg).unwrap();

    let _:() = conn.lpush(&key, serialized).await?;
    let _:() = conn.ltrim(&key, 0, 99).await?;
    Ok(())
}