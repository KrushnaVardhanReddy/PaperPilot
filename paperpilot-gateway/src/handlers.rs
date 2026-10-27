#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
use axum::{
    body::Bytes,
    extract::Path,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Json},
};
use paperpilot_mcp::server::PaperPilotMcpServer;
use rmcp::model::{CallToolRequestParams, CallToolResponse};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::File;
use std::io::Write;
use tempfile::NamedTempFile;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct HealthResponse {
    #[schema(example = "ok")]
    pub status: String,
    #[schema(example = "0.1.0")]
    pub version: String,
    #[schema(example = 44)]
    pub tools_count: usize,
}

/// Standard API response envelope (Responses section shows expected schema contract).
#[derive(Serialize, Deserialize, ToSchema)]
pub struct ApiResponse {
    #[schema(example = true)]
    pub success: bool,
    pub result: Option<Value>,
    pub error: Option<String>,
}

/// Binary PDF upload schema for Swagger/OpenAPI.
#[derive(ToSchema)]
#[schema(value_type = String, format = Binary)]
pub struct TikaPdfBody(pub Vec<u8>);

/// Tika 3 / recursive-metadata response, consumed by Open-WebUI's Tika loader.
#[derive(Serialize, ToSchema)]
pub struct TikaMetadata {
    #[serde(rename = "X-TIKA:content")]
    pub content: String,
    #[serde(rename = "Content-Type")]
    pub content_type: String,
    #[serde(rename = "xmpTPg:NPages")]
    pub page_count: String,
}

/// Tika 4 renamed the content key to `tk:content`.
#[derive(Serialize, ToSchema)]
pub struct TikaMarkdownMetadata {
    #[serde(rename = "tk:content")]
    pub content: String,
    #[serde(rename = "Content-Type")]
    pub content_type: String,
    #[serde(rename = "xmpTPg:NPages")]
    pub page_count: String,
}

type TikaError = (StatusCode, Json<ApiResponse>);

fn tika_error(status: StatusCode, message: &str) -> TikaError {
    (
        status,
        Json(ApiResponse {
            success: false,
            result: None,
            error: Some(message.to_owned()),
        }),
    )
}

async fn extract_tika_metadata(
    headers: &HeaderMap,
    body: Bytes,
) -> Result<TikaMetadata, TikaError> {
    if body.is_empty() {
        return Err(tika_error(StatusCode::BAD_REQUEST, "The PDF body is empty"));
    }
    let media_type = headers
        .get(axum::http::header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("application/octet-stream")
        .split(';')
        .next()
        .unwrap_or_default()
        .trim();
    if !(media_type.eq_ignore_ascii_case("application/pdf")
        || media_type.eq_ignore_ascii_case("application/octet-stream"))
        || !body.starts_with(b"%PDF-")
    {
        return Err(tika_error(
            StatusCode::UNSUPPORTED_MEDIA_TYPE,
            "Send raw PDF bytes with Content-Type application/pdf or application/octet-stream",
        ));
    }

    // PDF parsing is CPU-bound; keep it off Tokio's async request workers.
    // Bytes extraction applies the router's existing 100 MiB body limit.
    let pages = tokio::task::spawn_blocking(move || {
        paperpilot_pdf::operations::extract_text::run_extract_text_bytes(&body, None)
    })
    .await
    .map_err(|error| {
        tracing::error!(%error, "Tika extraction worker failed");
        tika_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "PDF extraction worker failed",
        )
    })?
    .map_err(|error| {
        tracing::debug!(%error, "Tika PDF extraction failed");
        tika_error(
            StatusCode::UNPROCESSABLE_ENTITY,
            "Unable to extract this PDF",
        )
    })?;
    if pages.is_empty() {
        return Err(tika_error(
            StatusCode::UNPROCESSABLE_ENTITY,
            "The PDF has no pages",
        ));
    }
    Ok(TikaMetadata {
        page_count: pages.len().to_string(),
        content: pages.join("\n\n"),
        content_type: "application/pdf".to_owned(),
    })
}

/// Extract a raw PDF as UTF-8 text, or a metadata object with Accept: application/json.
#[utoipa::path(
    put, path = "/tika", tag = "Document Ingestion",
    request_body(content = TikaPdfBody, content_type = "application/pdf"),
    responses(
        (status = 200, description = "Page-ordered text with detected Markdown tables", body = String, content_type = "text/plain"),
        (status = 400, description = "Empty body"),
        (status = 413, description = "Body exceeds 100 MiB"),
        (status = 415, description = "Only raw PDF uploads are supported"),
        (status = 422, description = "PDF could not be parsed")
    )
)]
pub async fn tika(headers: HeaderMap, body: Bytes) -> impl IntoResponse {
    match extract_tika_metadata(&headers, body).await {
        Ok(metadata) => {
            let wants_json = headers
                .get(axum::http::header::ACCEPT)
                .and_then(|value| value.to_str().ok())
                .is_some_and(|accept| {
                    accept.split(',').any(|value| {
                        value
                            .split(';')
                            .next()
                            .unwrap_or_default()
                            .trim()
                            .eq_ignore_ascii_case("application/json")
                    })
                });
            if wants_json {
                Json(metadata).into_response()
            } else {
                metadata.content.into_response()
            }
        }
        Err(error) => error.into_response(),
    }
}

/// Open-WebUI's Tika 3 loader expects an object even without an Accept header.
#[utoipa::path(
    put, path = "/tika/text", tag = "Document Ingestion",
    request_body(content = TikaPdfBody, content_type = "application/pdf"),
    responses((status = 200, description = "Open-WebUI Tika 3 metadata", body = TikaMetadata))
)]
pub async fn tika_text(headers: HeaderMap, body: Bytes) -> Result<Json<TikaMetadata>, TikaError> {
    extract_tika_metadata(&headers, body).await.map(Json)
}

/// Open-WebUI's Tika 4 loader uses a different route and content key.
#[utoipa::path(
    put, path = "/tika/json/md", tag = "Document Ingestion",
    request_body(content = TikaPdfBody, content_type = "application/pdf"),
    responses((status = 200, description = "Open-WebUI Tika 4 metadata", body = TikaMarkdownMetadata))
)]
pub async fn tika_markdown(
    headers: HeaderMap,
    body: Bytes,
) -> Result<Json<TikaMarkdownMetadata>, TikaError> {
    let metadata = extract_tika_metadata(&headers, body).await?;
    Ok(Json(TikaMarkdownMetadata {
        content: metadata.content,
        content_type: metadata.content_type,
        page_count: metadata.page_count,
    }))
}

/// Tika's recursive metadata contract is an array; this adapter returns the root PDF only.
#[utoipa::path(
    put, path = "/rmeta/text", tag = "Document Ingestion",
    request_body(content = TikaPdfBody, content_type = "application/pdf"),
    responses((status = 200, description = "Root PDF metadata array; attachments are not parsed", body = Vec<TikaMetadata>))
)]
pub async fn tika_rmeta(
    headers: HeaderMap,
    body: Bytes,
) -> Result<Json<Vec<TikaMetadata>>, TikaError> {
    extract_tika_metadata(&headers, body)
        .await
        .map(|metadata| Json(vec![metadata]))
}

#[utoipa::path(
    post, path = "/api/v1/tika/rmeta/text", tag = "Document Ingestion",
    request_body(content = TikaPdfBody, content_type = "application/pdf"),
    responses((status = 200, description = "Tika-compatible root PDF metadata array", body = Vec<TikaMetadata>))
)]
pub async fn tika_rmeta_api(
    headers: HeaderMap,
    body: Bytes,
) -> Result<Json<Vec<TikaMetadata>>, TikaError> {
    tika_rmeta(headers, body).await
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct OfficeConvertJsonRequest {
    #[schema(example = "tests/e2e_fixtures/real/test.xlsx")]
    pub input: String,
    #[schema(example = "tests/e2e_fixtures/out/penta_e2e/doc.md")]
    pub output: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct RagChunksJsonRequest {
    #[schema(example = "tests/e2e_fixtures/real/single_page.pdf")]
    pub input: String,
    pub max_chunk_size: Option<usize>,
    pub overlap: Option<usize>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct McpExecRequest {
    #[schema(example = "pdf_metadata")]
    pub tool: String,
    #[schema(example = json!({"input": "/path/to/document.pdf"}))]
    pub arguments: Option<Value>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct MergeJsonRequest {
    #[schema(example = json!(["doc1.pdf", "doc2.pdf"]))]
    pub inputs: Vec<String>,
    #[schema(example = "merged.pdf")]
    pub output: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct SplitJsonRequest {
    #[schema(example = "input.pdf")]
    pub input: String,
    #[schema(example = "1-3,5")]
    pub pages: Option<String>,
    #[schema(example = "output_dir/")]
    pub output: Option<String>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct CompressJsonRequest {
    #[schema(example = "input.pdf")]
    pub input: String,
    #[schema(example = "output.pdf")]
    pub output: String,
    #[schema(example = "medium")]
    pub quality: Option<String>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct ExtractTextJsonRequest {
    #[schema(example = "input.pdf")]
    pub input: String,
    #[schema(example = "1")]
    pub page: Option<u32>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct ConvertJsonRequest {
    #[schema(example = "input.pdf")]
    pub input: String,
    #[schema(example = "output.docx")]
    pub output: String,
    #[schema(example = "docx")]
    pub format: Option<String>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct WatermarkJsonRequest {
    #[schema(example = "input.pdf")]
    pub input: String,
    #[schema(example = "watermarked.pdf")]
    pub output: String,
    #[schema(example = "CONFIDENTIAL")]
    pub text: Option<String>,
    #[schema(example = 45.0)]
    pub angle: Option<f32>,
    #[schema(example = 0.2)]
    pub opacity: Option<f32>,
    #[schema(example = "gray")]
    pub color: Option<String>,
    #[schema(example = 54.0)]
    pub font_size: Option<f32>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct InfoJsonRequest {
    #[schema(example = "input.pdf")]
    pub input: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct CompareJsonRequest {
    #[schema(example = "doc_v1.pdf")]
    pub file1: String,
    #[schema(example = "doc_v2.pdf")]
    pub file2: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct ExcelToJsonRequest {
    #[schema(example = "data.xlsx")]
    pub input: String,
    #[schema(example = "Sheet1")]
    pub sheet: Option<String>,
    #[schema(example = true)]
    pub has_headers: Option<bool>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct ExcelToCsvRequest {
    #[schema(example = "data.xlsx")]
    pub input: String,
    #[schema(example = "data.csv")]
    pub output: String,
    #[schema(example = "Sheet1")]
    pub sheet: Option<String>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct PdfAttachmentsJsonRequest {
    #[schema(example = "in.pdf")]
    pub input: String,
    #[schema(example = "list")]
    pub action: Option<String>,
    #[schema(example = "report.pdf")]
    pub output: Option<String>,
    #[schema(example = "invoice.pdf")]
    pub file_name: Option<String>,
}

/// Gateway health check
#[utoipa::path(
    get,
    path = "/health",
    tag = "Health",
    responses(
        (status = 200, description = "Service is healthy and ready", body = HealthResponse)
    )
)]
pub async fn health() -> impl IntoResponse {
    let tools_count = match PaperPilotMcpServer::execute_list_tools() {
        Ok(res) => res.tools.len(),
        Err(_) => 0,
    };
    let resp = HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        tools_count,
    };
    (StatusCode::OK, Json(resp))
}

/// List all available tools and operations across PaperPilot
#[utoipa::path(
    get,
    path = "/api/v1/tools",
    tag = "Tools",
    responses(
        (status = 200, description = "List of all registered tools and their schema definitions")
    )
)]
pub async fn list_tools() -> impl IntoResponse {
    match PaperPilotMcpServer::execute_list_tools() {
        Ok(res) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "success": true,
                "total": res.tools.len(),
                "tools": res.tools
            })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "success": false,
                "error": e.message
            })),
        ),
    }
}

/// Execute MCP tool directly by tool name with arbitrary JSON arguments [Input: JSON | Output: JSON]
#[utoipa::path(
    post,
    path = "/api/v1/pdf/mcp-exec",
    tag = "MCP",
    request_body = McpExecRequest,
    responses(
        (status = 200, description = "Tool executed successfully", body = ApiResponse),
        (status = 400, description = "Invalid request arguments"),
        (status = 500, description = "Execution error")
    )
)]
pub async fn mcp_exec(Json(payload): Json<McpExecRequest>) -> impl IntoResponse {
    let mut request = CallToolRequestParams::default();
    request.name = payload.tool.into();
    if let Some(Value::Object(args)) = payload.arguments {
        request.arguments = Some(args);
    } else if payload.arguments.is_some() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "arguments must be an object"})),
        );
    }

    match PaperPilotMcpServer::execute_call_tool(request) {
        Ok(CallToolResponse::Complete(result)) => (
            StatusCode::OK,
            Json(serde_json::json!({"success": true, "result": result})),
        ),
        Ok(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"success": false, "error": "Unknown response format"})),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"success": false, "error": e.message})),
        ),
    }
}

/// Execute any MCP tool by URL path param (/api/v1/pdf/tools/{tool_name}) via multipart or JSON [Input: Multipart/JSON | Output: File/JSON]
#[utoipa::path(
    post,
    path = "/api/v1/pdf/tools/{tool_name}",
    tag = "MCP",
    params(
        ("tool_name" = String, Path, description = "Name of the tool (with or without 'pdf_' prefix)")
    ),
    responses(
        (status = 200, description = "Tool executed successfully", body = ApiResponse),
        (status = 400, description = "Invalid multipart or payload"),
        (status = 500, description = "Execution error")
    )
)]
pub async fn handle_tool(
    Path(tool_name): Path<String>,
    req: axum::extract::Request,
) -> impl IntoResponse {
    let content_type = req
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    let mut args = serde_json::Map::new();
    let mut temp_files = vec![];

    if content_type.starts_with("application/json") {
        let body_bytes = axum::body::to_bytes(req.into_body(), usize::MAX)
            .await
            .unwrap_or_default();
        if let Ok(Value::Object(map)) = serde_json::from_slice(&body_bytes) {
            args = map;

            // Check if URL is in JSON payload
            if let Some(Value::String(url_str)) = args.get("url")
                && (url_str.starts_with("http://") || url_str.starts_with("https://"))
            {
                if let Ok(path) = paperpilot_core::url_fetch::resolve_url_to_temp_file(url_str) {
                    args.insert(
                        "input".to_string(),
                        Value::String(path.to_string_lossy().to_string()),
                    );
                } else {
                    return (
                        axum::http::StatusCode::BAD_REQUEST,
                        Json(serde_json::json!({"success": false, "error": "Failed to fetch URL"})),
                    )
                        .into_response();
                }
            }
        }
    } else {
        use axum::extract::FromRequest;
        let mut multipart =
            match axum::extract::Multipart::from_request(req, &()).await {
                Ok(m) => m,
                Err(_) => return (
                    axum::http::StatusCode::BAD_REQUEST,
                    Json(
                        serde_json::json!({"success": false, "error": "Invalid multipart request"}),
                    ),
                )
                    .into_response(),
            };
        while let Ok(Some(field)) = multipart.next_field().await {
            let name = field.name().unwrap_or("").to_string();
            if name.is_empty() {
                continue;
            }

            let file_name = field.file_name().map(|s| s.to_string());

            let data = match field.bytes().await {
                Ok(d) => d,
                Err(_) => continue,
            };

            if file_name.is_some()
                || name == "file"
                || name == "input"
                || name == "file1"
                || name == "file2"
                || name == "file_a"
                || name == "file_b"
                || name == "input_a"
                || name == "input_b"
            {
                let temp_file = NamedTempFile::new().unwrap();
                let path = temp_file.path().to_owned();
                let mut file = File::create(&path).unwrap();
                file.write_all(&data).unwrap();

                if name == "input" || name == "file" {
                    args.insert(
                        "input".to_string(),
                        Value::String(path.to_string_lossy().to_string()),
                    );
                } else if args.contains_key(&name) {
                    if let Value::Array(arr) = args.get_mut(&name).unwrap() {
                        arr.push(Value::String(path.to_string_lossy().to_string()));
                    }
                } else {
                    args.insert(
                        name.clone(),
                        Value::String(path.to_string_lossy().to_string()),
                    );
                }
                temp_files.push(temp_file);
            } else {
                let s = String::from_utf8_lossy(&data).to_string();
                if name == "url" && (s.starts_with("http://") || s.starts_with("https://")) {
                    if let Ok(path) = paperpilot_core::url_fetch::resolve_url_to_temp_file(&s) {
                        args.insert(
                            "input".to_string(),
                            Value::String(path.to_string_lossy().to_string()),
                        );
                    } else {
                        return (
                            axum::http::StatusCode::BAD_REQUEST,
                            Json(serde_json::json!({"success": false, "error": "Failed to fetch URL"})),
                        ).into_response();
                    }
                } else if let Ok(val) = serde_json::from_str::<Value>(&s) {
                    args.insert(name, val);
                } else {
                    args.insert(name, Value::String(s));
                }
            }
        }
    }

    let mut request = CallToolRequestParams::default();

    let mut resolved_name = if tool_name.starts_with("pdf_")
        || tool_name == "excel_to_json"
        || tool_name == "excel_to_csv"
        || tool_name == "json_to_pdf"
        || tool_name == "workspace_search"
        || tool_name.starts_with("office_")
        || tool_name == "document_pii_scan"
    {
        tool_name.clone()
    } else {
        format!("pdf_{}", tool_name)
    };

    if resolved_name == "pdf_rotate" {
        if !args.contains_key("pages") {
            args.insert(
                "pages".to_string(),
                serde_json::Value::String("all".to_string()),
            );
        }
        if let Some(angle) = args.get("degrees").or_else(|| args.get("angle")) {
            let angle_val = angle.clone();
            args.insert("angle".to_string(), angle_val);
        }
    } else if resolved_name == "pdf_crop" {
        if !args.contains_key("box") {
            let x = args.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let y = args.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let w = args.get("width").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let h = args.get("height").and_then(|v| v.as_f64()).unwrap_or(0.0);
            args.insert(
                "box".to_string(),
                serde_json::Value::String(format!("{},{},{},{}", x, y, w, h)),
            );
        }
        if !args.contains_key("pages") {
            args.insert(
                "pages".to_string(),
                serde_json::Value::String("all".to_string()),
            );
        }
    } else if resolved_name == "pdf_split" {
        let out_opt = args
            .get("output")
            .or_else(|| args.get("output_dir"))
            .cloned();
        if let Some(out_val) = out_opt {
            args.insert("output_dir".to_string(), out_val.clone());
            args.insert("output".to_string(), out_val.clone());
            if let Some(path) = out_val.as_str() {
                let path_obj = std::path::Path::new(path);
                if !path_obj.exists() {
                    let _ = std::fs::create_dir_all(path_obj);
                }
            }
        }
    } else if resolved_name == "pdf_burst" {
        if let Some(out_dir) = args.get("output_dir")
            && let Some(path) = out_dir.as_str()
        {
            let path_obj = std::path::Path::new(path);
            if !path_obj.exists() {
                let _ = std::fs::create_dir_all(path_obj);
            }
        }
    } else if resolved_name == "pdf_reorder_pages" {
        if let (Some(order_val), Some(input_val)) = (args.get("order"), args.get("input"))
            && let (Some(order_str), Some(input_path)) = (order_val.as_str(), input_val.as_str())
        {
            let order_parts: Vec<usize> = order_str
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            if let Ok(doc) = lopdf::Document::load(input_path) {
                let page_count = doc.get_pages().len();
                if order_parts.len() < page_count {
                    let mut full_order = order_parts.clone();
                    let already_listed: std::collections::HashSet<usize> =
                        order_parts.into_iter().collect();
                    for page in 1..=page_count {
                        if !already_listed.contains(&page) {
                            full_order.push(page);
                        }
                    }
                    let new_order_str = full_order
                        .into_iter()
                        .map(|p| p.to_string())
                        .collect::<Vec<_>>()
                        .join(",");
                    args.insert(
                        "order".to_string(),
                        serde_json::Value::String(new_order_str),
                    );
                }
            }
        }
    } else if resolved_name == "pdf_convert" {
        if let Some(format_val) = args.get("format") {
            if let Some(format_str) = format_val.as_str() {
                resolved_name = match format_str {
                    "docx" => "pdf_to_docx".to_string(),
                    "xlsx" => "pdf_to_xlsx".to_string(),
                    "pptx" => "pdf_to_pptx".to_string(),
                    "html" => "pdf_convert_html".to_string(),
                    "markdown" => "pdf_convert_markdown".to_string(),
                    "excel" | "csv" => "pdf_convert_excel".to_string(),
                    "pdf" => {
                        let has_json_ext = args
                            .get("input")
                            .and_then(|v| v.as_str())
                            .is_some_and(|s| s.ends_with(".json"));
                        let has_json_content = args.get("json_content").is_some();
                        if has_json_ext || has_json_content {
                            "json_to_pdf".to_string()
                        } else {
                            "pdf_convert_html".to_string()
                        }
                    }
                    _ => format!("pdf_to_{}", format_str),
                };
            }
        } else {
            resolved_name = "pdf_to_docx".to_string(); // default
        }
    } else if resolved_name == "pdf_render" && !args.contains_key("page") {
        args.insert("page".to_string(), serde_json::Value::Number(1.into()));
    }

    request.name = resolved_name.clone().into();
    request.arguments = Some(args.clone());

    match PaperPilotMcpServer::execute_call_tool(request) {
        Ok(CallToolResponse::Complete(result)) => {
            if resolved_name == "pdf_extract_text"
                && let Some(output_val) = args.get("output")
                && let Some(output_path) = output_val.as_str()
                && let Ok(metadata) = std::fs::metadata(output_path)
                && metadata.len() == 0
            {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({"success": false, "error": "Extracted text is empty"})),
                )
                    .into_response();
            }
            (
                StatusCode::OK,
                Json(serde_json::json!({"success": true, "result": result})),
            )
                .into_response()
        }
        Ok(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"success": false, "error": "Unknown response type"})),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"success": false, "error": e.message})),
        )
            .into_response(),
    }
}

/// Merge multiple PDF documents into a single PDF [Input: 2+ .pdfs | Output: .pdf]
#[utoipa::path(
    post,
    path = "/api/v1/pdf/merge",
    tag = "PDF Operations",
    request_body(content = MergeJsonRequest, description = "JSON configuration or multipart form"),
    responses(
        (status = 200, description = "Expected schema contract for successful merge", body = ApiResponse),
        (status = 400, description = "Expected schema contract for invalid input arguments", body = ApiResponse),
        (status = 500, description = "Expected schema contract for engine execution errors", body = ApiResponse)
    )
)]
pub async fn merge(req: axum::extract::Request) -> impl IntoResponse {
    handle_tool(Path("merge".to_string()), req).await
}

/// Split a PDF document into separate pages or ranges [Input: .pdf | Output: multiple .pdfs]
#[utoipa::path(
    post,
    path = "/api/v1/pdf/split",
    tag = "PDF Operations",
    request_body(content = SplitJsonRequest, description = "JSON configuration or multipart form"),
    responses(
        (status = 200, description = "Split successfully", body = ApiResponse)
    )
)]
pub async fn split(req: axum::extract::Request) -> impl IntoResponse {
    handle_tool(Path("split".to_string()), req).await
}

/// Compress a PDF document to reduce file size [Input: .pdf | Output: .pdf]
#[utoipa::path(
    post,
    path = "/api/v1/pdf/compress",
    tag = "PDF Operations",
    request_body(content = CompressJsonRequest, description = "JSON configuration or multipart form"),
    responses(
        (status = 200, description = "Compressed successfully", body = ApiResponse)
    )
)]
pub async fn compress(req: axum::extract::Request) -> impl IntoResponse {
    handle_tool(Path("pdf_compress".to_string()), req).await
}

/// Extract clean text from a PDF document [Input: .pdf | Output: .txt]
#[utoipa::path(
    post,
    path = "/api/v1/pdf/extract-text",
    tag = "PDF Operations",
    request_body(content = ExtractTextJsonRequest, description = "JSON configuration or multipart form"),
    responses(
        (status = 200, description = "Text extracted successfully", body = ApiResponse)
    )
)]
pub async fn extract_text(req: axum::extract::Request) -> impl IntoResponse {
    handle_tool(Path("extract_text".to_string()), req).await
}

/// Convert a PDF to/from another format (docx, images, html, xlsx, etc.) [Input: .pdf | Output: .docx/.png/.html/.xlsx]
#[utoipa::path(
    post,
    path = "/api/v1/pdf/convert",
    tag = "PDF Operations",
    request_body(content = ConvertJsonRequest, description = "JSON configuration or multipart form"),
    responses(
        (status = 200, description = "Converted successfully", body = ApiResponse)
    )
)]
pub async fn convert(req: axum::extract::Request) -> impl IntoResponse {
    // Add logging and surface errors dynamically
    tracing::info!("Received convert request");
    let res = handle_tool(Path("convert".to_string()), req)
        .await
        .into_response();
    if res.status().is_server_error() || res.status().is_client_error() {
        tracing::error!("Convert failed with status: {}", res.status());
    }
    res
}

#[utoipa::path(
    post,
    path = "/api/v1/office/convert",
    tag = "Office Operations",
    responses(
        (status = 200, description = "Converted successfully", body = ApiResponse)
    )
)]
pub async fn office_convert(req: axum::extract::Request) -> impl IntoResponse {
    tracing::info!("Received office convert request");
    handle_tool(Path("office_modernize_legacy".to_string()), req)
        .await
        .into_response()
}

/// Convert an Excel sheet into structured JSON format [Input: .xlsx, .xls, .csv | Output: .json]
#[utoipa::path(
    post,
    path = "/api/v1/office/excel-to-json",
    tag = "Office Operations",
    request_body(content = ExcelToJsonRequest, description = "JSON configuration or multipart form"),
    responses(
        (status = 200, description = "Converted Excel to JSON successfully", body = ApiResponse),
        (status = 400, description = "Invalid request arguments"),
        (status = 500, description = "Execution error")
    )
)]
pub async fn excel_to_json(req: axum::extract::Request) -> impl IntoResponse {
    tracing::info!("Received excel_to_json request");
    handle_tool(Path("excel_to_json".to_string()), req)
        .await
        .into_response()
}

/// Convert an Excel sheet into CSV format [Input: .xlsx, .xls | Output: .csv]
#[utoipa::path(
    post,
    path = "/api/v1/office/excel-to-csv",
    tag = "Office Operations",
    request_body(content = ExcelToCsvRequest, description = "JSON configuration or multipart form"),
    responses(
        (status = 200, description = "Converted Excel to CSV successfully", body = ApiResponse),
        (status = 400, description = "Invalid request arguments"),
        (status = 500, description = "Execution error")
    )
)]
pub async fn excel_to_csv(req: axum::extract::Request) -> impl IntoResponse {
    tracing::info!("Received excel_to_csv request");
    handle_tool(Path("excel_to_csv".to_string()), req)
        .await
        .into_response()
}

/// Apply text or image watermarks to a PDF document [Input: .pdf | Output: .pdf]
#[utoipa::path(
    post,
    path = "/api/v1/pdf/watermark",
    tag = "PDF Operations",
    request_body(content = WatermarkJsonRequest, description = "JSON configuration or multipart form"),
    responses(
        (status = 200, description = "Watermarked successfully", body = ApiResponse)
    )
)]
pub async fn watermark(req: axum::extract::Request) -> impl IntoResponse {
    handle_tool(Path("watermark".to_string()), req).await
}

/// Deeply sanitize a PDF document by removing metadata, scripts, attachments, and thumbnails [Input: .pdf | Output: .pdf]
#[utoipa::path(
    post,
    path = "/api/v1/pdf/sanitize",
    tag = "PDF Operations",
    responses(
        (status = 200, description = "Sanitized successfully", body = ApiResponse),
        (status = 400, description = "Invalid request arguments"),
        (status = 500, description = "Execution error")
    )
)]
pub async fn sanitize(req: axum::extract::Request) -> impl IntoResponse {
    handle_tool(Path("sanitize".to_string()), req).await
}

/// Scan document for PII and sensitive data [Input: .pdf | Output: JSON report]
#[utoipa::path(
    post,
    path = "/api/v1/pdf/pii-scan",
    tag = "PDF Operations",
    responses(
        (status = 200, description = "PII scan completed", body = ApiResponse),
        (status = 400, description = "Invalid request arguments"),
        (status = 500, description = "Execution error")
    )
)]
pub async fn pii_scan(req: axum::extract::Request) -> impl IntoResponse {
    handle_tool(Path("document_pii_scan".to_string()), req).await
}

/// Retrieve metadata and page information for a PDF document [Input: .pdf | Output: .json]
#[utoipa::path(
    post,
    path = "/api/v1/pdf/info",
    tag = "PDF Operations",
    request_body(content = InfoJsonRequest, description = "JSON configuration or multipart form"),
    responses(
        (status = 200, description = "Info extracted successfully", body = ApiResponse)
    )
)]
pub async fn info(req: axum::extract::Request) -> impl IntoResponse {
    handle_tool(Path("metadata".to_string()), req).await
}

/// Compare two PDF documents for visual and textual differences [Input: 2 .pdfs | Output: .pdf]
#[utoipa::path(
    post,
    path = "/api/v1/pdf/compare",
    tag = "PDF Operations",
    request_body(content = CompareJsonRequest, description = "JSON configuration or multipart form"),
    responses(
        (status = 200, description = "Comparison completed", body = ApiResponse)
    )
)]
pub async fn compare(req: axum::extract::Request) -> impl IntoResponse {
    handle_tool(Path("pdf_compare".to_string()), req).await
}

/// Render a single PDF page directly to a PNG image stream [Input: .pdf | Output: .png]
#[utoipa::path(
    post,
    path = "/api/v1/pdf/render-page",
    tag = "PDF Operations",
    responses(
        (status = 200, description = "Rendered PNG image binary", content_type = "image/png"),
        (status = 400, description = "Invalid request"),
        (status = 500, description = "Render failure")
    )
)]
pub async fn render_page(req: axum::extract::Request) -> axum::response::Response {
    use axum::extract::FromRequest;
    use axum::response::IntoResponse;
    let mut args = serde_json::Map::new();
    let mut temp_files = vec![];

    let content_type = req
        .headers()
        .get(axum::http::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    let is_json = content_type.starts_with("application/json");

    if is_json {
        let body_bytes = match axum::body::to_bytes(req.into_body(), usize::MAX).await {
            Ok(b) => b,
            Err(_) => return (axum::http::StatusCode::BAD_REQUEST, "Invalid body").into_response(),
        };
        if let Ok(json_val) = serde_json::from_slice::<serde_json::Value>(&body_bytes) {
            if let Some(obj) = json_val.as_object() {
                args = obj.clone();
            } else {
                return (axum::http::StatusCode::BAD_REQUEST, "Invalid JSON object")
                    .into_response();
            }
        } else {
            return (axum::http::StatusCode::BAD_REQUEST, "Invalid JSON").into_response();
        }
    } else {
        let mut multipart = match axum::extract::Multipart::from_request(req, &()).await {
            Ok(m) => m,
            Err(_) => {
                return (axum::http::StatusCode::BAD_REQUEST, "Invalid multipart").into_response();
            }
        };

        while let Ok(Some(field)) = multipart.next_field().await {
            let name = field.name().unwrap_or("").to_string();
            if name.is_empty() {
                continue;
            }

            let file_name = field.file_name().map(|s| s.to_string());
            let data = match field.bytes().await {
                Ok(d) => d,
                Err(_) => continue,
            };

            if file_name.is_some() || name == "file" || name == "input" {
                let mut temp_file = match tempfile::NamedTempFile::new() {
                    Ok(tf) => tf,
                    Err(_) => {
                        return (
                            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                            "Failed to create temp file",
                        )
                            .into_response();
                    }
                };
                let path = temp_file.path().to_owned();
                use std::io::Write;
                if temp_file.write_all(&data).is_err() {
                    return (
                        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to write data",
                    )
                        .into_response();
                };
                args.insert(
                    "input".to_string(),
                    serde_json::Value::String(path.to_string_lossy().to_string()),
                );
                temp_files.push(temp_file);
            } else {
                let s = String::from_utf8_lossy(&data).to_string();
                if let Ok(val) = serde_json::from_str::<serde_json::Value>(&s) {
                    args.insert(name, val);
                } else {
                    args.insert(name, serde_json::Value::String(s));
                }
            }
        }
    }

    let caller_provided_output = args
        .get("output")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let output_path = if let Some(out) = &caller_provided_output {
        std::path::PathBuf::from(out)
    } else {
        std::env::temp_dir().join(format!("{}.png", uuid::Uuid::new_v4()))
    };

    args.insert(
        "output".to_string(),
        serde_json::Value::String(output_path.to_string_lossy().to_string()),
    );

    let mut request = rmcp::model::CallToolRequestParams::default();
    request.name = "pdf_render".into();
    request.arguments = Some(args);

    match paperpilot_mcp::server::PaperPilotMcpServer::execute_call_tool(request) {
        Ok(res) => {
            if caller_provided_output.is_some() {
                // If output was provided, return standard JSON response
                let content_blocks = match res {
                    rmcp::model::CallToolResponse::Complete(r) => r.content,
                    _ => vec![],
                };
                let res_json = content_blocks.first().and_then(|b| {
                    if let rmcp::model::ContentBlock::Text(text_block) = b {
                        serde_json::from_str::<serde_json::Value>(&text_block.text).ok()
                    } else {
                        None
                    }
                }).unwrap_or(serde_json::json!({"success": true, "output_path": output_path.to_string_lossy().to_string()}));

                (axum::http::StatusCode::OK, axum::Json(res_json)).into_response()
            } else {
                // If output was not provided, stream the image bytes
                if let Ok(bytes) = std::fs::read(&output_path) {
                    let _ = std::fs::remove_file(&output_path);
                    let mut res = axum::response::Response::new(axum::body::Body::from(bytes));
                    res.headers_mut().insert(
                        axum::http::header::CONTENT_TYPE,
                        axum::http::HeaderValue::from_static("image/png"),
                    );
                    res.into_response()
                } else {
                    (
                        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                        "File generated but could not be read",
                    )
                        .into_response()
                }
            }
        }
        Err(e) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            e.message.clone(),
        )
            .into_response(),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/pdf/office-to-markdown",
    request_body = OfficeConvertJsonRequest,
    responses((status = 200, body = ApiResponse))
)]
pub async fn office_to_markdown(
    Json(payload): Json<OfficeConvertJsonRequest>,
) -> impl IntoResponse {
    let input = std::path::PathBuf::from(&payload.input);
    let output = std::path::PathBuf::from(&payload.output);
    match paperpilot_pdf::operations::office::OfficeToMarkdownOperation::execute(&input, &output) {
        Ok(_) => (
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                result: Some(serde_json::json!({"output": payload.output})),
                error: None,
            }),
        )
            .into_response(),
        Err(e) => {
            let err_str = e.to_string();
            (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse {
                    success: false,
                    result: None,
                    error: Some(err_str),
                }),
            )
                .into_response()
        }
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/pdf/office-extract-text",
    request_body = OfficeConvertJsonRequest,
    responses((status = 200, body = ApiResponse))
)]
pub async fn office_extract_text(
    Json(payload): Json<OfficeConvertJsonRequest>,
) -> impl IntoResponse {
    let input = std::path::PathBuf::from(&payload.input);
    let output = std::path::PathBuf::from(&payload.output);
    match paperpilot_pdf::operations::office::OfficeExtractTextOperation::execute(&input, &output) {
        Ok(_) => (
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                result: Some(serde_json::json!({"output": payload.output})),
                error: None,
            }),
        )
            .into_response(),
        Err(e) => {
            let err_str = e.to_string();
            (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse {
                    success: false,
                    result: None,
                    error: Some(err_str),
                }),
            )
                .into_response()
        }
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/pdf/rag-chunks",
    request_body = RagChunksJsonRequest,
    responses((status = 200, body = ApiResponse))
)]
pub async fn rag_chunks(Json(payload): Json<RagChunksJsonRequest>) -> impl IntoResponse {
    use paperpilot_core::traits::PdfOperation;
    use paperpilot_pdf::document::LopdfDocument;

    let _max_chunk_size = payload.max_chunk_size.unwrap_or(1000);
    let _overlap = payload.overlap.unwrap_or(200);

    let input_path = std::path::PathBuf::from(&payload.input);

    let mut doc = match LopdfDocument::load(&input_path) {
        Ok(d) => d,
        Err(e) => {
            let err_str = e.to_string();
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse {
                    success: false,
                    result: None,
                    error: Some(err_str),
                }),
            )
                .into_response();
        }
    };

    let op = paperpilot_pdf::operations::rag_chunks::PdfRagChunksOperation::new();
    match op.execute(&mut doc) {
        Ok(_) => {
            let chunks = op.chunks.lock().unwrap().clone();
            let count = chunks.len();
            (
                StatusCode::OK,
                Json(ApiResponse {
                    success: true,
                    result: Some(serde_json::json!({"chunks": chunks, "count": count})),
                    error: None,
                }),
            )
                .into_response()
        }
        Err(e) => {
            let err_str = e.to_string();
            (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse {
                    success: false,
                    result: None,
                    error: Some(err_str),
                }),
            )
                .into_response()
        }
    }
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct WorkflowValidateRequest {
    #[schema(value_type = Object)]
    pub recipe: Option<serde_json::Value>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct WorkflowRunJsonRequest {
    pub input: String,
    pub output: String,
    pub preset: Option<String>,
    #[schema(value_type = Object)]
    pub recipe: Option<serde_json::Value>,
}

/// Discover all built-in declarative workflow presets (Legal Discovery, Clean Scan, Redact & Protect)
#[utoipa::path(
    get,
    path = "/api/v1/workflow/presets",
    tag = "Workflow",
    responses(
        (status = 200, description = "List of workflow presets")
    )
)]
pub async fn workflow_presets() -> impl IntoResponse {
    let presets = paperpilot_core::workflow::get_default_presets();
    (StatusCode::OK, Json(presets))
}

/// Validate a workflow recipe JSON structure before execution
#[utoipa::path(
    post,
    path = "/api/v1/workflow/validate",
    tag = "Workflow",
    request_body = WorkflowValidateRequest,
    responses(
        (status = 200, description = "Recipe validated successfully"),
        (status = 400, description = "Invalid recipe schema")
    )
)]
pub async fn workflow_validate(Json(payload): Json<WorkflowValidateRequest>) -> impl IntoResponse {
    if let Some(recipe_val) = payload.recipe {
        if let Ok(wf) =
            serde_json::from_value::<paperpilot_core::workflow::SavedWorkflow>(recipe_val)
        {
            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "valid": true,
                    "steps_count": wf.steps.len()
                })),
            )
        } else {
            (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "valid": false,
                    "error": "Invalid recipe schema"
                })),
            )
        }
    } else {
        (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "valid": false,
                "error": "Missing recipe payload"
            })),
        )
    }
}

/// Execute a multi-step workflow recipe via JSON referencing file paths
#[utoipa::path(
    post,
    path = "/api/v1/workflow/run-json",
    tag = "Workflow",
    request_body = WorkflowRunJsonRequest,
    responses(
        (status = 200, description = "Workflow executed successfully"),
        (status = 400, description = "Invalid preset or recipe"),
        (status = 500, description = "Execution error")
    )
)]
pub async fn workflow_run_json(Json(payload): Json<WorkflowRunJsonRequest>) -> impl IntoResponse {
    let workflow = if let Some(preset_name) = &payload.preset {
        match paperpilot_core::workflow::get_preset_by_id(preset_name) {
            Some(w) => w,
            None => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({"status": "error", "error": "Preset not found"})),
                )
                    .into_response();
            }
        }
    } else if let Some(recipe_val) = payload.recipe {
        match serde_json::from_value::<paperpilot_core::workflow::SavedWorkflow>(recipe_val) {
            Ok(w) => w,
            Err(e) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({"status": "error", "error": format!("Invalid recipe JSON: {}", e)})),
                )
                    .into_response();
            }
        }
    } else {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"status": "error", "error": "Must provide preset or recipe"})),
        )
            .into_response();
    };

    let input_bytes = match std::fs::read(&payload.input) {
        Ok(b) => b,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"status": "error", "error": format!("Read input failed: {}", e)})),
            )
                .into_response();
        }
    };

    let (out_bytes, report) =
        match paperpilot_pdf::operations::workflow::run_workflow_bytes(&input_bytes, &workflow) {
            Ok(res) => res,
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"status": "error", "error": e.to_string()})),
                )
                    .into_response();
            }
        };

    if let Some(parent) = std::path::Path::new(&payload.output).parent()
        && !parent.as_os_str().is_empty()
    {
        let _ = std::fs::create_dir_all(parent);
    }

    if let Err(e) = std::fs::write(&payload.output, out_bytes) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"status": "error", "error": format!("Write output failed: {}", e)})),
        )
            .into_response();
    }

    (
        StatusCode::OK,
        Json(serde_json::json!({
            "status": "success",
            "report": report
        })),
    )
        .into_response()
}

pub async fn workflow_run(req: axum::extract::Request) -> impl IntoResponse {
    use axum::extract::FromRequest;
    use axum::http::HeaderMap;

    let mut file_bytes = Vec::new();
    let mut preset_name = None;
    let mut recipe_json = None;

    let content_type = req
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if content_type.starts_with("application/json") {
        let body_bytes = match axum::body::to_bytes(req.into_body(), usize::MAX).await {
            Ok(b) => b,
            Err(_) => {
                return (StatusCode::BAD_REQUEST, "Invalid JSON body").into_response();
            }
        };
        if let Ok(val) = serde_json::from_slice::<serde_json::Value>(&body_bytes) {
            if let Some(p) = val.get("preset").and_then(|v| v.as_str()) {
                preset_name = Some(p.to_string());
            }
            if let Some(inp) = val.get("input").and_then(|v| v.as_str())
                && let Ok(bytes) = std::fs::read(inp)
            {
                file_bytes = bytes;
            }
        }
    } else {
        let mut multipart = match axum::extract::Multipart::from_request(req, &()).await {
            Ok(m) => m,
            Err(_) => {
                return (StatusCode::BAD_REQUEST, "Invalid multipart request").into_response();
            }
        };

        while let Ok(Some(field)) = multipart.next_field().await {
            let name = field.name().unwrap_or("").to_string();
            let data = match field.bytes().await {
                Ok(d) => d,
                Err(_) => continue,
            };

            if name == "file" || name == "input" {
                file_bytes = data.to_vec();
            } else if name == "preset" {
                preset_name = Some(String::from_utf8_lossy(&data).trim().to_string());
            } else if name == "recipe" {
                recipe_json = Some(String::from_utf8_lossy(&data).trim().to_string());
            }
        }
    }

    let workflow = if let Some(p) = preset_name {
        match paperpilot_core::workflow::get_preset_by_id(&p) {
            Some(w) => w,
            None => {
                return (StatusCode::BAD_REQUEST, "Preset not found").into_response();
            }
        }
    } else if let Some(r) = recipe_json {
        match serde_json::from_str::<paperpilot_core::workflow::SavedWorkflow>(&r) {
            Ok(w) => w,
            Err(_) => return (StatusCode::BAD_REQUEST, "Invalid recipe JSON").into_response(),
        }
    } else {
        return (StatusCode::BAD_REQUEST, "Must provide preset or recipe").into_response();
    };

    if file_bytes.is_empty() {
        return (StatusCode::BAD_REQUEST, "Missing input file bytes").into_response();
    }

    let (out_bytes, report) =
        match paperpilot_pdf::operations::workflow::run_workflow_bytes(&file_bytes, &workflow) {
            Ok(res) => res,
            Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        };

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/pdf".parse().unwrap());
    headers.insert(
        "X-Workflow-Steps-Executed",
        report.steps_completed.to_string().parse().unwrap(),
    );
    headers.insert(
        "X-Workflow-Duration-Ms",
        report.duration_ms.to_string().parse().unwrap(),
    );

    (StatusCode::OK, headers, out_bytes).into_response()
}

pub async fn workspace_search_handler(Json(payload): Json<serde_json::Value>) -> impl IntoResponse {
    let input = payload
        .get("input")
        .or_else(|| payload.get("dir"))
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let query = payload.get("query").and_then(|v| v.as_str()).unwrap_or("");

    use paperpilot_core::traits::PdfOperation;
    use paperpilot_pdf::document::LopdfDocument;

    let path = std::path::PathBuf::from(input);
    let mut doc = match LopdfDocument::load(&path) {
        Ok(d) => d,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse {
                    success: false,
                    result: None,
                    error: Some(e.to_string()),
                }),
            )
                .into_response();
        }
    };

    let op = paperpilot_pdf::operations::search::SearchOperation::new(query.to_string());
    match op.execute(&mut doc) {
        Ok(_) => {
            let matches = op.match_pages.lock().unwrap().clone();
            (
                StatusCode::OK,
                Json(ApiResponse {
                    success: true,
                    result: Some(serde_json::json!({ "matches": matches })),
                    error: None,
                }),
            )
                .into_response()
        }
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse {
                success: false,
                result: None,
                error: Some(e.to_string()),
            }),
        )
            .into_response(),
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_convert_error_handling() {
        let req = Request::builder()
            .method("POST")
            .uri("/api/v1/pdf/convert")
            .header("content-type", "application/json")
            .body(Body::from(
                r#"{"input": "invalid.pdf", "output": "invalid.docx", "format": "docx"}"#,
            ))
            .unwrap();

        // This should not panic, but return a proper error (likely 500 or 400 depending on MCP)
        // Since we don't have invalid.pdf, execute_call_tool fails cleanly, and we return INTERNAL_SERVER_ERROR
        let res = super::convert(req).await.into_response();
        assert_eq!(res.status(), axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_convert_format_mapping() {
        let formats = vec!["docx", "xlsx", "pptx", "html", "markdown", "excel", "csv"];
        for format in formats {
            let body = format!(
                r#"{{"input": "invalid.pdf", "output": "invalid.{}", "format": "{}"}}"#,
                format, format
            );
            let req = Request::builder()
                .method("POST")
                .uri("/api/v1/pdf/convert")
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap();

            let res = super::convert(req).await.into_response();
            let status = res.status();
            assert_eq!(status, axum::http::StatusCode::INTERNAL_SERVER_ERROR);

            let body_bytes = axum::body::to_bytes(res.into_body(), usize::MAX)
                .await
                .unwrap();
            let body_str = String::from_utf8_lossy(&body_bytes);
            // It should NOT be "Unknown tool" because the tool was mapped correctly.
            assert!(
                !body_str.contains("Unknown tool"),
                "Format {} was not mapped correctly: {}",
                format,
                body_str
            );
        }

        // Test an unknown format
        let req = Request::builder()
            .method("POST")
            .uri("/api/v1/pdf/convert")
            .header("content-type", "application/json")
            .body(Body::from(
                r#"{"input": "invalid.pdf", "output": "invalid.unknown", "format": "unknown"}"#,
            ))
            .unwrap();
        let res = super::convert(req).await.into_response();
        let body_bytes = axum::body::to_bytes(res.into_body(), usize::MAX)
            .await
            .unwrap();
        let body_str = String::from_utf8_lossy(&body_bytes);
        assert!(body_str.contains("Unknown tool"));
    }

    #[tokio::test]
    async fn test_pdf_rotate_pages_optional() {
        let _req = McpExecRequest {
            tool: "pdf_rotate".to_string(),
            arguments: Some(json!({ "input": "test.pdf", "angle": 90, "output": "out.pdf" })),
        };
        // Just verify it doesn't fail at param mapping by passing empty req object and asserting logic
        // Actually, we'd need to mock execute_call_tool. Since execute_call_tool is static and uses global tools, we can test it directly.
    }

    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use axum::response::IntoResponse;
    use serde_json::json;

    #[tokio::test]
    async fn test_mcp_exec_handles_invalid_arguments() {
        let req = McpExecRequest {
            tool: "pdf_render".to_string(),
            arguments: Some(json!("not an object")),
        };
        let res = mcp_exec(axum::Json(req)).await;
        assert_eq!(
            res.into_response().status(),
            axum::http::StatusCode::BAD_REQUEST
        );
    }

    #[tokio::test]
    async fn test_render_page_handles_invalid_multipart() {
        let req = Request::builder()
            .method("POST")
            .uri("/api/v1/pdf/render-page")
            .body(Body::empty())
            .unwrap();

        let res = render_page(req).await;
        assert_eq!(
            res.into_response().status(),
            axum::http::StatusCode::BAD_REQUEST
        );
    }

    #[tokio::test]
    async fn test_render_page_handles_json_object() {
        let req = Request::builder()
            .method("POST")
            .uri("/api/v1/pdf/render-page")
            .header("content-type", "application/json")
            .body(Body::from(
                r#"{"input": "invalid.pdf", "page": 1, "output": "/tmp/out.png"}"#,
            ))
            .unwrap();

        let res = render_page(req).await.into_response();
        // Since execute_call_tool fails cleanly due to invalid.pdf missing, we expect INTERNAL_SERVER_ERROR,
        // but we verify that parsing didn't fail with BAD_REQUEST
        assert_eq!(res.status(), axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_render_page_handles_json_non_object() {
        let req = Request::builder()
            .method("POST")
            .uri("/api/v1/pdf/render-page")
            .header("content-type", "application/json")
            .body(Body::from(r#"["not", "an", "object"]"#))
            .unwrap();

        let res = render_page(req).await.into_response();
        assert_eq!(res.status(), axum::http::StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_workflow_presets_handler() {
        let res = workflow_presets().await.into_response();
        assert_eq!(res.status(), axum::http::StatusCode::OK);
    }

    #[tokio::test]
    async fn test_workflow_validate_handler() {
        let preset = paperpilot_core::workflow::get_preset_by_id("preset-clean-scan").unwrap();
        let preset_val = serde_json::to_value(&preset).unwrap();
        let req = WorkflowValidateRequest {
            recipe: Some(preset_val),
        };
        let res = workflow_validate(axum::Json(req)).await.into_response();
        assert_eq!(res.status(), axum::http::StatusCode::OK);
    }

    #[tokio::test]
    async fn test_workflow_run_json_invalid_preset() {
        let req = WorkflowRunJsonRequest {
            input: "in.pdf".to_string(),
            output: "out.pdf".to_string(),
            preset: Some("bad-preset".to_string()),
            recipe: None,
        };
        let res = workflow_run_json(axum::Json(req)).await.into_response();
        assert_eq!(res.status(), axum::http::StatusCode::BAD_REQUEST);
    }
}
#[derive(Debug, serde::Deserialize, utoipa::ToSchema)]
pub struct PdfValidateSignaturesJsonRequest {
    /// Input PDF document path or base64 encoded bytes
    #[schema(example = "contract.pdf")]
    pub input: String,
    /// Output JSON report path
    #[schema(example = "report.json")]
    pub output: String,
    /// Capture and report pre-sign incremental update tamper forensics / baseline integrity
    #[schema(example = false)]
    #[serde(default)]
    pub baseline: bool,
}

/// Validate Digital Signatures [Cryptographically verify digital signatures, certificate chains, and document integrity.] [Input: .pdf | Output: .json]
#[utoipa::path(
    post,
    path = "/api/v1/pdf/validate-signatures",
    request_body(content = PdfValidateSignaturesJsonRequest, description = "Validate Signatures arguments", content_type = "application/json"),
    tag = "PDF Operations",
    responses(
        (status = 200, description = "Validate Digital Signatures completed", body = ApiResponse),
        (status = 400, description = "Invalid request arguments"),
        (status = 500, description = "Execution error")
    )
)]
pub async fn pdf_validate_signatures(req: axum::extract::Request) -> impl IntoResponse {
    handle_tool(Path("pdf_validate_signatures".to_string()), req).await
}
/// Booklet Imposition [Reorder and impose pages for 2-sided folding booklet printing.] [Input: .pdf | Output: .pdf]
#[utoipa::path(
    post,
    path = "/api/v1/pdf/booklet",
    tag = "PDF Operations",
    responses(
        (status = 200, description = "Booklet Imposition completed", body = ApiResponse),
        (status = 400, description = "Invalid request arguments"),
        (status = 500, description = "Execution error")
    )
)]
pub async fn pdf_booklet(req: axum::extract::Request) -> impl IntoResponse {
    handle_tool(axum::extract::Path("pdf_booklet".to_string()), req).await
}
/// N-Up Page Imposition [Combine multiple pages per sheet in a 2, 4, 9, or 16-up grid layout.] [Input: .pdf | Output: .pdf]
#[utoipa::path(
    post,
    path = "/api/v1/pdf/nup",
    tag = "PDF Operations",
    responses(
        (status = 200, description = "N-Up Page Imposition completed", body = ApiResponse),
        (status = 400, description = "Invalid request arguments"),
        (status = 500, description = "Execution error")
    )
)]
pub async fn pdf_nup(req: axum::extract::Request) -> impl IntoResponse {
    handle_tool(Path("pdf_nup".to_string()), req).await
}
/// Scale Page Dimensions [Scale page dimensions between standard formats or percentages.] [Input: .pdf | Output: .pdf]
#[utoipa::path(
    post,
    path = "/api/v1/pdf/scale-pages",
    tag = "PDF Operations",
    responses(
        (status = 200, description = "Scale Page Dimensions completed", body = ApiResponse),
        (status = 400, description = "Invalid request arguments"),
        (status = 500, description = "Execution error")
    )
)]
pub async fn pdf_scale_pages(req: axum::extract::Request) -> impl IntoResponse {
    handle_tool(Path("pdf_scale_pages".to_string()), req).await
}
/// Remove Digital Signatures [Strip digital signature fields, cryptographic locks, and widget annotations.] [Input: .pdf | Output: .pdf]
#[utoipa::path(
    post,
    path = "/api/v1/pdf/remove-signatures",
    tag = "PDF Operations",
    responses(
        (status = 200, description = "Remove Digital Signatures completed", body = ApiResponse),
        (status = 400, description = "Invalid request arguments"),
        (status = 500, description = "Execution error")
    )
)]
pub async fn pdf_remove_signatures(req: axum::extract::Request) -> impl IntoResponse {
    handle_tool(Path("pdf_remove_signatures".to_string()), req).await
}
/// Stamp Image & Logo [Place coordinate-exact images, logos, and badges onto PDF pages.] [Input: .pdf | Output: .pdf]
#[utoipa::path(
    post,
    path = "/api/v1/pdf/stamp",
    tag = "PDF Operations",
    responses(
        (status = 200, description = "Stamp Image & Logo completed", body = ApiResponse),
        (status = 400, description = "Invalid request arguments"),
        (status = 500, description = "Execution error")
    )
)]
pub async fn pdf_stamp(req: axum::extract::Request) -> impl IntoResponse {
    handle_tool(Path("pdf_stamp".to_string()), req).await
}
/// Overlay & Underlay Stationary [Apply letterheads, watermarked stationary, and templates as background or foreground.] [Input: .pdf | Output: .pdf]
#[utoipa::path(
    post,
    path = "/api/v1/pdf/overlay-underlay",
    tag = "PDF Operations",
    responses(
        (status = 200, description = "Overlay & Underlay Stationary completed", body = ApiResponse),
        (status = 400, description = "Invalid request arguments"),
        (status = 500, description = "Execution error")
    )
)]
pub async fn pdf_overlay_underlay(req: axum::extract::Request) -> impl IntoResponse {
    handle_tool(Path("pdf_overlay_underlay".to_string()), req).await
}
/// Auto Upright Orientation [Heuristically detect text baseline and rotation angle to automatically upright pages.] [Input: .pdf | Output: .pdf]
#[utoipa::path(
    post,
    path = "/api/v1/pdf/auto-rotate",
    tag = "PDF Operations",
    responses(
        (status = 200, description = "Auto Upright Orientation completed", body = ApiResponse),
        (status = 400, description = "Invalid request arguments"),
        (status = 500, description = "Execution error")
    )
)]
pub async fn pdf_auto_rotate(req: axum::extract::Request) -> impl IntoResponse {
    handle_tool(Path("pdf_auto_rotate".to_string()), req).await
}
/// Stitch Continuous Canvas [Combine multiple pages vertically or horizontally into a continuous single-page canvas.] [Input: .pdf | Output: .pdf]
#[utoipa::path(
    post,
    path = "/api/v1/pdf/stitch-canvas",
    tag = "PDF Operations",
    responses(
        (status = 200, description = "Stitch Continuous Canvas completed", body = ApiResponse),
        (status = 400, description = "Invalid request arguments"),
        (status = 500, description = "Execution error")
    )
)]
pub async fn pdf_stitch_canvas(req: axum::extract::Request) -> impl IntoResponse {
    handle_tool(Path("pdf_stitch_canvas".to_string()), req).await
}
/// PDF Portfolio Attachments [Inspect, extract, or embed file attachments into PDF portfolios.] [Input: .pdf | Output: .json]
#[utoipa::path(
    post,
    path = "/api/v1/pdf/attachments",
    tag = "PDF Operations",
    request_body(content = PdfAttachmentsJsonRequest, description = "JSON configuration or multipart form"),
    responses(
        (status = 200, description = "PDF Portfolio Attachments completed", body = ApiResponse),
        (status = 400, description = "Invalid request arguments"),
        (status = 500, description = "Execution error")
    )
)]
pub async fn pdf_attachments(req: axum::extract::Request) -> impl IntoResponse {
    let mut args = serde_json::Map::new();

    // We get query parameters if they exist to pass action / file_name / etc.
    let uri = req.uri().clone();
    #[allow(clippy::collapsible_if)]
    if let Some(query) = uri.query() {
        if let Ok(params) = serde_urlencoded::from_str::<serde_json::Value>(query) {
            if let Some(obj) = params.as_object() {
                for (k, v) in obj {
                    args.insert(k.clone(), v.clone());
                }
            }
        }
    }

    let action_str = args
        .get("action")
        .and_then(|v| v.as_str())
        .unwrap_or("list")
        .to_string();

    let mut response = handle_tool(Path("pdf_attachments".to_string()), req)
        .await
        .into_response();

    if response.status() == StatusCode::OK {
        if action_str == "list" {
            response
                .headers_mut()
                .insert("Content-Type", "application/json".parse().unwrap());
        } else if action_str == "extract" {
            response
                .headers_mut()
                .insert("Content-Type", "application/octet-stream".parse().unwrap());
        } else {
            response
                .headers_mut()
                .insert("Content-Type", "application/pdf".parse().unwrap());
        }
    }

    response
}
/// Cryptographic Timestamp TSA [Apply RFC 3161 cryptographically verified TSA timestamp tokens.] [Input: .pdf | Output: .pdf]
#[utoipa::path(
    post,
    path = "/api/v1/pdf/timestamp",
    tag = "PDF Operations",
    responses(
        (status = 200, description = "Cryptographic Timestamp TSA completed", body = ApiResponse),
        (status = 400, description = "Invalid request arguments"),
        (status = 500, description = "Execution error")
    )
)]
pub async fn pdf_timestamp(req: axum::extract::Request) -> impl IntoResponse {
    handle_tool(Path("pdf_timestamp".to_string()), req).await
}
/// Modify PDF Permissions [Configure user and owner permissions for printing, copying, and modification.] [Input: .pdf | Output: .pdf]
#[utoipa::path(
    post,
    path = "/api/v1/pdf/change-permissions",
    tag = "PDF Operations",
    responses(
        (status = 200, description = "Modify PDF Permissions completed", body = ApiResponse),
        (status = 400, description = "Invalid request arguments"),
        (status = 500, description = "Execution error")
    )
)]
pub async fn pdf_change_permissions(req: axum::extract::Request) -> impl IntoResponse {
    handle_tool(Path("pdf_change_permissions".to_string()), req).await
}
/// Split Scanned Book Spine: Detect center binding gutter spine and split two-up page scans into single pages. [Input: .pdf | Output: .pdf]
#[utoipa::path(
    post,
    path = "/api/v1/pdf/split-scanned-book",
    tag = "PDF Operations",
    request_body(content = PdfSplitScannedBookJsonRequest, description = "JSON request containing file paths and split options", content_type = "application/json"),
    responses(
        (status = 200, description = "Split successful"),
        (status = 500, description = "Execution error")
    )
)]
pub async fn pdf_split_scanned_book(req: axum::extract::Request) -> impl IntoResponse {
    handle_tool(Path("pdf_split_scanned_book".to_string()), req).await
}
/// Comic Archive Conversion [Convert between PDF documents and comic book archive formats (CBZ/CBR).] [Input: .pdf | Output: .cbr]
#[utoipa::path(
    post,
    path = "/api/v1/pdf/comic-archive",
    tag = "PDF Operations",
    responses(
        (status = 200, description = "Comic Archive Conversion completed", body = ApiResponse),
        (status = 400, description = "Invalid request arguments"),
        (status = 500, description = "Execution error")
    )
)]
pub async fn pdf_comic_archive(req: axum::extract::Request) -> impl IntoResponse {
    handle_tool(Path("pdf_comic_archive".to_string()), req).await
}
#[derive(Deserialize, Serialize, ToSchema)]
pub struct PdfInvertColorsJsonRequest {
    #[schema(example = "tests/e2e_fixtures/real/single_page.pdf")]
    pub input: String,
    #[schema(example = "tests/e2e_fixtures/out/penta_e2e/inverted.pdf")]
    pub output: String,
    #[schema(example = "dark_mode")]
    pub mode: Option<String>,
    #[schema(example = "1")]
    pub pages: Option<String>,
}

/// Invert PDF colors or apply dark mode [Input: .pdf | Output: .pdf]
#[utoipa::path(
    post,
    path = "/api/v1/pdf/invert-colors",
    tag = "PDF Operations",
    request_body(content = PdfInvertColorsJsonRequest, description = "JSON configuration", content_type = "application/json"),
    responses(
        (status = 200, description = "Colors inverted successfully", body = ApiResponse),
        (status = 400, description = "Invalid request arguments"),
        (status = 500, description = "Execution error")
    )
)]
pub async fn pdf_invert_colors(req: axum::extract::Request) -> impl IntoResponse {
    handle_tool(Path("pdf_invert_colors".to_string()), req).await
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PdfSplitScannedBookJsonRequest {
    pub input: String,
    pub output: String,
    #[schema(example = "ltr")]
    pub reading_direction: Option<String>,
    #[schema(example = 0.0)]
    pub gutter_overlap: Option<f32>,
    #[schema(example = false)]
    pub split_horizontal: Option<bool>,
}
/// Split PDF by Outline [Split multi-chapter PDF documents based on their /Outlines bookmark hierarchy] [Input: .pdf | Output: .zip]
#[utoipa::path(
    post,
    path = "/api/v1/pdf/split-by-outline",
    tag = "PDF Operations",
    responses(
        (status = 200, description = "Split PDF by Outline completed", body = ApiResponse),
        (status = 400, description = "Invalid request arguments"),
        (status = 500, description = "Execution error")
    )
)]
pub async fn pdf_split_by_outline(req: axum::extract::Request) -> impl IntoResponse {
    handle_tool(Path("pdf_split_by_outline".to_string()), req).await
}

/// Alternating PDF Merge [Interleave pages from two PDF documents.] [Input: .pdf | Output: .pdf]
#[utoipa::path(
    post,
    path = "/api/v1/pdf/merge-alternate",
    tag = "PDF Operations",
    responses(
        (status = 200, description = "Alternating PDF Merge completed", body = ApiResponse),
        (status = 400, description = "Invalid request arguments"),
        (status = 500, description = "Execution error")
    )
)]
pub async fn pdf_merge_alternate(req: axum::extract::Request) -> impl IntoResponse {
    handle_tool(Path("pdf_merge_alternate".to_string()), req).await
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct PdfPosterizeJsonRequest {
    /// Document identifier (file path or URL)
    pub input: String,
    /// Output document path
    pub output: String,
    /// Target paper sheet size: "a4", "letter", "legal", or "custom" (default: "a4")
    pub target_sheet: Option<String>,
    /// Explicit grid columns (optional)
    pub grid_cols: Option<u32>,
    /// Explicit grid rows (optional)
    pub grid_rows: Option<u32>,
    /// Overlap margin in points between adjacent tiles (default: 14.17 pt ~= 5mm)
    pub overlap_pt: Option<f32>,
    /// Scale factor multiplier (default: 1.0)
    pub scale: Option<f32>,
    /// Include corner cut trim marks and tile coordinates (default: true)
    pub include_guides: Option<bool>,
}

/// Multi-Sheet Poster Tile Slicer [Tile oversized PDF pages across a grid of standard printable sheets with overlap margins and cut guides] [Input: .pdf | Output: .pdf]
#[utoipa::path(
    post,
    path = "/api/v1/pdf/posterize",
    tag = "PDF Operations",
    request_body(content = PdfPosterizeJsonRequest, description = "Multi-Sheet Poster Tile Slicer", content_type = "application/json"),
    responses(
        (status = 200, description = "Multi-Sheet Poster Tile Slicer completed", body = ApiResponse),
        (status = 400, description = "Invalid request arguments"),
        (status = 500, description = "Execution error")
    )
)]
pub async fn pdf_posterize(req: axum::extract::Request) -> impl IntoResponse {
    handle_tool(Path("pdf_posterize".to_string()), req).await
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct PdfDedupStreamsJsonRequest {
    // Add specific options here if needed later
}

/// Lossless Stream Deduplication [Lossless hash-based indirect object stream deduplication & CMYK-safe image compression] [Input: .pdf | Output: .pdf]
#[utoipa::path(
    post,
    path = "/api/v1/pdf/dedup-streams",
    request_body(content = PdfDedupStreamsJsonRequest, description = "Lossless hash-based indirect object stream deduplication & CMYK-safe image compression", content_type = "application/json"),
    responses(
        (status = 200, description = "Lossless Stream Deduplication completed", body = ApiResponse),
        (status = 400, description = "Invalid request arguments"),
        (status = 500, description = "Execution error")
    ),
    tag = "PDF Operations"
)]
pub async fn pdf_dedup_streams(req: axum::extract::Request) -> impl IntoResponse {
    handle_tool(Path("pdf_dedup_streams".to_string()), req).await
}
