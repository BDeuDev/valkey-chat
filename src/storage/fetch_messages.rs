use crate::storage::types::ChatMessage;
use redis::{aio::MultiplexedConnection, AsyncCommands};
pub async fn fetch_messages(
    conn: &mut MultiplexedConnection,
    room: &str,
) -> redis::RedisResult<Vec<ChatMessage>> {
    let key = format!("chat:{}:messages", room);
    let raw: Vec<String> = conn.lrange(&key, 0, -1).await?;
    Ok(raw.into_iter().filter_map(|r| serde_json::from_str(&r).ok()).collect())
}