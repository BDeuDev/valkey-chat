use reqwest::Client;

#[derive(Clone)]
pub struct TriggerService {
    http: Client,
    endpoint_url: String,
}

impl TriggerService {
    pub fn new(endpoint_url: String) -> Self {
        Self {
            http: Client::new(),
            endpoint_url,
        }
    }

    pub async fn trigger(&self) -> anyhow::Result<()> {
        let res = self.http.get(&self.endpoint_url).send().await?;
        if res.status().is_success() {
            println!("✅ Endpoint triggered successful: {}", self.endpoint_url);
        } else {
            eprintln!("❌ Endpoint failed: {} -> {:?}", self.endpoint_url, res.status());
        }
        Ok(())
    }
}
