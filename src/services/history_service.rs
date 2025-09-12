use actix_web::web::Bytes;
use anyhow::Result;
use aws_sdk_s3::Client;

#[derive(Clone)]
pub struct HistoryService {
    s3_client: Client,
}

impl HistoryService {
    pub fn new(s3_client: Client) -> Self {
        Self { s3_client }
    }

    pub async fn import_from_s3(&self, file_name: &str) -> Result<Bytes> {
        let bucket = std::env::var("S3_BUCKET").unwrap_or_else(|_| "my-bucket".to_string());
        let key = format!("messages-{}.parquet", file_name);

        let resp: aws_sdk_s3::operation::get_object::GetObjectOutput = self
            .s3_client
            .get_object()
            .bucket(&bucket)
            .key(&key)
            .send()
            .await?;

        let data: aws_sdk_s3::primitives::AggregatedBytes = resp.body.collect().await?;
        let bytes: actix_web::web::Bytes = data.into_bytes();

        Ok(bytes)
    }
}
