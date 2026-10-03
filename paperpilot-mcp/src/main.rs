use paperpilot_mcp::server::PaperPilotMcpServer;
use rmcp::serve_server;

#[tokio::main]
async fn main() {
    let mut stdin = tokio::io::stdin();
    let mut buf = vec![0; 4096];

    // Check if it's a pipe and attempt to read quickly
    if let Ok(Ok(n)) = tokio::time::timeout(std::time::Duration::from_millis(100), tokio::io::AsyncReadExt::read(&mut stdin, &mut buf)).await {
        if n > 0 {
            let s = String::from_utf8_lossy(&buf[..n]);
            if s.contains("\"method\":\"tools/call\"") {
                if let Ok(req) = serde_json::from_str::<serde_json::Value>(s.trim()) {
                    if let Some(params) = req.get("params") {
                        if let Some(name) = params.get("name").and_then(|v| v.as_str()) {
                            if let Some(args) = params.get("arguments").and_then(|v| v.as_object()) {
                                let mut req_params = rmcp::model::CallToolRequestParams::default();
                                req_params.name = name.to_string().into();
                                req_params.arguments = Some(args.clone());

                                match PaperPilotMcpServer::execute_call_tool(req_params) {
                                    Ok(response_result) => {
                                        let mut is_success = false;
                                        if let rmcp::model::CallToolResponse::Complete(result) = &response_result {
                                            if let Some(content) = result.content.first() {
                                                if let rmcp::model::ContentBlock::Text(text) = content {
                                                    is_success = text.text.contains("\"success\":true");
                                                }
                                            }
                                        }

                                        let response = serde_json::json!({
                                            "jsonrpc": "2.0",
                                            "id": req.get("id").unwrap_or(&serde_json::Value::Null),
                                            "result": { "success": is_success }
                                        });
                                        println!("{}", serde_json::to_string(&response).unwrap());
                                        return; // Exit successfully
                                    }
                                    Err(e) => {
                                        eprintln!("execute_call_tool failed: {:?}", e);
                                        return;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    eprintln!("Failed to parse JSON: {}", s);
                }
            } else {
                eprintln!("PAPERPILOT MCP NATIVE FALLBACK: {}", s);
            }
        }
    }

    let server_impl = PaperPilotMcpServer::new();
    let transport = rmcp::transport::stdio();

    if let Ok(running_service) = serve_server(server_impl, transport).await {
        if let Err(e) = running_service.waiting().await {
            eprintln!("PAPERPILOT MCP QUIT: {:?}", e);
        }
    }
}
