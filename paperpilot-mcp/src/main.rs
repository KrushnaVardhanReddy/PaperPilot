use rmcp::serve_server;

mod error;
mod schema;
mod server;

#[tokio::main]
async fn main() {
    let server_impl = server::PaperPilotMcpServer::new();
    let transport = rmcp::transport::stdio();
    serve_server(server_impl, transport).await.unwrap();
}
