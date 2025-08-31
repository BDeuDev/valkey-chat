use chrono::Utc;
use redis::{AsyncCommands, aio::MultiplexedConnection};

use crate::models::chat_message::ChatMessage;

pub async fn fetch_messages(
    conn: &mut MultiplexedConnection,
    room: &str,
) -> redis::RedisResult<Vec<ChatMessage>> {
    let key = format!("chat:{}:messages", room);
    let raw: Vec<String> = conn.lrange(&key, 0, -1).await?;
    Ok(raw
        .into_iter()
        .filter_map(|r| serde_json::from_str(&r).ok())
        .collect())
}

pub async fn save(
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

    let _: () = conn.lpush(&key, serialized).await?;
    let _: () = conn.ltrim(&key, 0, 99).await?;
    Ok(())
}
