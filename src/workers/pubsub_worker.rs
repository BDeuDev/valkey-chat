use redis::aio::{PubSub, MultiplexedConnection};
use redis::{AsyncCommands, Client};
use futures_util::StreamExt;

use crate::services::trigger_service::TriggerService;

pub struct PubSubWorker {
    client: Client,
    trigger_service: TriggerService,
    channel: String,
    limit: i64,
}

impl PubSubWorker {
    pub fn new(client: Client, trigger_service: TriggerService, channel: &str, limit: i64) -> Self {
        Self {
            client,
            trigger_service,
            channel: channel.to_string(),
            limit,
        }
    }

    pub async fn run(self) -> redis::RedisResult<()> {
        // 🔹 1: crear conexión async normal y convertirla en PubSub
        let mut pubsub: PubSub = self.client
            .get_multiplexed_async_connection()
            .await?
            .into

        pubsub.subscribe(&self.channel).await?;
        let mut stream = pubsub.on_message();

        // 🔹 2: otra conexión multiplexada para hacer INCR
        let mut conn: MultiplexedConnection = self.client.get_multiplexed_tokio_connection().await?;

        while let Some(msg) = stream.next().await {
            let _: String = msg.get_payload()?; // mensaje publicado (ej: "nuevo_registro")

            let count: i64 = conn.incr("chat:counter", 1).await?;
            println!("📨 Nuevo evento recibido. Contador = {}", count);

            if count % self.limit == 0 {
                println!("🚀 Se alcanzó el límite de {} eventos", self.limit);
                if let Err(err) = self.trigger_service.trigger().await {
                    eprintln!("⚠️ Error al disparar endpoint: {:?}", err);
                }
            }
        }

        Ok(())
    }
}
