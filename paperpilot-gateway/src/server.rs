use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use crate::{handlers, sse};

pub async fn build_app() -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/health", get(handlers::health))
        .route("/mcp/sse", get(sse::sse_handler))
        .route("/mcp/messages", post(sse::messages_handler))
        .route("/api/v1/pdf/tools/{tool_name}", post(handlers::handle_tool))
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

pub async fn start(port: u16, bind: &str) -> std::io::Result<()> {
    let app = build_app().await;
    let addr = format!("{}:{}", bind, port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("Starting gateway server on {}", addr);
    axum::serve(listener, app).await
}
