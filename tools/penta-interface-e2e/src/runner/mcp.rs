use anyhow::{anyhow, Result};
use serde_json::{json, Value};
use std::process::Stdio;
use std::time::Instant;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio::process::Command;

pub struct McpRunner;

impl McpRunner {
    pub async fn call_tool(tool_name: &str, args: Value) -> Result<(bool, Value, f64)> {
        let bin = super::get_mcp_bin();
        let payload = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "tools/call",
            "params": {
                "name": tool_name,
                "arguments": args
            }
        });
        let payload_str = format!("{}\n", serde_json::to_string(&payload)?);

        let start = Instant::now();
        let mut child = Command::new(&bin)
            .arg("--direct-tool-call")
            .current_dir(std::env::current_dir()?)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| anyhow!("Failed to spawn MCP server: {}", e))?;

        let mut stdin = child.stdin.take().unwrap();
        stdin.write_all(payload_str.as_bytes()).await?;
        stdin.flush().await?;
        drop(stdin);

        let stdout = child.stdout.take().unwrap();
        let mut reader = tokio::io::BufReader::new(stdout);
        let mut response_line = String::new();
        reader.read_line(&mut response_line).await?;

        let stderr = child.stderr.take().unwrap();
        let mut err_reader = tokio::io::BufReader::new(stderr);
        let mut err_str = String::new();
        err_reader.read_line(&mut err_str).await?;

        let duration = start.elapsed().as_secs_f64() * 1000.0;

        let out: Value = serde_json::from_str(&response_line)
            .unwrap_or_else(|_| json!({"error": {"message": response_line, "stderr": err_str}}));

        if let Some(err) = out.get("error") {
            Ok((false, err.clone(), duration))
        } else if let Some(res) = out.get("result") {
            let is_error = res.get("isError").and_then(|v| v.as_bool()).unwrap_or(false);
            Ok((!is_error, res.clone(), duration))
        } else {
            Ok((false, json!("No valid result or error found"), duration))
        }
    }
}
