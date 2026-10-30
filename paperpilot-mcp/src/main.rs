mod client_config;

use clap::Parser;
use paperpilot_mcp::server::PaperPilotMcpServer;
use rmcp::model::{CallToolRequestParams, CallToolResponse, ContentBlock};
use std::io::{self, BufRead};

#[derive(Parser)]
#[command(version, about = "Local PDF tools over MCP (stdio)")]
struct Args {
    /// Print client configuration to stdout, then exit without starting the server
    #[arg(long, value_enum, conflicts_with = "direct_tool_call")]
    client_config: Option<client_config::Client>,

    /// Executable to put in the configuration (defaults to this executable's path)
    #[arg(long, requires = "client_config")]
    server_command: Option<String>,

    /// Execute one tools/call request from stdin (diagnostic mode, not MCP transport)
    #[arg(long)]
    direct_tool_call: bool,
}

#[tokio::main]
async fn main() {
    // Spec 146: MCP Protocol Stdio Isolation in paperpilot-mcp
    // Under NO circumstances should application logs be written to stdout.
    // All tracing_subscriber writers in paperpilot-mcp MUST be directed explicitly to std::io::stderr.
    let log_level = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(&log_level));
    let log_format = std::env::var("LOG_FORMAT")
        .unwrap_or_else(|_| "text".to_string())
        .to_lowercase();

    match log_format.as_str() {
        "json" => {
            let _ = tracing_subscriber::fmt()
                .json()
                .with_writer(std::io::stderr)
                .with_env_filter(env_filter)
                .try_init();
        }
        "compact" => {
            let _ = tracing_subscriber::fmt()
                .compact()
                .with_writer(std::io::stderr)
                .with_env_filter(env_filter)
                .try_init();
        }
        "pretty" => {
            let _ = tracing_subscriber::fmt()
                .pretty()
                .with_writer(std::io::stderr)
                .with_env_filter(env_filter)
                .try_init();
        }
        _ => {
            let _ = tracing_subscriber::fmt()
                .with_writer(std::io::stderr)
                .with_env_filter(env_filter)
                .try_init();
        }
    }

    let args = Args::parse();

    if let Some(client) = args.client_config {
        let command = args.server_command.unwrap_or_else(|| {
            std::env::current_exe()
                .expect("Cannot locate paperpilot-mcp; supply --server-command")
                .into_os_string()
                .into_string()
                .expect("Executable path is not UTF-8; supply --server-command")
        });
        println!("{}", client_config::render(client, &command).unwrap());
        return;
    }

    // Check if we are running with explicit flag
    if args.direct_tool_call {
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        let mut first_line = String::new();

        if handle.read_line(&mut first_line).is_ok()
            && !first_line.trim().is_empty()
            && let Ok(val) = serde_json::from_str::<serde_json::Value>(&first_line)
            && val.get("method").and_then(|m| m.as_str()) == Some("tools/call")
        {
            let id = val.get("id").cloned().unwrap_or(serde_json::json!(1));
            if let Some(params) = val.get("params")
                && let Ok(tool_params) =
                    serde_json::from_value::<CallToolRequestParams>(params.clone())
            {
                match PaperPilotMcpServer::new(None).execute_call_tool(tool_params) {
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
                        paperpilot_pdf::operations::watch::shared_watcher_manager().shutdown();
                        std::process::exit(0);
                    }
                    Ok(_) => {
                        eprintln!("Tool call returned unexpected response type");
                        paperpilot_pdf::operations::watch::shared_watcher_manager().shutdown();
                        std::process::exit(1);
                    }
                    Err(e) => {
                        eprintln!("Tool call error: {:?}", e);
                        paperpilot_pdf::operations::watch::shared_watcher_manager().shutdown();
                        std::process::exit(1);
                    }
                }
            }
        }
        paperpilot_pdf::operations::watch::shared_watcher_manager().shutdown();
        std::process::exit(1);
    }

    // Let rmcp read stdin from the very first byte. Peeking here can consume
    // initialize (or part of it), breaking clients that send immediately.
    let server_impl = PaperPilotMcpServer::new(None);
    let transport = rmcp::transport::stdio();
    let running_service = rmcp::serve_server(server_impl, transport).await.unwrap();
    if let Err(e) = running_service.waiting().await {
        eprintln!("PAPERPILOT MCP QUIT: {:?}", e);
    }
    paperpilot_pdf::operations::watch::shared_watcher_manager().shutdown();
}
