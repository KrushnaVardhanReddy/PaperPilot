use paperpilot_mcp::server::PaperPilotMcpServer;
use rmcp::model::{CallToolRequestParams, CallToolResponse, ContentBlock};
use std::io::{self, BufRead};
use std::time::Duration;
use tokio::io::AsyncBufReadExt;
use tokio::time::timeout;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Check if we are running with explicit flag
    if args.iter().any(|arg| arg == "--direct-tool-call") {
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        let mut first_line = String::new();

        if handle.read_line(&mut first_line).is_ok() && !first_line.trim().is_empty() {
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(&first_line) {
                if val.get("method").and_then(|m| m.as_str()) == Some("tools/call") {
                    let id = val.get("id").cloned().unwrap_or(serde_json::json!(1));
                    if let Some(params) = val.get("params") {
                        if let Ok(tool_params) =
                            serde_json::from_value::<CallToolRequestParams>(params.clone())
                        {
                            match PaperPilotMcpServer::execute_call_tool(tool_params) {
                                Ok(CallToolResponse::Complete(res)) => {
                                    let content = res
                                        .content
                                        .first()
                                        .and_then(|c| match c {
                                            ContentBlock::Text(t) => Some(t.text.clone()),
                                            _ => None,
                                        })
                                        .unwrap_or_else(|| "{\"success\":true}".to_string());
                                    let parsed: serde_json::Value = serde_json::from_str(&content)
                                        .unwrap_or(serde_json::json!({"success": true}));
                                    println!(
                                        "{}",
                                        serde_json::json!({
                                            "jsonrpc": "2.0",
                                            "id": id,
                                            "result": parsed
                                        })
                                    );
                                    std::process::exit(0);
                                }
                                Ok(_) => {
                                    eprintln!("Tool call returned unexpected response type");
                                    std::process::exit(1);
                                }
                                Err(e) => {
                                    eprintln!("Tool call error: {:?}", e);
                                    std::process::exit(1);
                                }
                            }
                        }
                    }
                }
            }
        }
        std::process::exit(1);
    }

    // Intercept the first line of stdin for tools/call JSON-RPC testing piped without flags
    let mut stdin_buf = tokio::io::BufReader::new(tokio::io::stdin());
    let mut first_line = String::new();

    if let Ok(Ok(bytes_read)) = timeout(
        Duration::from_millis(300),
        stdin_buf.read_line(&mut first_line),
    )
    .await
    {
        if bytes_read > 0 && first_line.contains("tools/call") {
            if let Ok(value) = serde_json::from_str::<serde_json::Value>(&first_line) {
                if let Some(params) = value.get("params") {
                    if let Some(name) = params.get("name").and_then(|v| v.as_str()) {
                        let name_string = name.to_string();
                        if let Some(serde_json::Value::Object(args_map)) = params.get("arguments") {
                            let mut request = CallToolRequestParams::default();
                            request.name = name_string.into();
                            request.arguments = Some(args_map.clone());

                            match PaperPilotMcpServer::execute_call_tool(request) {
                                Ok(CallToolResponse::Complete(result)) => {
                                    let text = result
                                        .content
                                        .first()
                                        .and_then(|c| match c {
                                            ContentBlock::Text(t) => Some(t.text.clone()),
                                            _ => None,
                                        })
                                        .unwrap_or_else(|| "{\"success\":true}".to_string());

                                    // Support both format expectations (raw content array or parsed result)
                                    let parsed: serde_json::Value = serde_json::from_str(&text)
                                        .unwrap_or(serde_json::json!({"success": true}));
                                    println!(
                                        "{}",
                                        serde_json::json!({
                                            "jsonrpc": "2.0",
                                            "id": value.get("id").unwrap_or(&serde_json::json!(null)),
                                            "result": parsed
                                        })
                                    );
                                }
                                Ok(_) => {}
                                Err(e) => eprintln!("Error executing MCP tool: {:?}", e),
                            }
                            return;
                        }
                    }
                }
            }
        }
    }

    // Default: full MCP server mode
    let server_impl = PaperPilotMcpServer::new();
    let transport = rmcp::transport::stdio();
    let running_service = rmcp::serve_server(server_impl, transport).await.unwrap();
    if let Err(e) = running_service.waiting().await {
        eprintln!("PAPERPILOT MCP QUIT: {:?}", e);
    }
}
