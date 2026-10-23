use crate::{handlers, sse};
use axum::{
    Router,
    routing::{get, post},
};
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

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
        handlers::office_convert,
        handlers::excel_to_json,
        handlers::excel_to_csv,
        handlers::office_to_markdown,
        handlers::office_extract_text,
        handlers::rag_chunks,
        handlers::handle_tool,
        handlers::list_tools,
        handlers::pdf_validate_signatures,
        handlers::workflow_presets,
        handlers::workflow_validate,
        handlers::workflow_run_json
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
            handlers::CompareJsonRequest,
            handlers::ExcelToJsonRequest,
            handlers::ExcelToCsvRequest,
            handlers::OfficeConvertJsonRequest,
            handlers::RagChunksJsonRequest,
            handlers::WorkflowValidateRequest,
            handlers::WorkflowRunJsonRequest
        )
    ),
    tags(
        (name = "Health", description = "Gateway health check and tool status"),
        (name = "Tools", description = "Discovery and capability listing across all 52 tools"),
        (name = "MCP", description = "Direct Model Context Protocol execution endpoint for 50+ tools"),
        (name = "PDF Operations", description = "High-performance PDF manipulation endpoints supporting JSON and multipart/form-data"),
        (name = "Office Operations", description = "Office document and spreadsheet ingestion and modernizing endpoints"),
        (name = "Workflow", description = "Declarative multi-step document workflow execution and presets")
    ),
    info(
        title = "PaperPilot Gateway API & MCP Server",
        version = "0.1.0",
        description = "Unified REST and MCP API for PaperPilot: High performance local PDF processing engine and 50 intelligent document and spreadsheet tools."
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
        .route("/api/v1/tools", get(handlers::list_tools))
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
        .route("/api/v1/pdf/pii-scan", post(handlers::pii_scan))
        .route("/api/v1/pdf/sanitize", post(handlers::sanitize))
        .route("/api/v1/pdf/info", post(handlers::info))
        .route("/api/v1/pdf/compare", post(handlers::compare))
        .route("/api/v1/pdf/mcp-exec", post(handlers::mcp_exec))
        .route(
            "/api/v1/pdf/validate-signatures",
            post(handlers::pdf_validate_signatures),
        )
        .route("/api/v1/office/convert", post(handlers::office_convert))
        .route(
            "/api/v1/office/excel-to-json",
            post(handlers::excel_to_json),
        )
        .route("/api/v1/office/excel-to-csv", post(handlers::excel_to_csv))
        .route(
            "/api/v1/pdf/office-to-markdown",
            post(handlers::office_to_markdown),
        )
        .route(
            "/api/v1/pdf/office-extract-text",
            post(handlers::office_extract_text),
        )
        .route("/api/v1/pdf/rag-chunks", post(handlers::rag_chunks))
        .route(
            "/api/v1/search/workspace_search",
            post(handlers::workspace_search_handler),
        )
        .route("/api/v1/workflow/presets", get(handlers::workflow_presets))
        .route(
            "/api/v1/workflow/validate",
            post(handlers::workflow_validate),
        )
        .route(
            "/api/v1/workflow/run-json",
            post(handlers::workflow_run_json),
        )
        .route("/api/v1/workflow/run", post(handlers::workflow_run))
        .layer(cors)
}

pub async fn start(
    port: u16,
    bind: &str,
    shutdown_signal: impl std::future::Future<Output = ()> + Send + 'static,
) -> std::io::Result<()> {
    let app = build_app().await;
    let addr = format!("{}:{}", bind, port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("Starting gateway server on {}", addr);
    tracing::info!("Swagger UI accessible at http://{}/swagger-ui", addr);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal)
        .await
}
