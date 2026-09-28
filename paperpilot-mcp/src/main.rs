use paperpilot_mcp::server::PaperPilotMcpServer;
use rmcp::serve_server;

#[tokio::main]
async fn main() {
    let server_impl = PaperPilotMcpServer::new();
    let transport = rmcp::transport::stdio();
    serve_server(server_impl, transport).await.unwrap();
}
