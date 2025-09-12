use anyhow::Ok;
use aws_sdk_s3::{Client as S3Client, primitives::ByteStream};
use redis::Client;
use tokio::{fs::File, io::AsyncReadExt};

use crate::models::chat_message::Message;

#[derive(Clone)]
pub struct ExportService {
    pub export_path: String,
    pub redis_client: Client,
    pub s3_client: S3Client,
    pub bucket: Option<String>,
}

impl ExportService {
    pub fn new(
        export_path: String,
        s3_client: S3Client,
        redis_client: Client,
        bucket: Option<String>,
    ) -> Self {
        Self {
            export_path,
            s3_client,
            redis_client,
            bucket,
        }
    }

    pub async fn export_to_local(&self, messages: Vec<Message>) -> anyhow::Result<()> {
        crate::storage::parquet::write_file(messages, &self.export_path)?;
        Ok(())
    }

    pub async fn export_to_s3(
        &self,
        messages: Vec<Message>,
        file_name: &str,
    ) -> anyhow::Result<()> {
        crate::storage::parquet::write_file(messages, &self.export_path)?;

        let mut file = File::open(&self.export_path).await?;
        let mut buffer = Vec::new();

        file.read_to_end(&mut buffer).await?;

        let bucket = std::env::var("S3_BUCKET").unwrap_or_else(|_| "my-bucket".to_string());
        let key = format!("messages-{}.parquet", file_name);

        self.s3_client
            .put_object()
            .bucket(bucket)
            .key(key)
            .body(ByteStream::from(buffer))
            .send()
            .await?;

        Ok(())
    }
}
