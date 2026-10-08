use anyhow::{anyhow, Result};
use std::process::Stdio;
use std::time::Duration;
use tokio::process::{Child, Command};

pub struct GatewayServer {
    child: Option<Child>,
    port: u16,
}

impl GatewayServer {
    pub fn new(port: u16) -> Self {
        Self { child: None, port }
    }

    pub async fn start(&mut self) -> Result<()> {
        let bin = super::get_cli_bin();
        let port_str = self.port.to_string();
        let child = Command::new(&bin)
            .args(["serve", "--port", &port_str])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
        self.child = Some(child);

        // Poll /health
        let client = reqwest::Client::new();
        let health_url = format!("http://127.0.0.1:{}/health", self.port);
        let start = std::time::Instant::now();

        while start.elapsed() < Duration::from_secs(15) {
            if let Ok(resp) = client.get(&health_url).send().await {
                if resp.status().is_success() {
                    return Ok(());
                }
            }
            tokio::time::sleep(Duration::from_millis(300)).await;
        }

        Err(anyhow!(
            "Gateway server failed to respond on :{} within 15s",
            self.port
        ))
    }

    pub async fn stop(&mut self) {
        if let Some(mut child) = self.child.take() {
            child.kill().await.ok();
        }
    }
}

impl Drop for GatewayServer {
    fn drop(&mut self) {
        if let Some(mut child) = self.child.take() {
            let _ = child.start_kill();
        }
    }
}
