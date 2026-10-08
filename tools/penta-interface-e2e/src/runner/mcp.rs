use std::process::Stdio;
use tokio::process::Command;
use tokio::io::{AsyncWriteExt, AsyncBufReadExt, BufReader};
use std::time::Instant;
use serde_json::{json, Value};
use anyhow::{Result, anyhow};

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
            .spawn()?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(payload_str.as_bytes()).await?;
            stdin.flush().await?;
            drop(stdin); // Close stdin so child process completes reading immediately
        }

        let output = child.wait_with_output().await?;
        let stdout_str = String::from_utf8_lossy(&output.stdout);
        let stderr_str = String::from_utf8_lossy(&output.stderr);
        let latency_ms = start.elapsed().as_secs_f64() * 1000.0;

        let mut response_val: Option<Value> = None;
        for line in stdout_str.lines() {
            if let Ok(v) = serde_json::from_str::<Value>(line) {
                if v.get("id").is_some() || v.get("result").is_some() || v.get("error").is_some() {
                    response_val = Some(v);
                    break;
                }
            }
        }

        if let Some(resp) = response_val {
            let is_error = resp.get("error").is_some() ||
                resp.get("result").and_then(|r| r.get("isError")).and_then(|e| e.as_bool()).unwrap_or(false);
            Ok((!is_error, resp, latency_ms))
        } else if !stderr_str.trim().is_empty() {
            // Server exited with stderr
            let err_obj = serde_json::json!({ "error": stderr_str.trim() });
            Ok((false, err_obj, latency_ms))
        } else {
            Err(anyhow!("No valid JSON-RPC response from MCP server. stdout='{}', stderr='{}'", stdout_str, stderr_str))
        }
    }
}
