//! Exercise the actual process, including stdin ownership and capability discovery.
use serde_json::{Value, json};
use std::process::Stdio;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::Command;
use tokio::time::{sleep, timeout};

async fn exchange_initialize(delay: bool, split: bool) {
    let mut child = Command::new(env!("CARGO_BIN_EXE_paperpilot-mcp"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .kill_on_drop(true)
        .spawn()
        .unwrap();
    let mut input = child.stdin.take().unwrap();
    let mut output = BufReader::new(child.stdout.take().unwrap()).lines();
    if delay {
        sleep(Duration::from_millis(500)).await;
    }
    let request = format!(
        "{}\n",
        json!({
            "jsonrpc": "2.0", "id": 1, "method": "initialize",
            "params": {
                "protocolVersion": "2025-03-26", "capabilities": {},
                "clientInfo": {"name": "compatibility-test", "version": "1.0"}
            }
        })
    );
    if split {
        input.write_all(&request.as_bytes()[..15]).await.unwrap();
        input.flush().await.unwrap();
        sleep(Duration::from_millis(500)).await;
        input.write_all(&request.as_bytes()[15..]).await.unwrap();
    } else {
        input.write_all(request.as_bytes()).await.unwrap();
    }
    input.flush().await.unwrap();
    let response: Value = serde_json::from_str(
        &timeout(Duration::from_secs(10), output.next_line())
            .await
            .expect("initialize timed out")
            .unwrap()
            .expect("server closed stdout"),
    )
    .unwrap();
    assert_eq!(response["id"], 1);
    assert!(response.get("error").is_none(), "{response}");
    assert!(response["result"]["capabilities"]["tools"].is_object());

    for request in [
        json!({"jsonrpc": "2.0", "method": "notifications/initialized"}),
        json!({"jsonrpc": "2.0", "id": 2, "method": "tools/list", "params": {}}),
    ] {
        input
            .write_all(format!("{request}\n").as_bytes())
            .await
            .unwrap();
    }
    input.flush().await.unwrap();
    let response: Value = serde_json::from_str(
        &timeout(Duration::from_secs(10), output.next_line())
            .await
            .expect("tools/list timed out")
            .unwrap()
            .unwrap(),
    )
    .unwrap();
    assert_eq!(response["id"], 2);
    let tools = response["result"]["tools"].as_array().unwrap();
    assert!(tools.iter().any(|tool| tool["name"] == "pdf_merge"));

    // A routed tool error must keep JSON-RPC framing and leave the process alive.
    let request = json!({
        "jsonrpc": "2.0", "id": 3, "method": "tools/call",
        "params": {"name": "unknown_compatibility_test_tool", "arguments": {}}
    });
    input
        .write_all(format!("{request}\n").as_bytes())
        .await
        .unwrap();
    input.flush().await.unwrap();
    let response: Value = serde_json::from_str(
        &timeout(Duration::from_secs(10), output.next_line())
            .await
            .expect("tools/call timed out")
            .unwrap()
            .unwrap(),
    )
    .unwrap();
    assert_eq!(response["id"], 3);
    assert!(response["error"].is_object(), "{response}");

    // Exercise a successful document operation through the same stdio session.
    let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../tests/fixtures/simple.pdf")
        .canonicalize()
        .unwrap();
    let temp = tempfile::tempdir().unwrap();
    let merged = temp.path().join("merged.pdf");
    let request = json!({
        "jsonrpc": "2.0", "id": 4, "method": "tools/call",
        "params": {
            "name": "pdf_merge",
            "arguments": {"inputs": [fixture, fixture], "output": merged}
        }
    });
    input
        .write_all(format!("{request}\n").as_bytes())
        .await
        .unwrap();
    input.flush().await.unwrap();
    let response: Value = serde_json::from_str(
        &timeout(Duration::from_secs(30), output.next_line())
            .await
            .expect("pdf_merge timed out")
            .unwrap()
            .unwrap(),
    )
    .unwrap();
    assert_eq!(response["id"], 4);
    assert!(response.get("error").is_none(), "{response}");
    assert_ne!(response["result"]["isError"], true, "{response}");
    let original_pages = lopdf::Document::load(&fixture).unwrap().get_pages().len();
    assert_eq!(
        lopdf::Document::load(&merged).unwrap().get_pages().len(),
        original_pages * 2
    );
    child.kill().await.unwrap();
    child.wait().await.unwrap();
}

#[tokio::test]
async fn immediate_initialize() {
    exchange_initialize(false, false).await;
}

#[tokio::test]
async fn delayed_initialize() {
    exchange_initialize(true, false).await;
}

#[tokio::test]
async fn fragmented_initialize() {
    exchange_initialize(false, true).await;
}

#[test]
fn client_config_output_is_machine_readable() {
    let command = "C:\\Program Files\\PaperPilot\\paperpilot-mcp.exe";
    for client in [
        "codex",
        "claude-code",
        "kimi-code",
        "pi",
        "claude-desktop",
        "cursor",
    ] {
        let output = std::process::Command::new(env!("CARGO_BIN_EXE_paperpilot-mcp"))
            .args(["--client-config", client, "--server-command", command])
            .output()
            .unwrap();
        assert!(output.status.success(), "{client}");
        assert!(output.stderr.is_empty());
        let text = std::str::from_utf8(&output.stdout).unwrap();
        if client == "codex" {
            let config: toml::Value = toml::from_str(text).unwrap();
            assert_eq!(
                config["mcp_servers"]["paperpilot"]["command"].as_str(),
                Some(command)
            );
        } else {
            let config: Value = serde_json::from_str(text).unwrap();
            assert_eq!(config["mcpServers"]["paperpilot"]["command"], command);
        }
    }
}

#[test]
fn invalid_or_conflicting_config_flags_fail() {
    for args in [
        vec!["--client-config", "unknown"],
        vec!["--client-config", "pi", "--direct-tool-call"],
        vec!["--server-command", "paperpilot-mcp"],
    ] {
        assert!(
            !std::process::Command::new(env!("CARGO_BIN_EXE_paperpilot-mcp"))
                .args(args)
                .output()
                .unwrap()
                .status
                .success()
        );
    }
}
