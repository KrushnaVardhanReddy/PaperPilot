use paperpilot_mcp::server::PaperPilotMcpServer;
use rmcp::serve_server;

#[tokio::main]
async fn main() {
    let server_impl = PaperPilotMcpServer::new();
    let transport = rmcp::transport::stdio();
    let running_service = serve_server(server_impl, transport).await.unwrap();
    if let Err(e) = running_service.waiting().await {
        eprintln!("PAPERPILOT MCP QUIT: {:?}", e);
    }
}
