use crate::controllers::types::MessagePayload;
use crate::models::chat_message::Message;
use chrono::Utc;
use redis::{AsyncCommands, Client, aio::MultiplexedConnection};

#[derive(Clone)]
pub struct MessageService {
    redis_client: Client,
}

impl MessageService {
    pub fn new(redis_client: Client) -> Self {
        Self { redis_client }
    }

    pub async fn save_message(&self, payload: MessagePayload) -> redis::RedisResult<()> {
        let mut conn: MultiplexedConnection =
            self.redis_client.get_multiplexed_tokio_connection().await?;

        let msg = Message {
            user: payload.user,
            room: payload.room.clone(),
            text: payload.text,
            timestamp: Utc::now().timestamp(),
        };

        let key = format!("chat:{}:messages", payload.room);

        let serialized = serde_json::to_string(&msg).unwrap();

        let _: () = conn.lpush(&key, serialized).await?;

        let _: () = conn.ltrim(&key, 0, 99).await?;

        Ok(())
    }

    pub async fn get_recent_messages(&self, room: &str) -> Result<Vec<Message>,redis::RedisError> {
        let mut conn = self.redis_client.get_multiplexed_tokio_connection().await?;
        let key = format!("chat:{}:messages", room);

        let serialized: Vec<String> = conn.lrange(key, 0, 99).await?;
        let messages: Vec<Message> = serialized
            .into_iter()
            .filter_map(|s| serde_json::from_str(&s).ok())
            .collect();

        Ok(messages)
    }

    pub async fn get_all_messages(&self, room: &str) -> Result<Vec<Message>, redis::RedisError> {
        let mut conn = self.redis_client.get_multiplexed_tokio_connection().await?;
        let key = format!("chat:{}:messages", room);

        let serialized: Vec<String> = conn.lrange(key, 0, -1) .await?;
        let messages: Vec<Message> = serialized
            .into_iter()
            .filter_map(|s| serde_json::from_str(&s).ok())
            .collect();

        Ok(messages)
    }
}
