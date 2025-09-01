use redis::Client;

#[derive(Clone)]
pub struct ValkeyConfig {
    pub host: String,
    pub port: u16,
}

impl ValkeyConfig {
    pub fn load_env_or_default() -> Self {
        let host = std::env::var("VALKEY_HOST").unwrap_or_else(|_| "valkey".to_string());
        let port = std::env::var("VALKEY_PORT")
            .unwrap_or_else(|_| "6379".to_string())
            .parse()
            .unwrap_or(6379);

        Self { host, port }
    }

    fn to_url(&self) -> String {
        format!("redis://{}:{}", self.host, self.port)
    }

    pub async fn create_client(&self) -> Result<Client, redis::RedisError>{
        Client::open(self.to_url()) 
    }

}