use rmcp::serve_server;
use rmcp::handler::server::ServerHandler;
use std::future::Future;
use rmcp::model::{CallToolResponse, CallToolResult};

#[derive(Clone)]
struct MyServer;

impl ServerHandler for MyServer {
    fn call_tool(
        &self,
        request: rmcp::model::CallToolRequestParams,
        context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> impl Future<Output = Result<CallToolResponse, rmcp::ErrorData>> + rmcp::service::MaybeSendFuture + '_ {
        let _ = context;
        async move {
            if request.name == "hello" {
                Ok(CallToolResponse::Complete(CallToolResult::success(vec![
                    rmcp::model::ContentBlock::text("hello world")
                ])))
            } else {
                Err(rmcp::ErrorData::invalid_params("Tool not found", None))
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let t = rmcp::transport::stdio();
    serve_server(MyServer, t).await.unwrap();
}
