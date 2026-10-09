use std::fs;
use std::path::PathBuf;
use std::process::{Child, Command};
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(Debug, Clone, serde::Serialize)]
pub struct SupervisorStatus {
    pub is_running: bool,
    pub pid: Option<u32>,
    pub port: u16,
    pub model_path: PathBuf,
}

#[derive(Debug, thiserror::Error)]
pub enum SupervisorError {
    #[error("I/O Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("HTTP Error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Timeout waiting for model to load")]
    Timeout,
    #[error("Llamafile binary failed to start")]
    SpawnFailed,
}

pub struct LlamafileSupervisor {
    model_path: PathBuf,
    port: u16,
    child: Arc<Mutex<Option<Child>>>,
    http_client: reqwest::Client,
}

impl LlamafileSupervisor {
    pub fn new(model_path: PathBuf, port: u16) -> Self {
        Self {
            model_path,
            port,
            child: Arc::new(Mutex::new(None)),
            http_client: reqwest::Client::builder()
                .timeout(Duration::from_secs(2))
                .build()
                .unwrap_or_else(|_| reqwest::Client::new()),
        }
    }

    /// Spawns the llamafile binary and waits until /health or /v1/models responds
    pub async fn start(&self, timeout: Duration) -> Result<(), SupervisorError> {
        // 1. Check if the port is already in use by a compatible service
        let check_url = format!("http://127.0.0.1:{}/v1/models", self.port);
        #[allow(clippy::collapsible_if)]
        if let Ok(resp) = self.http_client.get(&check_url).send().await {
            if resp.status().is_success() {
                // Service is already running, bind to it.
                return Ok(());
            }
        }

        // 2. Ensure executable permissions (Linux/macOS)
        let metadata = fs::metadata(&self.model_path)?;
        let mut perms = metadata.permissions();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if perms.mode() & 0o111 == 0 {
                perms.set_mode(perms.mode() | 0o111);
                fs::set_permissions(&self.model_path, perms)?;
            }
        }

        // 3. Spawn the llamafile process
        // Spawns the executable with flags: `--server --jinja --port <port> --host 127.0.0.1 -ngl 999`
        let child = Command::new(&self.model_path)
            .arg("--server")
            .arg("--jinja")
            .arg("--port")
            .arg(self.port.to_string())
            .arg("--host")
            .arg("127.0.0.1")
            .arg("-ngl")
            .arg("999")
            .spawn()?;

        {
            let mut child_guard = self.child.lock().unwrap();
            *child_guard = Some(child);
        }

        // 4. Poll health endpoint with exponential backoff
        let start_time = tokio::time::Instant::now();
        let mut backoff = Duration::from_millis(100);
        let max_backoff = Duration::from_secs(2);

        while start_time.elapsed() < timeout {
            if self.is_ready().await {
                return Ok(());
            }

            // Check if process has died unexpectedly
            #[allow(clippy::collapsible_if)]
            if let Ok(mut guard) = self.child.lock() {
                if let Some(child) = guard.as_mut() {
                    if let Ok(Some(_status)) = child.try_wait() {
                        return Err(SupervisorError::SpawnFailed);
                    }
                }
            }

            tokio::time::sleep(backoff).await;
            backoff = std::cmp::min(backoff * 2, max_backoff);
        }

        self.stop()?;
        Err(SupervisorError::Timeout)
    }

    /// Polls the health endpoint
    pub async fn is_ready(&self) -> bool {
        let health_url = format!("http://127.0.0.1:{}/health", self.port);
        #[allow(clippy::collapsible_if)]
        if let Ok(resp) = self.http_client.get(&health_url).send().await {
            if resp.status().is_success() {
                return true;
            }
        }

        // Fallback to /v1/models if /health doesn't exist on this particular llamafile build
        let models_url = format!("http://127.0.0.1:{}/v1/models", self.port);
        #[allow(clippy::collapsible_if)]
        if let Ok(resp) = self.http_client.get(&models_url).send().await {
            if resp.status().is_success() {
                return true;
            }
        }

        false
    }

    /// Cleanly terminates the process
    pub fn stop(&self) -> Result<(), SupervisorError> {
        let mut child_guard = self.child.lock().unwrap();
        if let Some(mut child) = child_guard.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
        Ok(())
    }

    /// Inspect current process state
    pub fn status(&self) -> SupervisorStatus {
        let mut is_running = false;
        let mut pid = None;

        #[allow(clippy::collapsible_if)]
        if let Ok(mut child_guard) = self.child.lock() {
            if let Some(child) = child_guard.as_mut() {
                pid = Some(child.id());
                // try_wait returns Ok(None) if the process is still running
                if let Ok(None) = child.try_wait() {
                    is_running = true;
                }
            }
        }

        // If we attached to an existing process, the child lock is None but the service might be running
        if !is_running && pid.is_none() {
            // we could theoretically do a sync check here, but status should ideally not block.
            // For now, if we didn't spawn it, we just consider it running if we attached to it,
            // but this method is typically called to check our spawned child.
            // Since it's a sync method, we just return the child status.
        }

        SupervisorStatus {
            is_running,
            pid,
            port: self.port,
            model_path: self.model_path.clone(),
        }
    }
}

/// Implement Drop for LlamafileSupervisor to guarantee child cleanup
impl Drop for LlamafileSupervisor {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supervisor_initial_state() {
        let supervisor = LlamafileSupervisor::new(PathBuf::from("/dummy/path"), 8080);
        let status = supervisor.status();
        assert!(!status.is_running);
        assert_eq!(status.pid, None);
        assert_eq!(status.port, 8080);
        assert_eq!(status.model_path, PathBuf::from("/dummy/path"));
    }
}
