use aws_config::BehaviorVersion;
use aws_sdk_s3::Client;
use aws_sdk_s3::config::{Credentials, Region};

#[derive(Clone)]
pub struct S3Config {
    pub endpoint: String,
    pub bucket: String,
    pub region: String,
    pub access_key: String,
    pub secret_key: String,
}

impl S3Config {
    pub fn from_env_or_default() -> Self {
        let host = std::env::var("S3_HOST").unwrap_or_else(|_| "http://127.0.0.1".to_string());
        let port = std::env::var("S3_PORT").unwrap_or_else(|_| "9000".to_string());
        let endpoint = std::env::var("S3_ENDPOINT").unwrap_or_else(|_| format!("{host}:{port}"));

        Self {
            endpoint,
            bucket: std::env::var("S3_BUCKET").unwrap_or_else(|_| "my-bucket".to_string()),
            region: std::env::var("S3_REGION").unwrap_or_else(|_| "us-east-1".to_string()),
            access_key: std::env::var("S3_ACCESS_KEY").unwrap_or_else(|_| "minio".to_string()),
            secret_key: std::env::var("S3_SECRET_KEY").unwrap_or_else(|_| "minio123".to_string()),
        }
    }

    pub async fn create_client(&self) -> Client {
        let creds = Credentials::new(
            self.access_key.clone(),
            self.secret_key.clone(),
            None,
            None,
            "env-config",
        );

        let mut loader = aws_config::defaults(BehaviorVersion::latest())
            .region(Region::new(self.region.clone()))
            .credentials_provider(creds);

        if !self.endpoint.is_empty() {
            loader = loader.endpoint_url(self.endpoint.clone());
        }

        let conf = loader.load().await;
        Client::new(&conf)
    }
}
