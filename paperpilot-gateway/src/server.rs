use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use crate::{handlers, sse};

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::health,
        handlers::mcp_exec,
        handlers::merge,
        handlers::split,
        handlers::compress,
        handlers::extract_text,
        handlers::convert,
        handlers::watermark,
        handlers::info,
        handlers::compare,
        handlers::render_page,
        handlers::handle_tool
    ),
    components(
        schemas(
            handlers::HealthResponse,
            handlers::McpExecRequest,
            handlers::ApiResponse,
            handlers::MergeJsonRequest,
            handlers::SplitJsonRequest,
            handlers::CompressJsonRequest,
            handlers::ExtractTextJsonRequest,
            handlers::ConvertJsonRequest,
            handlers::WatermarkJsonRequest,
            handlers::InfoJsonRequest,
            handlers::CompareJsonRequest
        )
    ),
    tags(
        (name = "Health", description = "Gateway health check and tool status"),
        (name = "MCP", description = "Direct Model Context Protocol execution endpoint for 44+ tools"),
        (name = "PDF Operations", description = "High-performance PDF manipulation endpoints supporting JSON and multipart/form-data")
    ),
    info(
        title = "PaperPilot Gateway API & MCP Server",
        version = "0.1.0",
        description = "Unified REST and MCP API for PaperPilot: High performance local PDF processing engine and 44 intelligent PDF tools."
    )
)]
pub struct ApiDoc;

pub async fn build_app() -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/health", get(handlers::health))
        .route("/mcp/sse", get(sse::sse_handler))
        .route("/mcp/messages", post(sse::messages_handler))
        .route("/api/v1/pdf/tools/{tool_name}", post(handlers::handle_tool))
        .route("/api/v1/pdf/render-page", post(handlers::render_page))
        .route("/api/v1/pdf/merge", post(handlers::merge))
        .route("/api/v1/pdf/split", post(handlers::split))
        .route("/api/v1/pdf/compress", post(handlers::compress))
        .route("/api/v1/pdf/extract-text", post(handlers::extract_text))
        .route("/api/v1/pdf/convert", post(handlers::convert))
        .route("/api/v1/pdf/watermark", post(handlers::watermark))
        .route("/api/v1/pdf/info", post(handlers::info))
        .route("/api/v1/pdf/compare", post(handlers::compare))
        .route("/api/v1/pdf/mcp-exec", post(handlers::mcp_exec))
        .layer(cors)
}

pub async fn start(port: u16, bind: &str, shutdown_signal: impl std::future::Future<Output = ()> + Send + 'static) -> std::io::Result<()> {
    let app = build_app().await;
    let addr = format!("{}:{}", bind, port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("Starting gateway server on {}", addr);
    tracing::info!("Swagger UI accessible at http://{}/swagger-ui", addr);
    axum::serve(listener, app).with_graceful_shutdown(shutdown_signal).await
}
