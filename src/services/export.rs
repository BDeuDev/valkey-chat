use aws_sdk_s3::Client as S3Client;
use redis::Client;

use crate::models::chat_message::Message;

#[derive(Clone)]
pub struct ExportService {
    pub export_path: String,
    pub redis_client: Client,
    pub s3_client: S3Client,
    pub bucket: Option<String>,
}

impl ExportService {
    pub fn new(export_path: String, s3_client: S3Client, redis_client: Client, bucket: Option<String>) -> Self {
        Self {
            export_path,
            s3_client,
            redis_client,
            bucket,
        }
    }

    pub async fn export_to_local(&self, messages: Vec<Message>) -> anyhow::Result<()> {
        crate::storage::parquet::export(messages, &self.export_path)?;
        Ok(())
    }
}