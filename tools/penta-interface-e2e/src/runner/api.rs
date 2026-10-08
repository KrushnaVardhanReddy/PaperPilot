use anyhow::Result;
use reqwest::Client;
use serde_json::Value;
use std::time::Instant;

pub struct ApiRunner {
    client: Client,
    base_url: String,
}

impl ApiRunner {
    pub fn new(port: u16) -> Self {
        Self {
            client: Client::new(),
            base_url: format!("http://127.0.0.1:{}", port),
        }
    }

    pub async fn post_json(&self, endpoint: &str, payload: Value) -> Result<(bool, Value, f64)> {
        let url = format!("{}{}", self.base_url, endpoint);
        let start = Instant::now();
        let resp = self.client.post(&url).json(&payload).send().await?;
        let latency_ms = start.elapsed().as_secs_f64() * 1000.0;
        let status_ok = resp.status().is_success();
        let json_body: Value = resp.json().await.unwrap_or(Value::Null);
        Ok((status_ok, json_body, latency_ms))
    }
}
