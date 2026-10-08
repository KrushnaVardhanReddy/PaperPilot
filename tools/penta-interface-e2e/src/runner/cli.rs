use std::process::Stdio;
use tokio::process::Command;
use std::time::Instant;
use anyhow::Result;

pub struct CliRunner;

impl CliRunner {
    pub async fn run(args: &[&str]) -> Result<(bool, String, f64)> {
        let bin = super::get_cli_bin();
        let start = Instant::now();
        let output = Command::new(&bin)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await?;
        let latency_ms = start.elapsed().as_secs_f64() * 1000.0;
        let success = output.status.success();
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let out_msg = if success {
            if stdout.trim().is_empty() { stderr } else { stdout }
        } else {
            format!("Error (exit code {:?}): {}", output.status.code(), stderr)
        };
        Ok((success, out_msg, latency_ms))
    }
}
