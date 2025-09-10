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
        let res = self.http.post(&self.endpoint_url).send().await?;
        if res.status().is_success() {
            println!("✅ Endpoint disparado con éxito: {}", self.endpoint_url);
        } else {
            eprintln!("❌ Falló el endpoint: {} -> {:?}", self.endpoint_url, res.status());
        }
        Ok(())
    }
}
