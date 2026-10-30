use crate::{handlers, sse};
use axum::{
    Router,
    routing::{delete, get, post, put},
};
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::health,
        handlers::tika,
        handlers::tika_text,
        handlers::tika_markdown,
        handlers::tika_rmeta,
        handlers::tika_rmeta_api,
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
        handlers::pii_scan,
        handlers::sanitize,
        handlers::rag_chunks,
        handlers::handle_tool,
        handlers::list_tools,
        handlers::pdf_to_epub,
        handlers::pdf_dedup_streams,
        handlers::pdf_posterize,
        handlers::pdf_split_by_outline,
        handlers::pdf_merge_alternate,
        handlers::pdf_comic_archive,
        handlers::pdf_split_scanned_book,
        handlers::pdf_change_permissions,
        handlers::pdf_timestamp,
        handlers::pdf_attachments,
        handlers::pdf_stitch_canvas,
        handlers::pdf_auto_rotate,
        handlers::pdf_overlay_underlay,
        handlers::pdf_stamp,
        handlers::pdf_remove_signatures,
        handlers::pdf_scale_pages,
        handlers::pdf_nup,
        handlers::pdf_booklet,
        handlers::pdf_validate_signatures,
        handlers::workflow_presets,
        handlers::workflow_validate,
        handlers::workflow_run_json,
        handlers::start_watcher,
        handlers::list_watchers,
        handlers::stop_watcher,
        handlers::run_recipe_once
    ),
    components(
        schemas(
        handlers::PdfInvertColorsJsonRequest,
            handlers::PdfSplitScannedBookJsonRequest,
            handlers::HealthResponse,
            handlers::McpExecRequest,
            handlers::ApiResponse,
            handlers::TikaMetadata,
            handlers::TikaMarkdownMetadata,
            handlers::TikaPdfBody,
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
            handlers::WorkflowRunJsonRequest,
            handlers::PdfValidateSignaturesJsonRequest,
            handlers::PdfPosterizeJsonRequest,
            handlers::PdfAttachmentsJsonRequest,
            handlers::PdfToEpubJsonRequest,
            handlers::StartWatcherRequest,
            handlers::StartWatcherResponse,
            handlers::WatcherListResponse,
            handlers::WatcherSummary,
            handlers::RunRecipeOnceResponse
        ,
            handlers::PdfEncryptJsonRequest,
            handlers::PdfDecryptJsonRequest,
            handlers::PdfSanitizeJsonRequest,
            handlers::PdfRedactJsonRequest,
            handlers::PdfReadFormJsonRequest,
            handlers::PdfFillFormJsonRequest,
            handlers::PdfFlattenJsonRequest,
            handlers::PdfStampJsonRequest,
            handlers::PdfRemoveSignaturesJsonRequest,
            handlers::PdfChangePermissionsJsonRequest,
            handlers::PdfCreateFormFieldJsonRequest,
            handlers::PdfTimestampJsonRequest,
            handlers::PdfBatesJsonRequest,
            handlers::PdfMergeJsonRequest,
            handlers::PdfMergeAlternateJsonRequest,
            handlers::PdfSplitJsonRequest,
            handlers::PdfSplitByOutlineJsonRequest,
            handlers::PdfRotateJsonRequest,
            handlers::PdfAutoRotateJsonRequest,
            handlers::PdfCropJsonRequest,
            handlers::PdfScalePagesJsonRequest,
            handlers::PdfDeletePagesJsonRequest,
            handlers::PdfReorderPagesJsonRequest,
            handlers::PdfExtractPagesJsonRequest,
            handlers::PdfNupJsonRequest,
            handlers::PdfBookletJsonRequest,
            handlers::PdfStitchCanvasJsonRequest,
            handlers::PdfToHtmlJsonRequest,
            handlers::PdfToMarkdownJsonRequest,
            handlers::PdfToJsonJsonRequest,
            handlers::JsonToPdfJsonRequest,
            handlers::PdfToPdfAJsonRequest,
            handlers::PdfComicArchiveJsonRequest,
            handlers::ExcelToJsonJsonRequest,
            handlers::PdfLinearizeJsonRequest,
            handlers::PdfRepairJsonRequest,
            handlers::PdfExtractTextJsonRequest,
            handlers::PdfOcrJsonRequest,
            handlers::PdfExtractTablesJsonRequest,
            handlers::DocumentPiiScanJsonRequest,
            handlers::PdfBookmarksJsonRequest,
            handlers::PdfHeaderFooterJsonRequest,
            handlers::PdfClassifyTypeJsonRequest,
            handlers::PdfCompareJsonRequest,
            handlers::PdfHashJsonRequest,
            handlers::PdfValidateJsonRequest,
            handlers::PdfRenderJsonRequest,
            handlers::PdfRagChunksJsonRequest,
            handlers::WorkspaceSearchJsonRequest,
            handlers::SemanticSearchJsonRequest)
    ),
    tags(
        (name = "Health", description = "Gateway health check and tool status"),
        (name = "Document Ingestion", description = "In-memory PDF extraction for Open-WebUI and Tika-compatible clients"),
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

use paperpilot_pdf::operations::watch::{
    WatchConfig, WatchSnapshot, WatcherManager, shared_watcher_manager,
};
use std::sync::Arc;
pub type WatcherRegistry = Arc<WatcherManager>;
pub struct WatcherRegistryAdapter(pub WatcherRegistry);
impl paperpilot_mcp::server::WatcherRegistryTrait for WatcherRegistryAdapter {
    fn add_named_watcher(&self, id: String, config: WatchConfig, name_template: Option<String>) -> Result<WatchSnapshot, String> {
        self.0.start_with_naming(id, config, name_template, Arc::new(|_| {})).map_err(|e|e.to_string())
    }
    fn add_watcher(&self, id: String, config: WatchConfig) -> Result<WatchSnapshot, String> {
        self.0
            .start(id, config, Arc::new(|_| {}))
            .map_err(|e| e.to_string())
    }
    fn remove_watcher(&self, id: &str) -> Result<(), String> {
        self.0.remove(id).map_err(|e| e.to_string())
    }
    fn list_watchers(&self) -> Vec<WatchSnapshot> {
        self.0.list()
    }
}

pub async fn build_app(registry: WatcherRegistry) -> Router {
    // Restrict CORS: Allow only trusted local origins (Tauri desktop client, Vite dev server, localhost)
    let cors = CorsLayer::new()
        .allow_origin(tower_http::cors::AllowOrigin::predicate(|origin, _| {
            let s = origin.to_str().unwrap_or("");
            s.starts_with("tauri://")
                || s.starts_with("http://localhost")
                || s.starts_with("http://127.0.0.1")
                || s.starts_with("https://localhost")
                || s.starts_with("https://127.0.0.1")
        }))
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/health", get(handlers::health))
        .route("/tika", put(handlers::tika))
        .route("/tika/text", put(handlers::tika_text))
        .route("/tika/json/md", put(handlers::tika_markdown))
        .route(
            "/rmeta/text",
            put(handlers::tika_rmeta).post(handlers::tika_rmeta),
        )
        .route(
            "/tika/rmeta/text",
            put(handlers::tika_rmeta).post(handlers::tika_rmeta),
        )
        .route(
            "/api/v1/tika/rmeta/text",
            post(handlers::tika_rmeta_api).put(handlers::tika_rmeta),
        )
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
        .route("/api/v1/pdf/to-epub", post(handlers::pdf_to_epub))
        .route(
            "/api/v1/pdf/dedup-streams",
            post(handlers::pdf_dedup_streams),
        )
        .route("/api/v1/pdf/posterize", post(handlers::pdf_posterize))
        .route(
            "/api/v1/pdf/merge-alternate",
            post(handlers::pdf_merge_alternate),
        )
        .route(
            "/api/v1/pdf/split-by-outline",
            post(handlers::pdf_split_by_outline),
        )
        .route(
            "/api/v1/pdf/comic-archive",
            post(handlers::pdf_comic_archive),
        )
        .route(
            "/api/v1/pdf/split-scanned-book",
            post(handlers::pdf_split_scanned_book),
        )
        .route(
            "/api/v1/pdf/change-permissions",
            post(handlers::pdf_change_permissions),
        )
        .route("/api/v1/pdf/timestamp", post(handlers::pdf_timestamp))
        .route("/api/v1/pdf/attachments", post(handlers::pdf_attachments))
        .route(
            "/api/v1/pdf/invert-colors",
            post(handlers::pdf_invert_colors),
        )
        .route(
            "/api/v1/pdf/stitch-canvas",
            post(handlers::pdf_stitch_canvas),
        )
        .route("/api/v1/pdf/auto-rotate", post(handlers::pdf_auto_rotate))
        .route(
            "/api/v1/pdf/overlay-underlay",
            post(handlers::pdf_overlay_underlay),
        )
        .route("/api/v1/pdf/stamp", post(handlers::pdf_stamp))
        .route(
            "/api/v1/pdf/remove-signatures",
            post(handlers::pdf_remove_signatures),
        )
        .route("/api/v1/pdf/scale-pages", post(handlers::pdf_scale_pages))
        .route("/api/v1/pdf/nup", post(handlers::pdf_nup))
        .route("/api/v1/pdf/booklet", post(handlers::pdf_booklet))
        .route("/api/v1/booklet", post(handlers::pdf_booklet))
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
        .route(
            "/v1/automation/watchers",
            post(handlers::start_watcher).get(handlers::list_watchers),
        )
        .route(
            "/v1/automation/watchers/{id}",
            delete(handlers::stop_watcher),
        )
        .route("/v1/automation/fill-batch", post(crate::batch_routes::fill))
        .route("/v1/automation/run", post(handlers::run_recipe_once))
        .route("/v1/automation/receipts", get(crate::receipt_routes::list))
        .route(
            "/v1/automation/receipts/{id}",
            get(crate::receipt_routes::get),
        )

        .route("/api/v1/pdf/pdf_encrypt", post(handlers::pdf_encrypt))
        .route("/api/v1/pdf/pdf_decrypt", post(handlers::pdf_decrypt))
        .route("/api/v1/pdf/pdf_sanitize", post(handlers::pdf_sanitize))
        .route("/api/v1/pdf/pdf_redact", post(handlers::pdf_redact))
        .route("/api/v1/pdf/pdf_read_form", post(handlers::pdf_read_form))
        .route("/api/v1/pdf/pdf_fill_form", post(handlers::pdf_fill_form))
        .route("/api/v1/pdf/pdf_flatten", post(handlers::pdf_flatten))
        .route("/api/v1/pdf/pdf_stamp", post(handlers::pdf_stamp))
        .route("/api/v1/pdf/pdf_validate_signatures", post(handlers::pdf_validate_signatures))
        .route("/api/v1/pdf/pdf_remove_signatures", post(handlers::pdf_remove_signatures))
        .route("/api/v1/pdf/pdf_change_permissions", post(handlers::pdf_change_permissions))
        .route("/api/v1/pdf/pdf_create_form_field", post(handlers::pdf_create_form_field))
        .route("/api/v1/pdf/pdf_timestamp", post(handlers::pdf_timestamp))
        .route("/api/v1/pdf/pdf_bates", post(handlers::pdf_bates))
        .route("/api/v1/pdf/pdf_merge", post(handlers::pdf_merge))
        .route("/api/v1/pdf/pdf_merge_alternate", post(handlers::pdf_merge_alternate))
        .route("/api/v1/pdf/pdf_split", post(handlers::pdf_split))
        .route("/api/v1/pdf/pdf_split_by_outline", post(handlers::pdf_split_by_outline))
        .route("/api/v1/pdf/pdf_split_scanned_book", post(handlers::pdf_split_scanned_book))
        .route("/api/v1/pdf/pdf_rotate", post(handlers::pdf_rotate))
        .route("/api/v1/pdf/pdf_auto_rotate", post(handlers::pdf_auto_rotate))
        .route("/api/v1/pdf/pdf_crop", post(handlers::pdf_crop))
        .route("/api/v1/pdf/pdf_scale_pages", post(handlers::pdf_scale_pages))
        .route("/api/v1/pdf/pdf_delete_pages", post(handlers::pdf_delete_pages))
        .route("/api/v1/pdf/pdf_reorder_pages", post(handlers::pdf_reorder_pages))
        .route("/api/v1/pdf/pdf_extract_pages", post(handlers::pdf_extract_pages))
        .route("/api/v1/pdf/pdf_nup", post(handlers::pdf_nup))
        .route("/api/v1/pdf/pdf_booklet", post(handlers::pdf_booklet))
        .route("/api/v1/pdf/pdf_posterize", post(handlers::pdf_posterize))
        .route("/api/v1/pdf/pdf_stitch_canvas", post(handlers::pdf_stitch_canvas))
        .route("/api/v1/pdf/pdf_to_html", post(handlers::pdf_to_html))
        .route("/api/v1/pdf/pdf_to_markdown", post(handlers::pdf_to_markdown))
        .route("/api/v1/pdf/pdf_to_json", post(handlers::pdf_to_json))
        .route("/api/v1/pdf/json_to_pdf", post(handlers::json_to_pdf))
        .route("/api/v1/pdf/pdf_to_epub", post(handlers::pdf_to_epub))
        .route("/api/v1/pdf/pdf_to_pdf_a", post(handlers::pdf_to_pdf_a))
        .route("/api/v1/pdf/pdf_comic_archive", post(handlers::pdf_comic_archive))
        .route("/api/v1/pdf/excel_to_json", post(handlers::excel_to_json))
        .route("/api/v1/pdf/excel_to_csv", post(handlers::excel_to_csv))
        .route("/api/v1/pdf/pdf_dedup_streams", post(handlers::pdf_dedup_streams))
        .route("/api/v1/pdf/pdf_invert_colors", post(handlers::pdf_invert_colors))
        .route("/api/v1/pdf/pdf_linearize", post(handlers::pdf_linearize))
        .route("/api/v1/pdf/pdf_repair", post(handlers::pdf_repair))
        .route("/api/v1/pdf/pdf_extract_text", post(handlers::pdf_extract_text))
        .route("/api/v1/pdf/pdf_ocr", post(handlers::pdf_ocr))
        .route("/api/v1/pdf/pdf_extract_tables", post(handlers::pdf_extract_tables))
        .route("/api/v1/pdf/document_pii_scan", post(handlers::document_pii_scan))
        .route("/api/v1/pdf/pdf_bookmarks", post(handlers::pdf_bookmarks))
        .route("/api/v1/pdf/pdf_header_footer", post(handlers::pdf_header_footer))
        .route("/api/v1/pdf/pdf_classify_type", post(handlers::pdf_classify_type))
        .route("/api/v1/pdf/pdf_compare", post(handlers::pdf_compare))
        .route("/api/v1/pdf/pdf_hash", post(handlers::pdf_hash))
        .route("/api/v1/pdf/pdf_validate", post(handlers::pdf_validate))
        .route("/api/v1/pdf/pdf_render", post(handlers::pdf_render))
        .route("/api/v1/pdf/pdf_rag_chunks", post(handlers::pdf_rag_chunks))

        .route("/api/v1/search/semantic_search", post(handlers::semantic_search))

        .route("/api/v1/pdf/pdf_watermark", post(handlers::watermark))
        .layer(axum::extract::DefaultBodyLimit::max(100 * 1024 * 1024)) // 100 MB max request body to prevent memory exhaustion DoS
        .layer(cors)
        .with_state(registry)
}

pub async fn start(
    port: u16,
    bind: &str,
    shutdown_signal: impl std::future::Future<Output = ()> + Send + 'static,
) -> std::io::Result<()> {
    let registry: WatcherRegistry = shared_watcher_manager();
    let app = build_app(registry).await;
    let addr = format!("{}:{}", bind, port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("Starting gateway server on {}", addr);
    tracing::info!("Swagger UI accessible at http://{}/swagger-ui", addr);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal)
        .await
}
