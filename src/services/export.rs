use aws_sdk_s3::Client as S3Client;

use crate::AppState;

#[derive(Clone)]
pub struct ExportService {
    pub export_path: String,
    pub s3_client: Option<S3Client>,
    pub bucket: Option<String>,
}

impl ExportService {
    pub fn new(export_path: String, s3_client: Option<S3Client>, bucket: Option<String>) -> Self {
        Self {
            export_path,
            s3_client,
            bucket,
        }
    }

    pub async fn export_to_local(&self, state: &AppState) -> anyhow::Result<()> {
        let mut conn = state.redis_client.get_multiplexed_async_connection().await?;
        let msgs = crate::storage::messages::fetch_messages(&mut conn, "general").await?;
        crate::storage::parquet::export(msgs, &self.export_path)?;
        Ok(())
    }
}